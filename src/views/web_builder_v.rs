use std::collections::HashMap;

use sailfish::TemplateSimple;

use crate::{
    constant::web_builder::SECTIONS,
    models::web_builder_db::{DomTree, Node},
};

#[derive(TemplateSimple)]
#[template(path = "web_builder_file.stpl")]
struct WebBuilderFile {
    pub id: i32,
}

pub fn render_web_builder_file(file_id: i32) -> String {
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
#[template(path = "web_builder_structure.stpl")]
struct WebBuilderStructure {}

pub fn render_web_builder_structure() -> String {
    WebBuilderStructure {}.render_once().unwrap()
}

#[derive(TemplateSimple)]
#[template(path = "web_builder_review.stpl")]
struct WebBuilderReview<'a> {
    nodes: &'a HashMap<String, Node>,
    body_node: &'a Node,
}

pub fn render_web_builder_review(data: &DomTree) -> String {
    WebBuilderReview {
        nodes: &data.nodes,
        body_node: data.nodes.get(&data.body_node).unwrap(),
    }
    .render_once()
    .unwrap()
}

#[derive(TemplateSimple)]
#[template(path = "web_builder_node.stpl")]
struct WebBuilderNode<'a> {
    node: &'a Node,
}

pub fn render_web_builder_node(node: &Node) -> String {
    WebBuilderNode { node }.render_once().unwrap()
}

#[derive(TemplateSimple)]
#[template(path = "web_builder_setting.stpl")]
struct WebBuilderSetting {}

pub fn render_web_builder_setting() -> String {
    WebBuilderSetting {}.render_once().unwrap()
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
