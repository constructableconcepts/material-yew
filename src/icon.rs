use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub icon: Option<String>,
}

#[function_component(Icon)]
pub fn icon(props: &Props) -> Html {
    html! {
        <md-icon>{ props.icon.clone().unwrap_or_default() }</md-icon>
    }
}
