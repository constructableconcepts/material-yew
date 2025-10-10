use yew::prelude::*;

#[derive(Properties, PartialEq, Default)]
pub struct Props {
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub active: bool,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[function_component]
pub fn Tab(props: &Props) -> Html {
    html! {
        <md-tab
            disabled={props.disabled}
            active={props.active.then_some(AttrValue::from(""))}
            id={props.id.clone()}
            style={props.style.clone()}
        >
            { for props.children.iter() }
        </md-tab>
    }
}