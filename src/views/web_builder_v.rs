use std::collections::HashMap;

use sailfish::TemplateSimple;

use crate::{
    constant::web_builder::SECTIONS,
    models::web_builder_db::{DomTree, Node},
};

#[derive(PartialEq, Eq)]
pub enum ReviewMode {
    Download,
    Preview,
    None,
}

#[derive(Default)]
pub struct EditableElement {
    pub text: Option<String>,
    pub background: Option<String>,
}

#[derive(TemplateSimple)]
#[template(path = "web_builder_file.stpl")]
struct WebBuilderFile {
    pub id: Option<String>,
}

pub fn render_web_builder_file(file_id: Option<String>) -> String {
    WebBuilderFile { id: file_id }.render_once().unwrap()
}

#[derive(TemplateSimple)]
#[template(path = "web_builder_window.stpl")]
struct WebBuilderWindow<'a> {
    web_builder_id: i32,
    file_name: &'a str,
    data: &'a DomTree,
    window_width: i32,
    window_height: i32,
    top: i32,
    left: i32,
    builder_list: &'a HashMap<i32, &'a str>,
}

pub fn render_web_builder_window(
    web_builder_id: i32,
    file_name: &str,
    data: &DomTree,
    parent_height: i32,
    parent_width: i32,
    builder_list: &HashMap<i32, &str>,
) -> String {
    let window_width = parent_width * 90 / 100;
    let window_height = parent_height * 98 / 100;

    let left = ((parent_width / 2) - (window_width / 2)).max(0);
    let top = ((parent_height / 2) - (window_height / 2)).max(0);

    WebBuilderWindow {
        web_builder_id,
        file_name,
        data,
        window_width,
        window_height,
        top,
        left,
        builder_list,
    }
    .render_once()
    .unwrap()
}

#[derive(TemplateSimple)]
#[template(path = "web_builder_setting.stpl")]
struct WebBuilderSetting<'a> {
    web_builder_id: i32,
    nodes: &'a HashMap<String, Node>,
    body_node: &'a Node,
}

pub fn render_web_builder_setting(data: &DomTree, web_builder_id: i32) -> String {
    WebBuilderSetting {
        web_builder_id,
        nodes: &data.nodes,
        body_node: data.nodes.get(&data.body_node).unwrap(),
    }
    .render_once()
    .unwrap()
}

#[derive(TemplateSimple)]
#[template(path = "web_builder_web_tree.stpl")]
struct WebBuilderWebTree<'a> {
    web_builder_id: i32,
    nodes: &'a HashMap<String, Node>,
    body_node: &'a Node,
    swap_oob: &'a str,
}

pub fn render_web_builder_web_tree(data: &DomTree, swap_oob: &str, web_builder_id: i32) -> String {
    WebBuilderWebTree {
        web_builder_id,
        nodes: &data.nodes,
        body_node: data.nodes.get(&data.body_node).unwrap(),
        swap_oob,
    }
    .render_once()
    .unwrap()
}

#[derive(TemplateSimple)]
#[template(path = "web_builder_web_tree_node.stpl")]
struct WebBuilderWebTreeNode<'a> {
    node: &'a Node,
    nodes: &'a HashMap<String, Node>,
    deep: i32,
    child_id: &'a String,
}

pub fn render_web_builder_web_tree_node(
    node: &Node,
    nodes: &HashMap<String, Node>,
    child_id: String,
) -> String {
    WebBuilderWebTreeNode {
        node,
        nodes,
        deep: 0,
        child_id: &child_id,
    }
    .render_once()
    .unwrap()
}

#[derive(TemplateSimple)]
#[template(path = "web_builder_review.stpl")]
struct WebBuilderReview<'a> {
    web_builder_id: i32,
    nodes: &'a HashMap<String, Node>,
    body_node: &'a Node,
    review_mode: ReviewMode,
}

pub fn render_web_builder_review(
    data: &DomTree,
    review_mode: ReviewMode,
    web_builder_id: i32,
) -> String {
    WebBuilderReview {
        web_builder_id,
        nodes: &data.nodes,
        body_node: data.nodes.get(&data.body_node).unwrap(),
        review_mode,
    }
    .render_once()
    .unwrap()
}

