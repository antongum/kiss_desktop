use dioxus::prelude::*;
#[derive(Props)]
pub struct BtnProps<'a> {
    title: &'a str,
    on_click: EventHandler<'a>,
}
#[component]
pub fn Btn<'a>(cx: Scope<'a, BtnProps<'a>>) -> Element {
    cx.render(rsx! {
        button{
            class: "btn",
            onclick: move |_| cx.props.on_click.call(()),
            "{cx.props.title}",
        }
    })
}
