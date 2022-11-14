#![allow(unused, non_snake_case)]
#![feature(fs_try_exists)]
mod app;
mod downloader;
use app::App;
use dioxus::desktop::tao::dpi::LogicalSize;

pub const FORMATS: [&'static str; 5] = ["mp4", "mkv", "mov", "png", "jpg"];

fn main() {
    dioxus::desktop::launch_cfg(App, |c| {
        c.with_window(|w| {
            w.with_resizable(false)
                .with_inner_size(LogicalSize {
                    width: 600.0,
                    height: 600.0,
                })
                .with_title("Convi")
        })
        .with_disable_context_menu(true)
    });
}
