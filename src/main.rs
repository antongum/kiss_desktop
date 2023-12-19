#![allow(non_snake_case)]

use dioxus_desktop::{Config, WindowBuilder};
use dioxus_router::prelude::*;

use dioxus::prelude::*;
use log::LevelFilter;

mod api;
mod components;
mod pages;
mod router;
mod state;
mod utils;

use router::Route;

fn main() {
    // логгер
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    // запускаю приложение
    dioxus_desktop::launch_cfg(
        app,
        Config::default()
            .with_window(
                // конфигурация окна
                WindowBuilder::new()
                    .with_title("Kiss Desktop")
                    .with_maximized(true)
                    .with_window_icon(utils::icon::get_app_icon()),
            )
            // добавляю стили
            .with_custom_head(r#"<link rel="stylesheet" href="public/style.css">"#.to_string()),
    );
}

fn app(cx: Scope) -> Element {
    use_shared_state_provider(cx, || state::auth_state::AuthState::default());
    render! {
        Router::<Route> {}
    }
}
