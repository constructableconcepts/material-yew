use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[function_component(Focus)]
pub fn focus(props: &Props) -> Html {
    html! {
        <md-focus disabled={props.disabled} id={props.id.clone()} style={props.style.clone()}>
            { for props.children.iter() }
        </md-focus>
    }
}