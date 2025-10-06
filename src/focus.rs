use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Focus)]
pub fn focus(props: &Props) -> Html {
    html! {
        <md-focus disabled={props.disabled}>
            { for props.children.iter() }
        </md-focus>
    }
}
