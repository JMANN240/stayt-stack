use yew::prelude::*;
use yew_nav::NavLink;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct {{crate_name | pascal_case}}NavLinkProps<R: PartialEq> {
    pub to: R,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn {{crate_name | pascal_case}}NavLink<R: Routable + 'static>(
    {{crate_name | pascal_case}}NavLinkProps {
        to,
        classes,
        children,
    }: &{{crate_name | pascal_case}}NavLinkProps<R>,
) -> Html {
    html! {
        <NavLink<R>
            classes={classes!("border-y", "border-t-transparent", classes.clone())}
            inactive_classes={classes!("border-b-transparent")}
            active_classes={classes!("border-b")}
            to={to.clone()}
        >
            { children.clone() }
        </NavLink<R>>
    }
}
