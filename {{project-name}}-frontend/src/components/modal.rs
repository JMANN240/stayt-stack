use web_sys::{
    HtmlDivElement, console,
    wasm_bindgen::{JsCast, JsValue},
};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    #[prop_or_default]
    pub children: Html,

    #[prop_or_default]
    pub shown: bool,

    #[prop_or_default]
    pub on_close: Callback<MouseEvent>,
}

#[function_component]
pub fn Modal(
    ModalProps {
        children,
        shown,
        on_close,
    }: &ModalProps,
) -> Html {
    let background_div_ref = use_node_ref();

    let onclick = {
        let on_close = on_close.clone();
        let background_div_ref = background_div_ref.clone();

        Callback::from(move |event: MouseEvent| {
            let maybe_target = event.target();

            let maybe_background_div = maybe_target.and_then(|t| t.dyn_into::<HtmlDivElement>().ok());

            if let Some(background_div) = maybe_background_div
                && let Some(background_div_ref) = background_div_ref.cast::<HtmlDivElement>()
                && background_div == background_div_ref
            {
                on_close.emit(event);
            }
        })
    };

    html! {
        <div
            ref={background_div_ref}
            class={ classes!("fixed", "top-0", "right-0", "bottom-0", "left-0", "flex", "justify-center", "items-center", "backdrop-brightness-50", if *shown { None } else { Some("hidden") }) }
            onclick={onclick}
        >
            { children.clone() }
        </div>
    }
}
