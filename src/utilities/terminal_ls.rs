use deadpool_postgres::Pool;

pub struct Ls<'a> {
    pub current_dir: String,
    pub pool: &'a Pool
}

impl<'a> Ls<'a> {
    pub fn new(current_dir: &str, pool: &'a Pool) -> Self {
        Ls {
            current_dir: current_dir.to_string(),
            pool,
        }
    }

    pub async fn list_file(&self) -> String {
        "".into()
    }
}
