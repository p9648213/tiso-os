use std::collections::HashMap;

use axum::http::StatusCode;
use csscolorparser::Color;
use ego_tree::NodeRef;
use regex::Regex;
use scraper::{Html, Node as ScrapperNode};

use crate::models::{error::AppError, web_builder_db::Node};

pub fn collect_descendants(
    node_id: &str,
    nodes: &serde_json::Map<String, serde_json::Value>,
    acc: &mut std::collections::HashSet<String>,
) {
    acc.insert(node_id.to_string());
    if let Some(node) = nodes.get(node_id)
        && let Some(children) = node.get("children").and_then(|c| c.as_array())
    {
        for child in children {
            if let Some(child_id) = child.as_str() {
                collect_descendants(child_id, nodes, acc);
            }
        }
    }
}

fn traverse_node(
    scrapper_node: NodeRef<'_, ScrapperNode>,
    nodes: &mut HashMap<String, Node>,
) -> Option<String> {
    match scrapper_node.value() {
        ScrapperNode::Element(element) => {
            let id = uuid::Uuid::new_v4().to_string();

            let mut attrs = HashMap::new();
            for (name, value) in &element.attrs {
                attrs.insert(name.local.to_string(), value.to_string());
            }

            let mut child_ids = vec![];
            let mut text: Option<String> = None;

            for child in scrapper_node.children() {
                match child.value() {
                    ScrapperNode::Text(t) => {
                        let trimmed = t.trim();
                        if !trimmed.is_empty() {
                            text = Some(trimmed.to_string());
                        }
                    }
                    ScrapperNode::Element(_) => {
                        if let Some(cid) = traverse_node(child, nodes) {
                            child_ids.push(cid);
                        }
                    }
                    _ => {}
                }
            }

            nodes.insert(
                id.clone(),
                Node {
                    tag: element.name.local.to_string(),
                    attributes: serde_json::to_value(attrs).unwrap(),
                    text,
                    children: child_ids,
                },
            );

            Some(id)
        }
        _ => None,
    }
}

pub fn html_to_nodes(html: &str) -> (HashMap<String, Node>, Vec<String>) {
    let document = Html::parse_fragment(html);
    let mut nodes = HashMap::new();
    let mut root_ids = vec![];

    for child in document.tree.root().first_child().unwrap().children() {
        root_ids.push(traverse_node(child, &mut nodes));
    }

    (
        nodes,
        root_ids
            .into_iter()
            .filter_map(|mut id| id.take())
            .collect(),
    )
}

pub fn extract_bg_class(input: &str) -> Option<String> {
    let regex = Regex::new(r"bg-\[[^\]]+\]|bg-[^\s]+").unwrap();
    regex.find(input).map(|m| m.as_str().to_string())
}

pub fn extract_hex_background_color(input: &str) -> Result<Option<String>, AppError> {
    if let Some(bg_class) = extract_bg_class(input) {
        if bg_class.chars().nth(3).unwrap_or_default() == '['
            && bg_class.chars().nth(4).unwrap_or_default() == '#'
        {
            return Ok(Some(bg_class[4..11].to_string()));
        }

        let css_file = include_str!("../../assets/css/lib/tailwind.css");
        let css_var = format!("--color-{}", &bg_class[3..]);

        let pattern = format!(r"{}:\s*([^;]+);", regex::escape(&css_var));
        let regex = Regex::new(&pattern).unwrap();

        if let Some(okch_color) = regex
            .captures(css_file)
            .and_then(|cap| Some(cap[1].trim().to_string()))
        {
            let okch_color: Color = okch_color.parse().map_err(|err| {
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &format!("Could not parse oklch color: {}", err),
                )
            })?;

            Ok(Some(okch_color.to_css_hex()))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

pub fn oklch_to_hex(l: f32, c: f32, h: f32) -> String {
    use std::f32::consts::PI;

    let hr = h * PI / 180.0;

    let a = c * hr.cos();
    let b = c * hr.sin();

    let l_ = l / 100.0;

    let l1 = l_;
    let m1 = l_ + 0.3963377774 * a + 0.2158037573 * b;
    let s1 = l_ - 0.1055613458 * a - 0.0638541728 * b;

    let mut r = 4.0767416621 * l1 - 3.3077115913 * m1 + 0.2309699292 * s1;
    let mut g = -1.2684380046 * l1 + 2.6097574011 * m1 - 0.3413193965 * s1;
    let mut b_ = -0.0041960863 * l1 - 0.7034186147 * m1 + 1.7076147010 * s1;

    fn srgb(x: f32) -> f32 {
        if x <= 0.0031308 {
            12.92 * x
        } else {
            1.055 * x.powf(1.0 / 2.4) - 0.055
        }
    }

    r = srgb(r).clamp(0.0, 1.0);
    g = srgb(g).clamp(0.0, 1.0);
    b_ = srgb(b_).clamp(0.0, 1.0);

    fn to_hex(v: f32) -> String {
        format!("{:02X}", (v * 255.0).round() as u8)
    }

    format!("#{}{}{}", to_hex(r), to_hex(g), to_hex(b_))
}
