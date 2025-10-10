use crate::customizable::CustomizableProps;
use wasm_bindgen::JsCast;
use web_sys::Element;
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
    /// Customizable properties.
    #[prop_or_default]
    pub customizable: CustomizableProps,
}

#[function_component]
pub fn CircularProgress(props: &Props) -> Html {
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

    crate::import_material_web_module!("/md-web/circular-progress.js");

    html! { <md-circular-progress
        ref={node_ref}
        value={props.value.to_string()}
        max={props.max.to_string()}
        indeterminate={props.indeterminate.then_some(AttrValue::from(""))}
        four-color={props.four_color.then_some(AttrValue::from(""))}
    /> }
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
    fn it_renders_with_f32() {
        let host = document().create_element("div").unwrap();
        let props = Props {
            value: 0.5,
            max: 2.0,
            indeterminate: false,
            four_color: false,
            customizable: CustomizableProps::default(),
        };

        yew::Renderer::<CircularProgress>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("value=\"0.5\""));
        assert!(rendered_html.contains("max=\"2\""));
    }

    #[wasm_bindgen_test]
    fn it_renders_with_custom_style_and_aria() {
        let host = document().create_element("div").unwrap();
        let mut aria = BTreeMap::new();
        aria.insert("aria-label".to_string(), "Loading...".into());
        let props = Props {
            value: 0.5,
            max: 2.0,
            indeterminate: false,
            four_color: false,
            customizable: CustomizableProps {
                style: Some("width: 100px;".into()),
                aria: Some(aria),
            },
        };

        yew::Renderer::<CircularProgress>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("style=\"width: 100px;\""));
        assert!(rendered_html.contains("aria-label=\"Loading...\""));
    }
}