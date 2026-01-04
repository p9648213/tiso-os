use deadpool_postgres::Pool;

pub struct Cp<'a> {
    pub args: &'a Vec<String>,
    pub user_id: i32,
    pub pool: &'a Pool,
}

#[derive(Default, Debug)]
struct CpArgsParse {
    pub from_path: String,
    pub to_path: String,
}

enum CpArgs {
    From,
    To
}


impl<'a> Cp<'a> {
    pub fn new(args: &'a Vec<String>, user_id: i32, pool: &'a Pool) -> Self {
        Cp {
            args,
            user_id,
            pool,
        }
    }

     fn parse_args(&self) -> Result<CpArgsParse, String> {
        let mut cp_args = CpArgsParse::default();
        let mut args_index: Option<usize> = None;
        let mut current_args: Option<CpArgs> = None;

        for (index, arg) in self.args.iter().enumerate() {
            if let Some(ai) = args_index
                && index == ai
                    && let Some(ca) = current_args {
                        match ca {
                            CpArgs::From => {
                                cp_args.from_path = arg.to_string();
                                args_index = None;
                                current_args = None;
                            }
                            CpArgs::To => {
                                cp_args.to_path = arg.to_string();
                                args_index = None;
                                current_args = None;
                            }
                        }
                    }

            if arg.starts_with("-") {
                match arg.as_str() {
                    "-from" => {
                        args_index = Some(index + 1);
                        current_args = Some(CpArgs::From);
                    },
                    "-to" => {
                        args_index = Some(index + 1);
                        current_args = Some(CpArgs::To);
                    },
                    _ => return Err(format!("Unknown argument: {}", arg)),
                }
            }
        }

        Ok(cp_args)
    }

    pub async fn copy(&self) -> Result<String, String> {
        let cp_args_parse = self.parse_args()?;

        // TODO implement copy

        println!("{:?}", cp_args_parse);

        Ok("Copied".to_string())
    }
}
