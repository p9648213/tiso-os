use std::collections::HashMap;

use axum::http::StatusCode;
use scraper::{Html, Selector};

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

pub fn html_to_nodes(html: &str) -> HashMap<String, Node> {
    let document = Html::parse_document(html);
    let body_selector = Selector::parse("body").unwrap();
    let first_element = document
        .select(&body_selector)
        .next()
        .unwrap()
        .first_child()
        .unwrap();

    println!("{:#?}", first_element);

    todo!()
}
