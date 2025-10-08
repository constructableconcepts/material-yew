use yew::prelude::*;
#[derive(Properties, PartialEq)]
pub struct Props {
    /// Progress to display, a fraction between 0 and <code>max</code>.
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
pub fn CircularProgress(props: &Props) -> Html {
    crate::import_material_web_module!("/md-web/circular-progress.js");
    html! { <md-circular-progress
        value={props.value.to_string()}
        max={props.max.to_string()}
        indeterminate={props.indeterminate.then(|| AttrValue::from(""))}
        four-color={props.four_color.then(|| AttrValue::from(""))}
    /> }
}
