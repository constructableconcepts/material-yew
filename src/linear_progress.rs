use crate::customizable::CustomizableProps;
use wasm_bindgen::JsCast;
use web_sys::Element;
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
    /// Customizable properties.
    #[prop_or_default]
    pub customizable: CustomizableProps,
}

#[function_component]
pub fn LinearProgress(props: &Props) -> Html {
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

    crate::import_material_web_module!("/md-web/linear-progress.js");

    html! { <md-linear-progress
        ref={node_ref}
        buffer={props.buffer.to_string()}
        value={props.value.to_string()}
        max={props.max.to_string()}
        indeterminate={props.indeterminate.then_some(AttrValue::from(""))}
        four-color={props.four_color.then_some(AttrValue::from(""))}
    /> }
}