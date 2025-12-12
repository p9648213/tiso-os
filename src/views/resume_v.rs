use sailfish::TemplateSimple;

#[derive(TemplateSimple)]
#[template(path = "resume_file.stpl")]
struct ResumeFile<'a> {
    pub id: &'a str,
    pub name: &'a str,
}

pub fn render_resume_file(
    file_id: i32,
    file_name: Option<String>,
    id_prefix: Option<String>,
) -> String {
    let file_name = file_name.clone().unwrap_or_else(|| "resume".into());
    let id = match id_prefix {
        Some(prefix) => format!("{prefix}-file-{file_id}"),
        None => format!("file-{file_id}"),
    };
    ResumeFile {
        id: &id,
        name: &file_name,
    }
    .render_once()
    .unwrap()
}

#[derive(TemplateSimple)]
#[template(path = "resume_window.stpl")]
pub struct ResumeWindow {
    pub top: i32,
    pub left: i32,
    pub window_width: i32,
    pub window_height: i32,
}

pub fn render_resume_window(parent_height: i32, parent_width: i32) -> String {
    let window_width = parent_width * 40 / 100;
    let window_height = parent_height * 75 / 100;

    let left = ((parent_width / 2) - (window_width / 2)).max(0);
    let top = ((parent_height / 2) - (window_height / 2)).max(0);

    ResumeWindow {
        top,
        left,
        window_width,
        window_height,
    }
    .render_once()
    .unwrap()
}
