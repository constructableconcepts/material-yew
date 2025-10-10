use crate::customizable::CustomizableProps;
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    /// Increases the leading space of the divider.
    #[prop_or_default]
    pub inset_start: bool,
    /// Increases the trailing space of the divider.
    #[prop_or_default]
    pub inset_end: bool,
    /// Indents the divider with equal padding on both sides.
    #[prop_or_default]
    pub inset: bool,
    /// Customizable properties.
    #[prop_or_default]
    pub customizable: CustomizableProps,
}

#[function_component(Divider)]
pub fn divider(props: &Props) -> Html {
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

    crate::import_material_web_module!("/md-web/divider.js");

    html! {
        <md-divider
            ref={node_ref}
            inset={props.inset.then_some(AttrValue::from(""))}
            inset-start={props.inset_start.then_some(AttrValue::from(""))}
            inset-end={props.inset_end.then_some(AttrValue::from(""))}
        />
    }
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
    fn it_renders_with_insets() {
        let host = document().create_element("div").unwrap();
        let props = Props {
            inset_start: true,
            inset_end: true,
            inset: true,
            customizable: CustomizableProps::default(),
        };

        yew::Renderer::<Divider>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("inset-start"));
        assert!(rendered_html.contains("inset-end"));
        assert!(rendered_html.contains("inset"));
    }

    #[wasm_bindgen_test]
    fn it_renders_with_custom_style_and_aria() {
        let host = document().create_element("div").unwrap();
        let mut aria = BTreeMap::new();
        aria.insert("aria-hidden".to_string(), "true".into());
        let props = Props {
            inset_start: false,
            inset_end: false,
            inset: false,
            customizable: CustomizableProps {
                style: Some("opacity: 0.5;".into()),
                aria: Some(aria),
            },
        };

        yew::Renderer::<Divider>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("style=\"opacity: 0.5;\""));
        assert!(rendered_html.contains("aria-hidden=\"true\""));
    }
}