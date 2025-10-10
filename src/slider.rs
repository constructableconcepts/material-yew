use crate::customizable::CustomizableProps;
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    /// Whether or not the slider is disabled.
    #[prop_or_default]
    pub disabled: bool,
    /// The slider minimum value
    #[prop_or_default]
    pub min: f32,
    /// The slider maximum value
    #[prop_or(100.0)]
    pub max: f32,
    /// The slider value displayed when range is false.
    #[prop_or_default]
    pub value: f32,
    /// The slider start value displayed when range is true.
    #[prop_or_default]
    pub value_start: f32,
    /// The slider end value displayed when range is true.
    #[prop_or(100.0)]
    pub value_end: f32,
    /// An optional label for the slider’s value displayed when range is false.
    #[prop_or_default]
    pub value_label: AttrValue,
    /// An optional label for the slider’s start value displayed when range is true.
    #[prop_or_default]
    pub value_label_start: AttrValue,
    /// An optional label for the slider’s end value displayed when range is true.
    #[prop_or_default]
    pub value_label_end: AttrValue,
    /// Aria label for the slider’s start handle displayed when range is true.
    #[prop_or_default]
    pub aria_label_start: AttrValue,
    /// Aria value text for the slider’s start value displayed when range is true.
    #[prop_or_default]
    pub aria_value_text_start: AttrValue,
    /// Aria label for the slider’s end handle displayed when range is true.
    #[prop_or_default]
    pub aria_label_end: AttrValue,
    /// Aria value text for the slider’s end value displayed when range is true.
    #[prop_or_default]
    pub aria_value_text_end: AttrValue,
    /// The step between values.
    #[prop_or(1.0)]
    pub step: f32,
    /// Whether or not to show tick marks.
    #[prop_or_default]
    pub ticks: bool,
    /// Whether or not to show a value label when activated.
    #[prop_or_default]
    pub labeled: bool,
    /// Whether or not to show a value range.
    #[prop_or_default]
    pub range: bool,
    /// The name of the slider.
    #[prop_or_default]
    pub name: AttrValue,
    /// The name of the slider's start handle.
    #[prop_or_default]
    pub name_start: AttrValue,
    /// The name of the slider's end handle.
    #[prop_or_default]
    pub name_end: AttrValue,
    /// Customizable properties.
    #[prop_or_default]
    pub customizable: CustomizableProps,
}

#[function_component]
pub fn Slider(props: &Props) -> Html {
    let node_ref = use_node_ref();
    let customizable = props.customizable.clone();
    use_effect_with((node_ref.clone(), customizable), |(node_ref, customizable)| {
        if let Some(element) = node_ref.get() {
            let element = element.dyn_ref::<Element>().unwrap();

            if let Some(style) = &customizable.style {
                element.set_attribute("style", style).unwrap();
            }

            if let Some(aria) = &customizable.aria {
                for (key, value) in aria {
                    if key.starts_with("aria-") {
                        element.set_attribute(key, value).unwrap();
                    }
                }
            }
        }
    });

    crate::import_material_web_module!("/md-web/slider.js");

    html! { <md-slider
        ref={node_ref}
        disabled={props.disabled}
        min={props.min.to_string()}
        max={props.max.to_string()}
        value={props.value.to_string()}
        value-start={props.value_start.to_string()}
        value-end={props.value_end.to_string()}
        value-label={props.value_label.clone()}
        value-label-start={props.value_label_start.clone()}
        value-label-end={props.value_label_end.clone()}
        aria-label-start={props.aria_label_start.clone()}
        aria-value-text-start={props.aria_value_text_start.clone()}
        aria-label-end={props.aria_label_end.clone()}
        aria-value-text-end={props.aria_value_text_end.clone()}
        step={props.step.to_string()}
        ticks={props.ticks.then_some(AttrValue::from(""))}
        labeled={props.labeled.then_some(AttrValue::from(""))}
        range={props.range.then_some(AttrValue::from(""))}
        name={props.name.clone()}
        name-start={props.name_start.clone()}
        name-end={props.name_end.clone()}
    />}
}

#[cfg(test)]
mod tests {
    use super::*;
    use gloo_utils::document;
    use std::collections::BTreeMap;
    use wasm_bindgen_test::*;
    use yew::prelude::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn it_renders_with_custom_style_and_aria() {
        let host = document().create_element("div").unwrap();
        let mut aria = BTreeMap::new();
        aria.insert("aria-label".to_string(), "Custom Slider".into());
        let props = Props {
            disabled: false,
            min: 0.0,
            max: 100.0,
            value: 50.0,
            value_start: 0.0,
            value_end: 100.0,
            value_label: AttrValue::default(),
            value_label_start: AttrValue::default(),
            value_label_end: AttrValue::default(),
            aria_label_start: AttrValue::default(),
            aria_value_text_start: AttrValue::default(),
            aria_label_end: AttrValue::default(),
            aria_value_text_end: AttrValue::default(),
            step: 1.0,
            ticks: false,
            labeled: false,
            range: false,
            name: AttrValue::default(),
            name_start: AttrValue::default(),
            name_end: AttrValue::default(),
            customizable: CustomizableProps {
                style: Some("width: 300px;".into()),
                aria: Some(aria),
            },
        };

        yew::Renderer::<Slider>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("style=\"width: 300px;\""));
        assert!(rendered_html.contains("aria-label=\"Custom Slider\""));
    }
}