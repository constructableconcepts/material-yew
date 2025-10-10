use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub label: AttrValue,
    #[prop_or_default]
    pub value: AttrValue,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Select)]
pub fn select(props: &Props) -> Html {
    html! {
        <md-select
            label={props.label.clone()}
            value={props.value.clone()}
            disabled={props.disabled}
        >
            { for props.children.iter() }
        </md-select>
    }
}