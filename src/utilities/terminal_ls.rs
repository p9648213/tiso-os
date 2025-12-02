use deadpool_postgres::Pool;

pub struct Ls<'a> {
    pub current_path: String,
    pub pool: &'a Pool
}

impl<'a> Ls<'a> {
    pub fn new(current_path: &str, pool: &'a Pool) -> Self {
        Ls {
            current_path: current_path.to_string(),
            pool,
        }
    }

    pub async fn list_file(&self) -> String {
        "".into()
    }
}
