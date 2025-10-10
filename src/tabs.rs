use crate::customizable::CustomizableProps;
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or(0)]
    pub active_index: u32,
    #[prop_or_default]
    pub children: Children,
    /// Customizable properties.
    #[prop_or_default]
    pub customizable: CustomizableProps,
}

#[function_component(Tabs)]
pub fn tabs(props: &Props) -> Html {
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

    html! {
        <md-tabs ref={node_ref} active-index={props.active_index.to_string()}>
            { for props.children.iter() }
        </md-tabs>
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
    fn it_renders_with_custom_style_and_aria() {
        let host = document().create_element("div").unwrap();
        let mut aria = BTreeMap::new();
        aria.insert("aria-label".to_string(), "Custom Tabs".into());
        let props = Props {
            active_index: 0,
            children: Children::new(vec![html! { <div /> }]),
            customizable: CustomizableProps {
                style: Some("background-color: yellow;".into()),
                aria: Some(aria),
            },
        };

        yew::Renderer::<Tabs>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("style=\"background-color: yellow;\""));
        assert!(rendered_html.contains("aria-label=\"Custom Tabs\""));
    }
}