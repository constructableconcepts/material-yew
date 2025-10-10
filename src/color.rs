use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub value: AttrValue,
    #[prop_or_default]
    pub disabled: bool,
}

#[function_component(Color)]
pub fn color(props: &Props) -> Html {
    html! {
        <md-color
            value={props.value.clone()}
            disabled={props.disabled}
        />
    }
}