use crate::components::single::{btn::Btn, input::Input, padding::Padding};
use dioxus::prelude::*;

#[derive(Props)]
pub struct AuthFormProps<'a> {
    login: &'a str,
    pwd: &'a str,
    on_login_input: EventHandler<'a, FormEvent>,
    on_pwd_input: EventHandler<'a, FormEvent>,
    on_btn_click: EventHandler<'a>,
}

#[component]
pub fn AuthForm<'a>(cx: Scope<'a, AuthFormProps<'a>>) -> Element {
    cx.render(rsx! {
         form{
             class: "auth_form",
             Input{
                 input_type:"text",
                 placeholder:"Логин",
                 value:cx.props.login,
                 on_input: move |event| cx.props.on_login_input.call(event),
            },
             Padding{},
             Input{
                 input_type:"password",
                 placeholder:"Пароль",
                 value:cx.props.pwd,
                 on_input: move |event| cx.props.on_pwd_input.call(event),
             },
             Padding{},
             Btn{
                 title:"Войти",
                 on_click: move |_| cx.props.on_btn_click.call(())
             },
         }
    })
}
