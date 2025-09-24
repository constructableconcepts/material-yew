use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub unbounded: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Ripple)]
pub fn ripple(props: &Props) -> Html {
    html! {
        <md-ripple
            unbounded={props.unbounded.then(|| AttrValue::from(""))}
            disabled={props.disabled}
        >
            { for props.children.iter() }
        </md-ripple>
    }
}
