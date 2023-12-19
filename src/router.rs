use crate::pages::auth_page::AuthPage;
use crate::pages::workspace_page::WorkspacePage;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    AuthPage {},
    #[route("/workspace")]
    WorkspacePage {},
}
