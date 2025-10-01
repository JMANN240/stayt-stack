use gloo::storage::{LocalStorage, Storage};
use yew::prelude::*;
use yew_nav::use_hide_nav_menu;
use yew_router::prelude::*;

use crate::{
    Route,
    components::token_provider::{TokenAction, TokenContext},
};

#[function_component]
pub fn LogoutPage() -> Html {
    use_hide_nav_menu(());

    let token_context = use_context::<TokenContext>().expect("no token found");

    token_context.dispatch(TokenAction::Clear);
    LocalStorage::delete("token");

    html! {
        <Redirect<Route> to={Route::Root} />
    }
}
