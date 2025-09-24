use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub label: Option<String>,
    #[prop_or_default]
    pub selected: bool,
    #[prop_or_default]
    pub disabled: bool,
}

#[function_component(Chips)]
pub fn chips(props: &Props) -> Html {
    html! {
        <md-chips
            label={props.label.clone().unwrap_or_default()}
            selected={props.selected}
            disabled={props.disabled}
        />
    }
}
