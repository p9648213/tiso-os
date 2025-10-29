use sailfish::TemplateSimple;

#[derive(TemplateSimple)]
#[template(path = "music_file.stpl")]
pub struct MusicFile;

pub fn render_music_file() -> String {
    MusicFile {}.render_once().unwrap()
}

#[derive(TemplateSimple)]
#[template(path = "music_player_window.stpl")]
pub struct MusicPlayerWindow {
    pub left: i32,
    pub window_width: i32,
}

pub fn render_music_player_window(_parent_height: i32, parent_width: i32) -> String {
    let window_width = 400;
    let left = ((parent_width / 2) - (window_width / 2)).max(0);

    MusicPlayerWindow { left, window_width }.render_once().unwrap()
}
