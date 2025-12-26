use crate::models::folder_db::{Folder, FolderType};
use deadpool_postgres::Pool;

pub struct Mkdir<'a> {
    pub current_dir: &'a str,
    pub args: &'a Vec<String>,
    pub pool: &'a Pool,
    pub user_id: i32,
}

#[derive(Default, Debug)]
struct MkdirArgsParse {
    pub name: String,
    pub desktop_position: String,
}

#[derive(Clone, Copy)]
enum MkdirArgs {
    Dp,
}

impl<'a> Mkdir<'a> {
    pub fn new(current_dir: &'a str, args: &'a Vec<String>, user_id: i32, pool: &'a Pool) -> Self {
        Mkdir {
            current_dir,
            args,
            pool,
            user_id,
        }
    }

    fn parse_args(&self) -> Result<MkdirArgsParse, String> {
        let mut mkdir_args = MkdirArgsParse::default();

        let mut name_end = false;
        let mut args_index: Option<usize> = None;
        let mut current_args: Option<MkdirArgs> = None;

        for (index, arg) in self.args.iter().enumerate() {
            if let Some(ai) = args_index {
                if index == ai {
                    if let Some(ca) = current_args {
                        match ca {
                            MkdirArgs::Dp => {
                                mkdir_args.desktop_position = arg.to_string();
                                args_index = None;
                                current_args = None;
                            }
                        }
                    }
                }
            }

            if arg.starts_with("-") {
                match arg.as_str() {
                    "-dp" => {
                        args_index = Some(index + 1);
                        current_args = Some(MkdirArgs::Dp);
                        name_end = true;
                    }
                    _ => return Err(format!("Unknown argument: {}", arg)),
                }
            } else {
                if name_end == false {
                    mkdir_args.name = if mkdir_args.name.is_empty() {
                        mkdir_args.name + arg
                    } else {
                        mkdir_args.name + " " + arg
                    };
                }
            }
        }

        Ok(mkdir_args)
    }

    pub async fn create_folder(&self) -> Result<String, String> {
        let mkdir_args_parse = self.parse_args()?;

        if mkdir_args_parse.name.is_empty() {
            return Err("Folder name is empty".to_string());
        }

        if self.current_dir == "/" {
            return Err(
                r#"Cannot create folder in path "/". Use "cd" to change directory."#.to_string(),
            );
        }

        let current_folder = Folder::get_folder_by_path(
            self.current_dir,
            self.user_id,
            vec!["id", "folder_type"],
            self.pool,
        )
        .await
        .map_err(|err| err.to_string())?;

        let desktop_position = if current_folder.folder_type.unwrap() == FolderType::Desktop {
            Some(mkdir_args_parse.desktop_position)
        } else {
            None
        };

        let folder = Folder::create_folder(
            self.user_id,
            mkdir_args_parse.name,
            FolderType::Normal,
            Some(current_folder.id.unwrap()),
            desktop_position,
            self.current_dir.to_string(),
            self.pool,
        )
        .await
        .map_err(|err| err.to_string())?;

        Ok(format!("{} created", folder.folder_name.unwrap()))
    }
}
