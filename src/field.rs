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

#[function_component(Field)]
pub fn field(props: &Props) -> Html {
    html! {
        <md-field
            label={props.label.clone()}
            value={props.value.clone()}
            disabled={props.disabled}
        >
            { for props.children.iter() }
        </md-field>
    }
}