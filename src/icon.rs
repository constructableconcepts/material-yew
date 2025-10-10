use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub icon: AttrValue,
}

#[function_component(Icon)]
pub fn icon(props: &Props) -> Html {
    html! {
        <md-icon>{ props.icon.clone() }</md-icon>
    }
}