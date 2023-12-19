use dioxus::prelude::*;

#[component]
pub fn Loader(cx: Scope) -> Element {
    cx.render(rsx! {
        div{
            class: "loader",
        }
    })
}
