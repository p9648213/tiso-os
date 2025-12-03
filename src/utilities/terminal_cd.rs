pub struct Cd {
    pub current_dir: String,
}

impl Cd {
    pub fn new(current_dir: &str) -> Self {
        Cd {
            current_dir: current_dir.to_string(),
        }
    }
}
