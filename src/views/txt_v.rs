use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "txt/txt_file.stpl")]
struct TxtFile<'a> {
    pub id: &'a str,
    pub name: &'a str,
}

pub fn render_txt_file(
    file_id: i32,
    file_name: Option<String>,
    id_prefix: Option<String>,
) -> String {
    let file_name = file_name.clone().unwrap_or_else(|| "New Text".into());
    let id = match id_prefix {
        Some(prefix) => format!("{prefix}-file-{file_id}"),
        None => format!("file-{file_id}"),
    };
    TxtFile {
        id: &id,
        name: &file_name,
    }
    .render_once()
    .unwrap()
}

#[derive(TemplateOnce)]
#[template(path = "txt/txt_input.stpl")]
struct TxtInput<'a> {
    pub file_id: i32,
    pub value: &'a str,
}

pub fn render_txt_input(file_id: i32, value: &str) -> String {
    TxtInput { file_id, value }.render_once().unwrap()
}

#[derive(TemplateOnce)]
#[template(path = "txt/txt_window.stpl")]
pub struct TxtWindow<'a> {
    pub file_name: &'a str,
    pub txt_id: i32,
    pub top: i32,
    pub left: i32,
    pub window_width: i32,
    pub window_height: i32,
}

pub fn render_txt_window(
    file_name: &str,
    txt_id: i32,
    parent_height: i32,
    parent_width: i32,
) -> String {
    let window_width = parent_width * 40 / 100;
    let window_height = parent_height * 75 / 100;

    let left = ((parent_width / 2) - (window_width / 2)).max(0);
    let top = ((parent_height / 2) - (window_height / 2)).max(0);

    TxtWindow {
        file_name,
        txt_id,
        top,
        left,
        window_width,
        window_height,
    }
    .render_once()
    .unwrap()
}
