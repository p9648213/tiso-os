use deadpool_postgres::Pool;

pub struct Rm<'a> {
    pub current_dir: String,
    pub args: &'a Vec<String>,
    pub user_id: i32,
    pub pool: &'a Pool,
}

impl<'a> Rm<'a> {
    pub fn new(current_dir: &str, args: &'a Vec<String>, user_id: i32, pool: &'a Pool) -> Self {
        Rm {
            current_dir: current_dir.to_string(),
            args,
            user_id,
            pool,
        }
    }

    pub async fn remove_item(&self) -> Result<String, String> {
        let item_name = if self.args.len() > 1 {
            self.args.join(" ")
        } else {
            self.args[0].trim().to_string()
        }; 

        Ok(format!("{} removed", item_name))
    }
}
