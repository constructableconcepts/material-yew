use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    /// Whether or not the slider is disabled.
    #[prop_or_default]
    pub disabled: bool,
    /// The slider minimum value
    #[prop_or_default]
    pub min: usize,
    /// The slider maximum value
    #[prop_or(100)]
    pub max: usize,
    /// The slider value displayed when range is false.
    #[prop_or_default]
    pub value: Option<usize>,
    /// The slider start value displayed when range is true.
    #[prop_or_default]
    pub value_start: Option<usize>,
    /// The slider end value displayed when range is true.
    #[prop_or_default]
    pub value_end: Option<usize>,
    /// An optional label for the slider’s value displayed when range is false; if not set, the
    /// label is the value itself.
    #[prop_or_default]
    pub value_label: Option<AttrValue>,
    /// An optional label for the slider’s start value displayed when range is true; if not set,
    /// the label is the valueStart itself.
    #[prop_or_default]
    pub value_label_start: Option<AttrValue>,
    /// An optional label for the slider’s end value displayed when range is true; if not set, the
    /// label is the valueEnd itself.
    #[prop_or_default]
    pub value_label_end: Option<AttrValue>,
    /// Aria label for the slider’s start handle displayed when range is true.
    #[prop_or_default]
    pub aria_label_start: Option<AttrValue>,
    /// Aria value text for the slider’s start value displayed when range is true.
    #[prop_or_default]
    pub aria_value_text_start: Option<AttrValue>,
    /// Aria label for the slider’s end handle displayed when range is true.
    #[prop_or_default]
    pub aria_label_end: Option<AttrValue>,
    /// Aria value text for the slider’s end value displayed when range is true.
    #[prop_or_default]
    pub aria_value_text_end: Option<AttrValue>,
    /// The step between values.
    #[prop_or(1)]
    pub step: usize,
    /// Whether or not to show tick marks.
    #[prop_or_default]
    pub ticks: bool,
    /// Whether or not to show a value label when activated.
    #[prop_or_default]
    pub labeled: bool,
    /// Whether or not to show a value range. When false, the slider displays a slideable handle
    /// for the value property; when true, it displays slideable handles for the valueStart and
    /// valueEnd properties.
    #[prop_or_default]
    pub range: bool,
    ///
    #[prop_or_default]
    pub name: Option<AttrValue>,
    ///
    #[prop_or_default]
    pub name_start: Option<AttrValue>,
    ///
    #[prop_or_default]
    pub name_end: Option<AttrValue>,
}

#[function_component]
pub fn Slider(props: &Props) -> Html {
    crate::import_material_web_module!("/md-web/slider.js");
    html! { <md-slider
        disabled={props.disabled}
        min={props.min.to_string()}
        max={props.max.to_string()}
        value={props.value.map(|v| v.to_string())}
        value-start={props.value_start.map(|v| v.to_string())}
        value-end={props.value_end.map(|v| v.to_string())}
        value-label={props.value_label.clone()}
        value-label-start={props.value_label_start.clone()}
        value-label-end={props.value_label_end.clone()}
        aria-label-start={props.aria_label_start.clone()}
        aria-value-text-start={props.aria_value_text_start.clone()}
        aria-label-end={props.aria_label_end.clone()}
        aria-value-text-end={props.aria_value_text_end.clone()}
        step={props.step.to_string()}
        ticks={props.ticks.then(|| AttrValue::from(""))}
        labeled={props.labeled.then(|| AttrValue::from(""))}
        range={props.range.then(|| AttrValue::from(""))}
        name={props.name.clone()}
        name-start={props.name_start.clone()}
        name-end={props.name_end.clone()}
    />}
}
