use yew::prelude::*;
use yew_nav::NavMenuStateProvider;
use yew_router::prelude::*;

use crate::components::nav::nav_bar::{{crate_name | pascal_case}}NavBar;
use crate::components::token_provider::TokenProvider;
use crate::pages::{login::LoginPage, logout::LogoutPage, register::RegisterPage, root::RootPage};

pub mod components;
pub mod pages;
pub mod util;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Root,
    #[at("/register")]
    Register,
    #[at("/login")]
    Login,
    #[at("/logout")]
    Logout,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Root => {
            html! { <RootPage /> }
        },
        Route::Register => {
            html! { <RegisterPage /> }
        },
        Route::Login => {
            html! { <LoginPage /> }
        },
        Route::Logout => {
            html! { <LogoutPage /> }
        },
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <TokenProvider>
            <NavMenuStateProvider>
                <BrowserRouter>
                    <header class="border-b px-4 py-2">
                        <{{crate_name | pascal_case}}NavBar />
                    </header>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </NavMenuStateProvider>
        </TokenProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
