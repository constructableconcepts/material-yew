use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub progress: Option<f32>,
    #[prop_or_default]
    pub indeterminate: bool,
    #[prop_or_default]
    pub four_color: bool,
}

#[function_component(Progress)]
pub fn progress(props: &Props) -> Html {
    html! {
        <md-progress
            progress={props.progress.unwrap_or(0.0)}
            indeterminate={props.indeterminate}
            four_color={props.four_color}
        />
    }
}
