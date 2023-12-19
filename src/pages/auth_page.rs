use crate::components::group::auth_form::AuthForm;
use crate::components::single::loader::Loader;
use crate::components::single::{logo::Logo, padding::Padding};
use crate::state::auth_state::AuthState;
use crate::utils::validate;
use dioxus::prelude::*;
#[component]
pub fn AuthPage(cx: Scope) -> Element {
    let is_loading = use_state(cx, || false);
    let login = use_state(cx, || "".to_string());
    let pwd = use_state(cx, || "".to_string());
    let app_state = use_shared_state::<AuthState>(cx).unwrap();
    // app_state.write().name = "Joe".to_string();// crashes here
    cx.render(rsx! {
        div{
            class: "auth_page",
            Logo{},
            Padding{height:3.0},
            AuthForm{
                login:login,
                pwd:pwd,
                on_login_input: move|event: FormEvent|{login.set(validate::auth_validate(event.value.clone(), login.to_string().clone()));},
                on_pwd_input: move|event: FormEvent|{pwd.set(validate::auth_validate(event.value.clone(), pwd.to_string().clone()));},
                on_btn_click: move |_| {app_state.write().name = "Joe".to_string();},// crashes here
            },
            // Loader{},
            p{
                "{app_state.read().name}"
            }
        }
    })
}
