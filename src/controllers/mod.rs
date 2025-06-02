use serde::Deserialize;

pub mod screen_c;

#[derive(Deserialize)]
pub struct GridForm {
    pub height: u16,
    pub width: u16,
}
