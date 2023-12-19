use dioxus::prelude::*;
#[derive(Props)]
pub struct InputProps<'a> {
    input_type: &'a str,
    placeholder: &'a str,
    value: &'a str,
    #[props(default = 20)]
    maxlength: i32,
    on_input: EventHandler<'a, FormEvent>,
}
#[component]
pub fn Input<'a>(cx: Scope<'a, InputProps<'a>>) -> Element {
    cx.render(rsx! {
        input {
            class:"input",
            r#type: "{cx.props.input_type}",
            placeholder: "{cx.props.placeholder}",
            value:"{cx.props.value}",
            maxlength:"{cx.props.maxlength}",
            oninput: move |event| cx.props.on_input.call(event),
        }
    })
}
