use dioxus::prelude::*;
#[derive(PartialEq, Props)]
pub struct DefaultProps {
    #[props(default = 1.5)]
    height: f32,
}
pub fn Padding(cx: Scope<DefaultProps>) -> Element {
    render!(div {
        height: "{cx.props.height}rem"
    })
}
