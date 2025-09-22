use yew::prelude::*;
use yew_router::components::Link;

use crate::{Route, components::token_provider::TokenContext};

#[function_component]
pub fn Navbar() -> Html {
    let token_context = use_context::<TokenContext>().expect("no token found");

    html! {
        <nav class="flex justify-between items-center">
            <div class="flex items-center gap-4">
                <Link<Route> to={Route::Root}>
                    <h1 class="text-xl">{ "{{project-name}}" }</h1>
                </Link<Route>>
            </div>
            <div class="flex gap-4">
                if token_context.0.is_some() {
                    <Link<Route> to={Route::Logout}>
                        <h4 style="color: #00FF00;">{ "Logout" }</h4>
                    </Link<Route>>
                } else {
                    <Link<Route> to={Route::Login}>
                        <h4 style="color: #FFFF00;">{ "Login" }</h4>
                    </Link<Route>>
                    <Link<Route> to={Route::Register}>
                        <h4 style="color: #00FF00;">{ "Register" }</h4>
                    </Link<Route>>
                }
            </div>
        </nav>
    }
}
