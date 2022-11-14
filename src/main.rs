#![allow(non_snake_case)]
mod app;
use app::App;
use dioxus::desktop::tao::dpi::LogicalSize;

fn main() {
    dioxus::desktop::launch_cfg(App, |c| {
        c.with_window(|w| {
            w.with_resizable(true)
                .with_inner_size(LogicalSize {
                    width: 600.0,
                    height: 600.0,
                })
                .with_title("Convi")
        })
        .with_disable_context_menu(true)
    });
}
