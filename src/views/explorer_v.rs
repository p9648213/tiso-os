use hypertext::prelude::*;

pub fn render_explorer(
    folder_id: i32,
    folder_name: String,
    parent_width: i32,
    parent_height: i32,
) -> impl Renderable {
    let window_width = 1320;
    let window_height = 800;

    let left = ((parent_width / 2) - (window_width / 2)).max(0);
    let top = ((parent_height / 2) - (window_height / 2)).max(0);

    maud! {
        div
            id={ "explorer-window-" (folder_id) }
            class="absolute flex flex-col bg-zinc-950 rounded-sm overflow-hidden text-white"
            style={ "top:" (top) "px; left:" (left) "px; width:" (window_width) "px; height:" (window_height) "px;" }
        {
            div id={ "explorer-toolbar-" (folder_id) } class="flex justify-between bg-zinc-900 px-3 h-12 select-none" {
                div class="flex gap-2" {
                    div class="flex gap-4 bg-zinc-800 mt-3 px-4 rounded-t-sm" {
                        div class="flex items-center gap-2" {
                            img class="w-5 h-5" src="/assets/images/folder.svg" draggable="false";
                            (folder_name)
                        }
                        img class="h-5 w-5 cursor-pointer m-auto hover:opacity-70" src="/assets/images/x.svg" draggable="false";

                    }
                    div class="mt-4" {
                        img class="h-5 w-5 cursor-pointer hover:opacity-70" src="/assets/images/plus.svg" draggable="false";
                    }
                }
                div class="flex items-center gap-3 h-full" {
                    img class="hover:opacity-70 w-5 h-5 cursor-pointer hide" src="/assets/images/minus.svg" draggable="false";
                    img class="hover:opacity-70 w-4 h-4 cursor-pointer maximize" src="/assets/images/square.svg" draggable="false";
                    img class="hover:opacity-70 w-5 h-5 cursor-pointer close" src="/assets/images/x.svg" draggable="false";
                }
            }
            div class="flex justify-between items-center bg-zinc-800 px-7 h-12 select-none gap-6" {
                div class="flex gap-6 shrink-0" {
                    img class="h-4 w-4 cursor-pointer hover:opacity-70" src="/assets/images/arrow-left.svg" draggable="false";
                    img class="h-4 w-4 cursor-pointer hover:opacity-70" src="/assets/images/arrow-right.svg" draggable="false";
                    img class="h-4 w-4 cursor-pointer hover:opacity-70" src="/assets/images/arrow-up.svg" draggable="false";
                    img class="h-4 w-4 cursor-pointer hover:opacity-70" src="/assets/images/rotate-ccw.svg" draggable="false";
                }
                div class="flex gap-4 w-full items-center" {
                    div class="flex h-full flex-2 bg-zinc-700 px-4 py-1 rounded-sm gap-3 items-center" {
                        "This PC"
                        img class="h-5 w-5 cursor-pointer hover:opacity-70" src="/assets/images/chevron-right.svg" draggable="false";
                    }
                    div class="relative flex-1" {
                        input class="w-full h-8  bg-zinc-700 text-white text-sm rounded-sm px-3" type="text" placeholder="Search This PC";
                        img src="/assets/images/search.svg" alt="search" class="top-1.5 right-3 absolute w-5 h-5" draggable="false";
                    }
                }
            }
            div class="h-full" {
                div class="w-full h-12 border-b border-zinc-700 flex items-center px-7 gap-8" {
                    div class="flex gap-2 items-center cursor-pointer hover:opacity-70 h-full" {
                        img src="/assets/images/plus.svg" class="h-5 w-5" draggable="false";
                        "New"
                        img src="/assets/images/chevron-down.svg" class="h-5 w-5" draggable="false";
                    }
                    div class="flex gap-8 items-center border-l border-zinc-700 pl-8 h-full shrink-0" {
                        img src="/assets/images/scissors.svg" class="h-5 w-5 cursor-pointer hover:opacity-70" draggable="false";
                        img src="/assets/images/copy.svg" class="h-5 w-5 cursor-pointer hover:opacity-70" draggable="false";
                        img src="/assets/images/clipboard-paste.svg" class="h-5 w-5 cursor-pointer hover:opacity-70" draggable="false";
                        img src="/assets/images/trash-2.svg" class="h-5 w-5 cursor-pointer hover:opacity-70" draggable="false";
                    }
                    div class="flex justify-between gap-4 w-full h-full" {
                        div class="flex gap-8 items-center border-l border-zinc-700 pl-8 h-full" {
                            div class="flex gap-2 items-center cursor-pointer hover:opacity-70 h-full" {
                                img src="/assets/images/arrow-up-down.svg" class="h-5 w-5" draggable="false";
                                "Sort"
                                img src="/assets/images/chevron-down.svg" class="h-5 w-5" draggable="false";
                            }
                            div class="flex gap-2 items-center cursor-pointer hover:opacity-70 h-full" {
                                img src="/assets/images/table-of-contents.svg" class="h-5 w-5" draggable="false";
                                "View"
                                img src="/assets/images/chevron-down.svg" class="h-5 w-5" draggable="false";
                            }
                        }
                        div class="flex gap-2 items-center cursor-pointer hover:opacity-70 h-full" {
                            img src="/assets/images/columns-2.svg" class="h-5 w-5" draggable="false";
                            "Preview"
                        }
                    }
                }
                div class="flex h-full gap-2" {
                    div class="flex-1 px-3 py-4 flex flex-col gap-4" {
                        div class="flex border-b border-zinc-700 pb-4 gap-2 items-center" {
                            img src="/assets/images/chevron-right.svg" class="h-5 w-5" draggable="false";
                            img src="/assets/images/thispc.svg" class="h-5 w-5" draggable="false";
                            "This PC" 
                        }
                        div class="flex gap-3 flex-col" {
                            div class="px-2" { "Desktop" }
                            div class="px-2" { "Documents" }
                            div class="px-2" { "Downloads" }
                            div class="px-2" { "Pictures" }
                            div class="px-2" { "Music" }
                            div class="px-2" { "Videos" }
                        }
                    }
                    div class="flex-5 border-l border-zinc-700" {}
                }
            }
        }
    }
}
