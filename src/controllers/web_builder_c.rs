use std::env;
use std::io::prelude::*;
use std::{collections::HashMap, process::Command};

use axum::{
    Extension, Json,
    extract::{Path, State},
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::{Html, IntoResponse},
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use tempfile::TempDir;
use tokio::fs;
use zip::{ZipWriter, write::FileOptions};

use crate::{
    constant::web_builder::{
        CONTACT_TEMPLATE_1, CONTACT_TEMPLATE_2, CONTACT_TEMPLATE_3, CONTACT_TEMPLATE_4,
        FOOTER_TEMPLATE_1, FOOTER_TEMPLATE_2, FOOTER_TEMPLATE_3, FOOTER_TEMPLATE_4,
        HEADER_TEMPLATE_1, HEADER_TEMPLATE_2, HEADER_TEMPLATE_3, HEADER_TEMPLATE_4,
        HERO_TEMPLATE_1, HERO_TEMPLATE_2, HERO_TEMPLATE_3, HERO_TEMPLATE_4,
    },
    middlewares::session_mw::UserId,
    models::{
        error::AppError,
        web_builder_db::{DomTree, Node, WebBuilder},
        web_builder_window::WebBuilderWindow,
    },
    utilities::common::{html_to_nodes, parse_user_id},
    views::web_builder_v::{
        ReviewMode, render_web_builder_review, render_web_builder_select_contact,
        render_web_builder_select_footer, render_web_builder_select_header,
        render_web_builder_select_hero, render_web_builder_select_section,
        render_web_builder_window,
    },
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

pub async fn get_selected_template(
    Path((section_type, template_index)): Path<(String, i32)>,
) -> Result<impl IntoResponse, AppError> {
    match section_type.as_str() {
        "Header" => Ok((
            [(
                "HX-Trigger",
                format!(
                    r#"{{"changeTemplateNumber":{{"templateNumber": {}}}}}"#,
                    template_index
                ),
            )],
            render_web_builder_select_header(template_index, ""),
        )),
        "Footer" => Ok((
            [(
                "HX-Trigger",
                format!(
                    r#"{{"changeTemplateNumber":{{"templateNumber": {}}}}}"#,
                    template_index
                ),
            )],
            render_web_builder_select_footer(template_index, ""),
        )),
        "Hero Section" => Ok((
            [(
                "HX-Trigger",
                format!(
                    r#"{{"changeTemplateNumber":{{"templateNumber": {}}}}}"#,
                    template_index
                ),
            )],
            render_web_builder_select_hero(template_index, ""),
        )),
        "Contact Form" => Ok((
            [(
                "HX-Trigger",
                format!(
                    r#"{{"changeTemplateNumber":{{"templateNumber": {}}}}}"#,
                    template_index
                ),
            )],
            render_web_builder_select_contact(template_index, ""),
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

    let template_html = match section_type.as_str() {
        "Header" => match template_number {
            1 => HEADER_TEMPLATE_1,
            2 => HEADER_TEMPLATE_2,
            3 => HEADER_TEMPLATE_3,
            4 => HEADER_TEMPLATE_4,
            _ => {
                return Err(AppError::new(StatusCode::NOT_FOUND, "Template not found"));
            }
        },
        "Footer" => match template_number {
            1 => FOOTER_TEMPLATE_1,
            2 => FOOTER_TEMPLATE_2,
            3 => FOOTER_TEMPLATE_3,
            4 => FOOTER_TEMPLATE_4,
            _ => {
                return Err(AppError::new(StatusCode::NOT_FOUND, "Template not found"));
            }
        },
        "Hero Section" => match template_number {
            1 => HERO_TEMPLATE_1,
            2 => HERO_TEMPLATE_2,
            3 => HERO_TEMPLATE_3,
            4 => HERO_TEMPLATE_4,
            _ => {
                return Err(AppError::new(StatusCode::NOT_FOUND, "Template not found"));
            }
        },
        "Contact Form" => match template_number {
            1 => CONTACT_TEMPLATE_1,
            2 => CONTACT_TEMPLATE_2,
            3 => CONTACT_TEMPLATE_3,
            4 => CONTACT_TEMPLATE_4,
            _ => {
                return Err(AppError::new(StatusCode::NOT_FOUND, "Template not found"));
            }
        },
        _ => {
            return Err(AppError::new(
                StatusCode::BAD_REQUEST,
                "Invalid section type",
            ));
        }
    };

    let (nodes, root_node_ids) = html_to_nodes(template_html);

    let web_builder =
        WebBuilder::insert_nodes_to_body(builder_id, user_id, nodes, root_node_ids, &pool).await?;

    let dom_tree = DomTree::deserialize(web_builder.data.unwrap()).map_err(|err| {
        tracing::error!("Could not parse dom tree: {}", err);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
    })?;

    Ok(render_web_builder_review(&dom_tree, ReviewMode::None))
}

pub async fn get_web_builder_review(
    Path(builder_id): Path<i32>,
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = parse_user_id(user_id)?;

    let web_builder_window =
        WebBuilderWindow::get_web_builder(builder_id, user_id, vec!["data"], vec![], &pool).await?;

    let dom_tree: DomTree = DomTree::deserialize(web_builder_window.web_builder.data.unwrap())
        .map_err(|err| {
            tracing::error!("Could not parse dom tree: {}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })?;

    Ok(Html(render_web_builder_review(
        &dom_tree,
        ReviewMode::Preview,
    )))
}

pub async fn download_website(
    Path(builder_id): Path<i32>,
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = parse_user_id(user_id)?;

    let web_builder_window =
        WebBuilderWindow::get_web_builder(builder_id, user_id, vec!["data"], vec![], &pool).await?;

    let dom_tree: DomTree = DomTree::deserialize(web_builder_window.web_builder.data.unwrap())
        .map_err(|err| {
            tracing::error!("Could not parse dom tree: {}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })?;

    let html = render_web_builder_review(&dom_tree, ReviewMode::Download);

    let temp_dir = TempDir::new().map_err(|err| {
        tracing::error!("Could not create temp directory: {}", err);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
    })?;

    let temp_path = temp_dir.path();
    let html_path = temp_path.join("index.html");
    let css_path = temp_path.join("styles.css");

    fs::write(&html_path, &html).await.map_err(|err| {
        tracing::error!("Could not write HTML file: {}", err);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
    })?;

    let project_dir = env::current_dir().map_err(|err| {
        tracing::error!("Could not get current directory: {}", err);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
    })?;

    let mut child = Command::new("npx")
        .current_dir(&project_dir)
        .args([
            "@tailwindcss/cli",
            "--output",
            css_path.to_str().unwrap(),
            "--content",
            html_path.to_str().unwrap(),
        ])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|err| {
            tracing::error!("Could not spawn Tailwind CLI: {}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })?;

    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write as StdWrite;
        stdin
            .write_all(b"@import \"tailwindcss\";\n")
            .map_err(|err| {
                tracing::error!("Could not write to Tailwind CLI stdin: {}", err);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
            })?;
    }

    let output = child.wait_with_output().map_err(|err| {
        tracing::error!("Could not wait for Tailwind CLI: {}", err);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
    })?;

    if !output.status.success() {
        tracing::error!(
            "Tailwind CLI failed: stderr={}, stdout={}",
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout)
        );
        return Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "CSS generation failed",
        ));
    }

    let css = fs::read_to_string(&css_path).await.map_err(|err| {
        tracing::error!("Could not read generated CSS: {}", err);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
    })?;

    let mut zip_buffer = Vec::new();
    {
        let mut zip = ZipWriter::new(std::io::Cursor::new(&mut zip_buffer));

        let options: FileOptions<()> = FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o755);

        zip.start_file("index.html", options).map_err(|err| {
            tracing::error!("Could not create HTML file in ZIP: {}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })?;

        zip.write_all(html.as_bytes()).map_err(|err| {
            tracing::error!("Could not write HTML to ZIP: {}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })?;

        zip.start_file("styles.css", options).map_err(|err| {
            tracing::error!("Could not create CSS file in ZIP: {}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })?;

        zip.write_all(css.as_bytes()).map_err(|err| {
            tracing::error!("Could not write CSS to ZIP: {}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })?;

        zip.finish().map_err(|err| {
            tracing::error!("Could not finalize ZIP: {}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })?;
    }

    let mut headers = HeaderMap::new();

    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/zip"),
    );

    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!(
            "attachment; filename=\"website-{}.zip\"",
            builder_id
        ))
        .map_err(|err| {
            tracing::error!("Invalid header value: {}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })?,
    );

    Ok((headers, zip_buffer))
}
