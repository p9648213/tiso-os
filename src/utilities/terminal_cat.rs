use deadpool_postgres::Pool;

use crate::models::{
    file_db::{File, FileType},
    txt_window::TxtWindow,
};

pub struct Cat<'a> {
    pub current_dir: String,
    pub args: &'a Vec<String>,
    pub user_id: i32,
    pub pool: &'a Pool,
}

impl<'a> Cat<'a> {
    pub fn new(current_dir: &str, args: &'a Vec<String>, user_id: i32, pool: &'a Pool) -> Self {
        Cat {
            current_dir: current_dir.to_string(),
            args,
            user_id,
            pool,
        }
    }

    pub async fn cat(&self) -> Result<String, String> {
        let file_name = if self.args.len() > 1 {
            self.args.join(" ")
        } else {
            self.args[0].trim().to_string()
        };

        let file_path = if self.current_dir == "/" {
            format!("/{}", file_name)
        } else {
            format!("{}/{}", self.current_dir, file_name)
        };

        let file =
            File::get_file_by_path(&file_path, self.user_id, vec!["id", "file_type"], self.pool)
                .await
                .map_err(|err| err.to_string())?;

        match file.file_type.unwrap() {
            FileType::Txt => {
                let txt_window = TxtWindow::get_txt_window(
                    file.id.unwrap(),
                    self.user_id,
                    vec!["text"],
                    vec![],
                    self.pool,
                )
                .await
                .map_err(|err| err.to_string())?;

                let text = txt_window.txt.text.unwrap();

                if text.is_empty() {
                    Err("Document is empty".to_string())
                } else {
                    Ok(text)
                }
            }
            _ => Err("Can not view this file".to_string()),
        }
    }
}
