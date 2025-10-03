use hypertext::{Raw, prelude::*};

use crate::{
    models::{
        file_db::FileType,
        folder_item::{FolderItem, ItemType},
    },
    views::{folder_v::render_folder, txt_v::render_txt_file},
};

pub fn render_explorer_window(
    folder_id: i32,
    folder_name: String,
    parent_width: i32,
    parent_height: i32,
    folder_items: &Vec<FolderItem>,
) -> impl Renderable {
    let window_width = 1320;
    let window_height = 800;

    let left = ((parent_width / 2) - (window_width / 2)).max(0);
    let top = ((parent_height / 2) - (window_height / 2)).max(0);

    maud! {
        (Raw::dangerously_create(format!(
            r#"
                <script type="module">
                    import {{setupExplorerWindow, setupExplorerToolBar}} from "/assets/js/explorer.js";
                    setupExplorerWindow({});
                    setupExplorerToolBar({});
                </script>
            "#,
            folder_id,
            folder_id
        )))
        div
            id={ "explorer-window-" (folder_id) }
            class="absolute flex flex-col bg-zinc-950 rounded-sm overflow-hidden text-white"
            style={ "top:" (top) "px; left:" (left) "px; width:" (window_width) "px; height:" (window_height) "px;" }
        {
            div id={ "explorer-toolbar" } class="flex justify-between bg-zinc-900 px-3 h-12 select-none" {
                div class="flex gap-2" {
                    div class="flex gap-4 bg-zinc-800 mt-3 px-4 rounded-t-sm" {
                        div class="flex items-center gap-2" {
                            img class="w-5 h-5" src="/assets/images/folder.svg" draggable="false";
                            (folder_name)
                        }
                        img class="hover:opacity-70 m-auto w-5 h-5 cursor-pointer" src="/assets/images/x.svg" draggable="false";

                    }
                    div class="mt-4" {
                        img class="hover:opacity-70 w-5 h-5 cursor-pointer" src="/assets/images/plus.svg" draggable="false";
                    }
                }
                div class="flex items-center gap-3 h-full" {
                    img class="hover:opacity-70 w-5 h-5 cursor-pointer hide" src="/assets/images/minus.svg" draggable="false";
                    img class="hover:opacity-70 w-4 h-4 cursor-pointer maximize" src="/assets/images/square.svg" draggable="false";
                    img class="hover:opacity-70 w-5 h-5 cursor-pointer close" src="/assets/images/x.svg" draggable="false";
                }
            }
            div class="flex justify-between items-center gap-6 bg-zinc-800 px-7 h-12 select-none" {
                div class="flex gap-6 shrink-0" {
                    img class="hover:opacity-70 w-4 h-4 cursor-pointer" src="/assets/images/arrow-left.svg" draggable="false";
                    img class="hover:opacity-70 w-4 h-4 cursor-pointer" src="/assets/images/arrow-right.svg" draggable="false";
                    img class="hover:opacity-70 w-4 h-4 cursor-pointer" src="/assets/images/arrow-up.svg" draggable="false";
                    img class="hover:opacity-70 w-4 h-4 cursor-pointer" src="/assets/images/rotate-ccw.svg" draggable="false";
                }
                div class="flex items-center gap-4 w-full" {
                    div class="flex flex-2 items-center gap-3 bg-zinc-700 px-4 py-1 rounded-sm h-full" {
                        "This PC"
                        img class="w-5 h-5" src="/assets/images/chevron-right.svg" draggable="false";
                    }
                    div class="relative flex-1" {
                        input class="bg-zinc-700 px-3 rounded-sm w-full h-8 text-white text-sm" type="text" placeholder="Search This PC";
                        img src="/assets/images/search.svg" alt="search" class="top-1.5 right-3 absolute w-5 h-5" draggable="false";
                    }
                }
            }
            div class="h-full" {
                div class="flex items-center gap-8 px-7 border-zinc-700 border-b w-full h-12" {
                    div class="flex items-center gap-2 hover:opacity-70 h-full cursor-pointer" {
                        img src="/assets/images/plus.svg" class="w-5 h-5" draggable="false";
                        "New"
                        img src="/assets/images/chevron-down.svg" class="w-5 h-5" draggable="false";
                    }
                    div class="flex items-center gap-8 pl-8 border-zinc-700 border-l h-full shrink-0" {
                        img src="/assets/images/scissors.svg" class="hover:opacity-70 w-5 h-5 cursor-pointer" draggable="false";
                        img src="/assets/images/copy.svg" class="hover:opacity-70 w-5 h-5 cursor-pointer" draggable="false";
                        img src="/assets/images/clipboard-paste.svg" class="hover:opacity-70 w-5 h-5 cursor-pointer" draggable="false";
                        img src="/assets/images/trash-2.svg" class="hover:opacity-70 w-5 h-5 cursor-pointer" draggable="false";
                    }
                    div class="flex justify-between gap-4 w-full h-full" {
                        div class="flex items-center gap-8 pl-8 border-zinc-700 border-l h-full" {
                            div class="flex items-center gap-2 hover:opacity-70 h-full cursor-pointer" {
                                img src="/assets/images/arrow-up-down.svg" class="w-5 h-5" draggable="false";
                                "Sort"
                                img src="/assets/images/chevron-down.svg" class="w-5 h-5" draggable="false";
                            }
                            div class="flex items-center gap-2 hover:opacity-70 h-full cursor-pointer" {
                                img src="/assets/images/table-of-contents.svg" class="w-5 h-5" draggable="false";
                                "View"
                                img src="/assets/images/chevron-down.svg" class="w-5 h-5" draggable="false";
                            }
                        }
                        div class="flex items-center gap-2 hover:opacity-70 h-full cursor-pointer" {
                            img src="/assets/images/columns-2.svg" class="w-5 h-5" draggable="false";
                            "Preview"
                        }
                    }
                }
                div class="flex gap-2 h-full" {
                    div class="flex flex-col flex-1 gap-4 px-3 py-4" {
                        div class="pb-4 border-zinc-700 border-b" {
                            div class="flex items-center gap-2 hover:opacity-70 px-2 cursor-pointer" {
                                img src="/assets/images/thispc.svg" class="w-5 h-5" draggable="false";
                                "This PC"
                                img src="/assets/images/chevron-right.svg" class="w-6 h-6" draggable="false";
                            }
                        }
                        div id="explorer-sidebar" class="flex flex-col gap-5" {
                            div id="sidebar-Desktop" class="flex items-center gap-2 hover:opacity-70 px-2 cursor-pointer" {
                                img src="/assets/images/rust-1.svg" class="w-6 h-6" draggable="false";
                                "Desktop"
                            }
                            div id="sidebar-Documents" class="flex items-center gap-2 hover:opacity-70 px-2 cursor-pointer" {
                                img src="/assets/images/google_docs-1.svg" class="w-6 h-6" draggable="false";
                                "Documents"
                            }
                            div id="sidebar-Downloads" class="flex items-center gap-2 hover:opacity-70 px-2 cursor-pointer" {
                                img src="/assets/images/video_download.svg" class="w-6 h-6" draggable="false";
                                "Downloads"
                            }
                            div id="sidebar-Pictures" class="flex items-center gap-2 hover:opacity-70 px-2 cursor-pointer" {
                                img src="/assets/images/pictures.svg" class="w-6 h-6" draggable="false";
                                "Pictures"
                            }
                            div id="sidebar-Music" class="flex items-center gap-2 hover:opacity-70 px-2 cursor-pointer" {
                                img src="/assets/images/apple_music-1.svg" class="w-6 h-6" draggable="false";
                                "Music"
                            }
                            div id="sidebar-Movies" class="flex items-center gap-2 hover:opacity-70 px-2 cursor-pointer" {
                                img src="/assets/images/movie-1.svg" class="w-6 h-6" draggable="false";
                                "Movies"
                            }
                        }
                    }
                    div class="flex flex-5 gap-4 p-4 border-zinc-700 border-l" {
                        @for item in folder_items {
                            @match item.item_type.as_ref().unwrap() {
                                ItemType::File => {
                                    @match item.file_type.as_ref().unwrap() {
                                        FileType::Txt => {
                                            div class="relative w-22 h-20" {
                                                (render_txt_file(item.id.unwrap(), &item.name, &Some("explorer".to_string())))
                                            }
                                        },
                                        _ => {}
                                    }
                                }
                                ItemType::Folder => {
                                    div class="relative w-22 h-20" {
                                        (render_folder(item.id.unwrap(), &item.name, &Some("explorer".to_string())))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
