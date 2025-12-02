pub struct Cd {
    pub current_path: String,
}

impl Cd {
    pub fn new(current_path: &str) -> Self {
        Cd {
            current_path: current_path.to_string(),
        }
    }
}
