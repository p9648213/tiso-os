use deadpool_postgres::Pool;

pub struct Mkdir<'a> {
    pub current_dir: &'a str,
    pub args: &'a Vec<String>,
    pub pool: &'a Pool,
    pub user_id: i32,
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

    pub async fn create_folder(&self) -> String {
        // TODO: Implement
        "".to_string()
    }
}
