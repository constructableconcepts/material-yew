use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    /// Buffer amount to display, a fraction between 0 and 1.
    #[prop_or(1.0)]
    pub buffer: f32,
    /// Progress to display, a fraction between 0 and `max`.
    #[prop_or_default]
    pub value: f32,
    /// Maximum progress to display, defaults to 1.
    #[prop_or(1.0)]
    pub max: f32,
    /// Whether or not to display indeterminate progress, which gives no indication to how long an
    /// activity will take.
    #[prop_or_default]
    pub indeterminate: bool,
    /// Whether or not to render indeterminate mode using 4 colors instead of one.
    #[prop_or_default]
    pub four_color: bool,
}

#[function_component]
pub fn LinearProgress(props: &Props) -> Html {
    crate::import_material_web_module!("/md-web/linear-progress.js");
    html! { <md-linear-progress
        buffer={props.buffer.to_string()}
        value={props.value.to_string()}
        max={props.max.to_string()}
        indeterminate={props.indeterminate.then_some(AttrValue::from(""))}
        four-color={props.four_color.then_some(AttrValue::from(""))}
    /> }
}