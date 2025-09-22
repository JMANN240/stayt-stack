use pages::{home::Home, levels::Levels};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::navbar::Navbar;
use crate::components::token_provider::TokenProvider;
use crate::pages::level::Level;
use crate::pages::login::Login;
use crate::pages::logout::Logout;
use crate::pages::register::Register;
use crate::pages::upload::Upload;

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
        Route::Home => {
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
            <BrowserRouter>
                <header class="border-b px-4 py-2">
                    <Navbar />
                </header>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </TokenProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
