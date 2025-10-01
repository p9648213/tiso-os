use hypertext::{Raw, prelude::*};

pub fn render_music_file() -> impl Renderable {
    maud! {
        div data-file-type="music" class="flex flex-col justify-center items-center gap-1.5 hover:bg-blue-900 p-1.5 rounded-xs w-23 h-fit cursor-pointer file" {
            img class="w-9 h-9 select-none" src="/assets/images/music.svg" draggable="false";
            div class="text-white text-sm text-center select-none" {
                "Music Player"
            }
        }
    }
}

pub fn render_music_player_window(parent_height: i32, parent_width: i32) -> impl Renderable {
    let window_width = 400;

    let left = ((parent_width / 2) - (window_width / 2)).max(0);
    let top = (parent_height / 2).max(0);

    maud! {
        (Raw::dangerously_create(r#"
            <script type="module">
                import {setupMusicPlayer} from "/assets/js/music.js";
                setupMusicPlayer();
            </script>
        "#))
        div
            id="music-player-window"
            class="absolute flex flex-col bg-zinc-950 rounded-sm overflow-hidden text-white"
            style={ "top:" (top) "px; left:" (left) "px; width:" (window_width) "px; height: fit-content; transform: translateY(-50%);" }
        {
            div id="music-player-header" class="flex justify-between items-center bg-zinc-900 px-3 h-12 select-none" {
                div class="flex items-center gap-2" {
                    img class="w-6 h-6" src="/assets/images/music.svg" draggable="false";
                    "Music Player"
                }
                div class="flex items-center gap-3" {
                    img class="hover:opacity-70 w-5 h-5 cursor-pointer hide" src="/assets/images/minus.svg" draggable="false";
                    img class="hover:opacity-70 w-5 h-5 cursor-pointer close" src="/assets/images/x.svg" draggable="false";
                }
            }
            div class="flex flex-col gap-4 p-4 h-full" {
                div class="flex flex-col justify-center items-center bg-zinc-800 p-4 rounded-sm" {
                    div id="album-art" class="flex justify-center items-center bg-slate-800 mb-4 rounded-sm w-32 h-32" {
                        img class="rounded-sm w-32 h-32" src="/assets/images/default-album.svg" draggable="false";
                    }
                    div id="song-title" class="mb-1 font-medium text-lg" { "No song selected" }
                    div id="artist-name" class="text-zinc-400 text-sm" { "Unknown artist" }
                }
                div class="flex flex-col gap-2" {
                    div class="relative" {
                        div class="top-1/2 absolute bg-zinc-700 rounded-full w-full h-1 -translate-y-1/2" {}
                        div id="progress-bar" class="top-1/2 absolute bg-blue-600 rounded-full w-0 h-1 -translate-y-1/2" {}
                        input id="seek-bar" type="range" min="0" max="100" value="0" class="z-10 opacity-0 w-full cursor-pointer";
                    }
                    div class="flex justify-between text-zinc-400 text-xs" {
                        span id="current-time" { "0:00" }
                        span id="total-time" { "0:00" }
                    }
                }
                div class="flex justify-center items-center gap-4" {
                    button id="prev-btn" class="hover:opacity-70" {
                        img class="w-8 h-8" src="/assets/images/skip-back.svg" draggable="false";
                    }
                    button id="play-pause-btn" class="hover:opacity-70" {
                        img id="play-icon" class="w-10 h-10" src="/assets/images/play.svg" draggable="false";
                        img id="pause-icon" class="hidden w-10 h-10" src="/assets/images/pause.svg" draggable="false";
                    }
                    button id="next-btn" class="hover:opacity-70" {
                        img class="w-8 h-8" src="/assets/images/skip-forward.svg" draggable="false";
                    }
                  }
                div class="flex justify-between items-center"{
                    button id="shuffle-btn" class="hover:opacity-70" {
                        img class="w-6 h-6" src="/assets/images/shuffle-off.svg" draggable="false";
                    }
                    div class="flex items-center gap-2"{
                        img class="w-6 h-6" src="/assets/images/volume.svg" draggable="false";
                        input id="volume-slider" type="range" min="0" max="100" value="70" class="w-20";
                    }
                    button id="repeat-btn" class="hover:opacity-70" {
                        img class="w-6 h-6" src="/assets/images/repeat-off.svg" draggable="false";
                    }
                }
                div class="flex-1 overflow-y-auto" {
                    div id="playlist-container" class="flex flex-col gap-2" {
                        div class="py-4 text-zinc-400 text-center" { "No songs in playlist" }
                    }
                }
                div class="flex justify-center p-2 pb-0 border-zinc-700 border-t" {
                    button id="load-music-btn" class="flex items-center gap-2 bg-blue-600 hover:bg-blue-700 px-4 py-2 rounded-sm" {
                        img class="w-4 h-4" src="/assets/images/plus.svg" draggable="false";
                        "Load Music"
                    }
                    input id="music-file-input" type="file" accept="audio/*" multiple class="hidden";
                }
            }
        }
    }
}
