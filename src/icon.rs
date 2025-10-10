use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub icon: AttrValue,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[function_component(Icon)]
pub fn icon(props: &Props) -> Html {
    html! {
        <md-icon id={props.id.clone()} style={props.style.clone()}>{ props.icon.clone() }</md-icon>
    }
}