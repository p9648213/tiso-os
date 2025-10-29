use sailfish::TemplateSimple;

#[derive(TemplateSimple)]
#[template(path = "thispc_file.stpl")]
pub struct ThisPcFile<'a> {
    pub id: i32,
    pub name: &'a str,
}

pub fn render_thispc_file(file_id: i32, file_name: Option<String>) -> String {
    let file_name = file_name.as_deref().unwrap_or("This PC");

    ThisPcFile {
        id: file_id,
        name: file_name,
    }
    .render_once()
    .unwrap()
}
