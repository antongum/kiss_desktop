use crate::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn WorkspacePage(cx: Scope) -> Element {
    render! {
        Link { to: Route::AuthPage {}, "Go to Auth" }
    }
}
