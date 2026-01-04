use deadpool_postgres::Pool;

pub struct Mv<'a> {
    pub args: &'a Vec<String>,
    pub user_id: i32,
    pub pool: &'a Pool,
}

#[derive(Default, Debug)]
struct MvArgsParse {
    pub from_path: String,
    pub to_path: String,
}

enum MvArgs {
    From,
    To
}


impl<'a> Mv<'a> {
    pub fn new(args: &'a Vec<String>, user_id: i32, pool: &'a Pool) -> Self {
        Mv {
            args,
            user_id,
            pool,
        }
    }

     fn parse_args(&self) -> Result<MvArgsParse, String> {
        let mut mv_args = MvArgsParse::default();
        let mut args_index: Option<usize> = None;
        let mut current_args: Option<MvArgs> = None;

        for (index, arg) in self.args.iter().enumerate() {
            if let Some(ai) = args_index
                && index == ai
                    && let Some(ca) = current_args {
                        match ca {
                            MvArgs::From => {
                                mv_args.from_path = arg.to_string();
                                args_index = None;
                                current_args = None;
                            }
                            MvArgs::To => {
                                mv_args.to_path = arg.to_string();
                                args_index = None;
                                current_args = None;
                            }
                        }
                    }

            if arg.starts_with("-") {
                match arg.as_str() {
                    "-from" => {
                        args_index = Some(index + 1);
                        current_args = Some(MvArgs::From);
                    },
                    "-to" => {
                        args_index = Some(index + 1);
                        current_args = Some(MvArgs::To);
                    },
                    _ => return Err(format!("Unknown argument: {}", arg)),
                }
            }
        }

        Ok(mv_args)
    }

    pub async fn cut(&self) -> Result<String, String> {
        let mv_args_parse = self.parse_args()?;

        // TODO implement cut

        println!("{:?}", mv_args_parse);

        Ok("Copied".to_string())
    }
}
