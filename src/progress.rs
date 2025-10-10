use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub progress: f32,
    #[prop_or_default]
    pub indeterminate: bool,
    #[prop_or_default]
    pub four_color: bool,
}

#[function_component(Progress)]
pub fn progress(props: &Props) -> Html {
    html! {
        <md-progress
            progress={props.progress.to_string()}
            indeterminate={props.indeterminate.then_some(AttrValue::from(""))}
            four-color={props.four_color.then_some(AttrValue::from(""))}
        />
    }
}