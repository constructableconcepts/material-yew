use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub label: Option<String>,
    #[prop_or_default]
    pub value: Option<String>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Field)]
pub fn field(props: &Props) -> Html {
    html! {
        <md-field
            label={props.label.clone().unwrap_or_default()}
            value={props.value.clone().unwrap_or_default()}
            disabled={props.disabled}
        >
            { for props.children.iter() }
        </md-field>
    }
}
