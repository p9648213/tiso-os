use std::collections::HashMap;

use axum::http::StatusCode;
use ego_tree::NodeRef;
use scraper::{Html, Node as ScrapperNode};

use crate::{
    middlewares::session_mw::UserId,
    models::{error::AppError, web_builder_db::Node},
};

pub fn parse_position(pos: &str) -> Option<(u16, u16)> {
    let parts: Vec<&str> = pos.strip_prefix("item-")?.split('-').collect();
    if parts.len() == 2 {
        let row = parts[0].parse().ok()?;
        let col = parts[1].parse().ok()?;
        Some((row, col))
    } else {
        None
    }
}

pub fn parse_user_id(user_id: UserId) -> Result<i32, AppError> {
    user_id
        .0
        .ok_or_else(|| AppError::new(StatusCode::UNAUTHORIZED, "UNAUTHORIZED"))?
        .parse::<i32>()
        .map_err(|err| {
            tracing::error!("Couldn't parse user_id: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })
}

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

            // collect attributes
            let mut attrs = HashMap::new();
            for (name, value) in &element.attrs {
                attrs.insert(name.local.to_string(), value.to_string());
            }

            // process children
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

pub fn html_to_nodes(html: &str) -> HashMap<String, Node> {
    let document = Html::parse_fragment(html);
    let mut nodes = HashMap::new();

    for child in document.tree.root().first_child().unwrap().children() {
        traverse_node(child, &mut nodes);
    }

    nodes
}
