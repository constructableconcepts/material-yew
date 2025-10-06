use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub value: Option<String>,
    #[prop_or_default]
    pub disabled: bool,
}

#[function_component(Color)]
pub fn color(props: &Props) -> Html {
    html! {
        <md-color
            value={props.value.clone().unwrap_or_default()}
            disabled={props.disabled}
        />
    }
}
