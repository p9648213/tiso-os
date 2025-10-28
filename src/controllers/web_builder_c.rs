use std::{collections::HashMap};

use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use deadpool_postgres::Pool;
use serde::Deserialize;

use crate::{
    constant::web_builder::{HEADER_TEMPLATE_1, HEADER_TEMPLATE_2, HEADER_TEMPLATE_3, HEADER_TEMPLATE_4}, middlewares::session_mw::UserId, models::{
        error::AppError,
        web_builder_db::{DomTree, Node, WebBuilder},
        web_builder_window::WebBuilderWindow,
    }, utilities::common::{html_to_nodes, parse_user_id}, views::web_builder_v::{
        render_web_builder_select_contact, render_web_builder_select_footer,
        render_web_builder_select_header, render_web_builder_select_hero,
        render_web_builder_select_section, render_web_builder_window,
    }
};

pub async fn get_web_builder_window(
    Path((file_id, height, width)): Path<(i32, i32, i32)>,
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = parse_user_id(user_id)?;

    let web_builder_window = WebBuilderWindow::get_web_builders(
        file_id,
        user_id,
        vec!["id", "data", "name"],
        vec!["id", "file_name"],
        &pool,
    )
    .await?;

    let first_builder = web_builder_window.first().unwrap();
    let data = first_builder.web_builder.data.as_ref().unwrap();

    let dom_tree: DomTree = DomTree::deserialize(data).map_err(|err| {
        tracing::error!("Could not parse dom tree: {}", err);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
    })?;

    let web_builder_id = first_builder.web_builder.id.unwrap();

    let mut builder_list: HashMap<i32, &str> = HashMap::new();

    web_builder_window.iter().for_each(|window| {
        builder_list.insert(
            window.web_builder.id.unwrap(),
            window.web_builder.name.as_ref().unwrap(),
        );
    });

    Ok((
        [(
            "HX-Trigger",
            format!(
                r#"{{"openFile":{{"image":"/assets/images/web-builder/web-builder.svg", "window_id": "web-builder-window-{}"}}}}"#,
                web_builder_id
            ),
        )],
        render_web_builder_window(
            web_builder_id,
            first_builder.file.file_name.as_ref().unwrap(),
            &dom_tree,
            height,
            width,
            &builder_list,
        ),
    ))
}

pub async fn get_web_builder(
    Path(builder_id): Path<i32>,
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = parse_user_id(user_id)?;

    let web_builder =
        WebBuilderWindow::get_web_builder(builder_id, user_id, vec!["data"], vec![], &pool).await?;

    println!("{:?}", web_builder);

    Ok(())
}

pub async fn get_node(
    Path((builder_id, node_id)): Path<(i32, String)>,
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = parse_user_id(user_id)?;

    let node = WebBuilderWindow::get_web_builder_node(builder_id, user_id, &node_id, &pool).await?;

    match node {
        Some(node) => {
            println!("{:?}", node);
            Ok(())
        }
        None => Err(AppError::new(StatusCode::NOT_FOUND, "Node not found")),
    }
}

pub async fn insert_node(
    Path((builder_id, parent_node_id)): Path<(i32, String)>,
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
    Json(payload): Json<Node>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = parse_user_id(user_id)?;

    WebBuilder::insert_node(
        builder_id,
        user_id,
        uuid::Uuid::new_v4().to_string(),
        parent_node_id,
        payload,
        &pool,
    )
    .await?;

    Ok(())
}

pub async fn edit_node(
    Path((builder_id, node_id)): Path<(i32, String)>,
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
    Json(payload): Json<Node>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = parse_user_id(user_id)?;

    WebBuilder::edit_node(builder_id, user_id, node_id, &payload, &pool).await?;

    Ok(())
}

pub async fn delete_node(
    Path((builder_id, node_id)): Path<(i32, String)>,
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = parse_user_id(user_id)?;

    WebBuilder::delete_node(builder_id, user_id, node_id, &pool).await?;

    Ok(())
}

pub async fn get_selected_section(
    Path(section_type): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    match section_type.as_str() {
        "Header" => Ok((
            [(
                "HX-Trigger",
                r#"{"changeSectionType":{"sectionType": "Header"}, "changeTemplateNumber":{"templateNumber": 1}}"#,
            )],
            format!(
                "{}{}",
                render_web_builder_select_section(&section_type),
                render_web_builder_select_header(1, "outerHTML")
            ),
        )),
        "Footer" => Ok((
            [(
                "HX-Trigger",
                r#"{"changeSectionType":{"sectionType": "Footer"}, "changeTemplateNumber":{"templateNumber": 1}}"#,
            )],
            format!(
                "{}{}",
                render_web_builder_select_section(&section_type),
                render_web_builder_select_footer(1, "outerHTML")
            ),
        )),
        "Hero Section" => Ok((
            [(
                "HX-Trigger",
                r#"{"changeSectionType":{"sectionType": "Hero Section"}, "changeTemplateNumber":{"templateNumber": 1}}"#,
            )],
            format!(
                "{}{}",
                render_web_builder_select_section(&section_type),
                render_web_builder_select_hero(1, "outerHTML")
            ),
        )),
        "Contact Form" => Ok((
            [(
                "HX-Trigger",
                r#"{"changeSectionType":{"sectionType": "Contact Form"}, "changeTemplateNumber":{"templateNumber": 1}}"#,
            )],
            format!(
                "{}{}",
                render_web_builder_select_section(&section_type),
                render_web_builder_select_contact(1, "outerHTML")
            ),
        )),
        _ => Err(AppError::new(StatusCode::NOT_FOUND, "Section not found")),
    }
}

pub async fn add_section(
    Path((builder_id, section_type, template_number)): Path<(i32, String, i32)>,
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = parse_user_id(user_id)?;

    match section_type.as_str() {
        "Header" => {
            let template_html =  match template_number {
                1 => HEADER_TEMPLATE_1,
                2 => HEADER_TEMPLATE_2,
                3 => HEADER_TEMPLATE_3,
                4 => HEADER_TEMPLATE_4,
                _ => {
                    return Err(AppError::new(StatusCode::NOT_FOUND, "Template not found"));
                }
            };

            let nodes = html_to_nodes(template_html);

            println!("{:#?}", nodes);

            Ok(())
        },
        _ => {
            Ok(())
        }
    }
}