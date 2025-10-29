use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "folder/folder.stpl")]
pub struct Folder<'a> {
    pub id: &'a str,
    pub name: &'a str,
}

pub fn render_folder(id: i32, name: Option<String>, id_prefix: Option<String>) -> String {
    let folder_name = name.as_deref().unwrap_or("New Folder");

    let id = match id_prefix {
        Some(prefix) => format!("{prefix}-folder-{id}"),
        None => format!("folder-{id}"),
    };

    Folder {
        id: &id,
        name: folder_name,
    }
    .render_once()
    .unwrap()
}

#[derive(TemplateOnce)]
#[template(path = "folder/folder_input.stpl")]
pub struct FolderInput<'a> {
    pub folder_id: i32,
    pub value: &'a str,
}

pub fn render_folder_input(folder_id: i32, value: &str) -> String {
    FolderInput { folder_id, value }.render_once().unwrap()
}
