use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub level: Option<u8>,
}

#[function_component(Elevation)]
pub fn elevation(props: &Props) -> Html {
    html! {
    <md-elevation level={props.level.unwrap_or(1).to_string()} />
    }
}