#[derive(TemplateSimple)]
#[template(path = "web_builder_node.stpl")]
struct WebBuilderNode<'a> {
    node: &'a Node,
    nodes: &'a HashMap<String, Node>,
    child_id: &'a String,
}

pub fn render_web_builder_node(
    node: &Node,
    nodes: &HashMap<String, Node>,
    child_id: String,
) -> String {
    WebBuilderNode {
        node,
        nodes,
        child_id: &child_id,
    }
    .render_once()
    .unwrap()
}

#[derive(TemplateSimple)]
#[template(path = "web_builder_edit_node.stpl")]
struct WebBuilderEditNode<'a> {
    web_builder_id: i32,
    node_selected: bool,
    swap_oob: &'a str,
    editable_element: EditableElement,
}

pub fn render_web_builder_edit_node(
    editable_element: EditableElement,
    swap_oob: &str,
    node_selected: bool,
    web_builder_id: i32,
) -> String {
    WebBuilderEditNode {
        web_builder_id,
        node_selected,
        editable_element,
        swap_oob,
    }
    .render_once()
    .unwrap()
}

#[derive(TemplateSimple)]
#[template(path = "web_builder_section_dialog.stpl")]
struct WebBuilderSectionDialog {}

pub fn render_web_builder_section_dialog() -> String {
    WebBuilderSectionDialog {}.render_once().unwrap()
}

#[derive(TemplateSimple)]
#[template(path = "web_builder_select_section.stpl")]
struct WebBuilderSelectSection<'a> {
    selected_section: &'a str,
}

pub fn render_web_builder_select_section(selected_section: &str) -> String {
    WebBuilderSelectSection { selected_section }
        .render_once()
        .unwrap()
}

#[derive(TemplateSimple)]
#[template(path = "web_builder_select_header.stpl")]
struct WebBuilderSelectHeader<'a> {
    selected_header: i32,
    swap_oob: &'a str,
}

pub fn render_web_builder_select_header(selected_header: i32, swap_oob: &str) -> String {
    WebBuilderSelectHeader {
        selected_header,
        swap_oob,
    }
    .render_once()
    .unwrap()
}

#[derive(TemplateSimple)]
#[template(path = "web_builder_select_footer.stpl")]
struct WebBuilderSelectFooter<'a> {
    selected_footer: i32,
    swap_oob: &'a str,
}

pub fn render_web_builder_select_footer(selected_footer: i32, swap_oob: &str) -> String {
    WebBuilderSelectFooter {
        selected_footer,
        swap_oob,
    }
    .render_once()
    .unwrap()
}

#[derive(TemplateSimple)]
#[template(path = "web_builder_select_hero.stpl")]
struct WebBuilderSelectHero<'a> {
    selected_hero: i32,
    swap_oob: &'a str,
}

pub fn render_web_builder_select_hero(selected_hero: i32, swap_oob: &str) -> String {
    WebBuilderSelectHero {
        selected_hero,
        swap_oob,
    }
    .render_once()
    .unwrap()
}

#[derive(TemplateSimple)]
#[template(path = "web_builder_select_contact.stpl")]
struct WebBuilderSelectContact<'a> {
    selected_contact: i32,
    swap_oob: &'a str,
}

pub fn render_web_builder_select_contact(selected_contact: i32, swap_oob: &str) -> String {
    WebBuilderSelectContact {
        selected_contact,
        swap_oob,
    }
    .render_once()
    .unwrap()
}

fn render_children_nodes(node: &Node, nodes: &HashMap<String, Node>) -> String {
    let mut out = String::new();
    for child_id in &node.children {
        if let Some(child) = nodes.get(child_id) {
            out.push_str(
                &WebBuilderNode {
                    node: child,
                    nodes,
                    child_id,
                }
                .render_once()
                .unwrap(),
            );
        }
    }
    out
}

fn render_children_tree_nodes(node: &Node, nodes: &HashMap<String, Node>, deep: i32) -> String {
    let mut out = String::new();
    for child_id in &node.children {
        if let Some(child) = nodes.get(child_id) {
            out.push_str(
                &WebBuilderWebTreeNode {
                    node: child,
                    nodes,
                    deep,
                    child_id,
                }
                .render_once()
                .unwrap(),
            );
        }
    }
    out
}
