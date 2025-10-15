use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Default)]
pub struct Props {
    #[prop_or_default]
    pub unbounded: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[function_component(Ripple)]
pub fn ripple(props: &Props) -> Html {
    html! {
        <md-ripple
            unbounded={props.unbounded.then_some(AttrValue::from(""))}
            disabled={props.disabled}
            id={props.id.clone()}
            style={props.style.clone()}
        >
            { for props.children.iter() }
        </md-ripple>
    }
}