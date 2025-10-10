use crate::customizable::CustomizableProps;
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub label: AttrValue,
    #[prop_or_default]
    pub value: AttrValue,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub children: Children,
    /// Customizable properties.
    #[prop_or_default]
    pub customizable: CustomizableProps,
}

#[function_component(Select)]
pub fn select(props: &Props) -> Html {
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
        <md-select
            ref={node_ref}
            label={props.label.clone()}
            value={props.value.clone()}
            disabled={props.disabled}
        >
            { for props.children.iter() }
        </md-select>
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
        aria.insert("aria-label".to_string(), "Custom Select".into());
        let props = Props {
            label: "Label".into(),
            value: "Value".into(),
            disabled: false,
            children: Children::new(vec![]),
            customizable: CustomizableProps {
                style: Some("color: purple;".into()),
                aria: Some(aria),
            },
        };

        yew::Renderer::<Select>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("style=\"color: purple;\""));
        assert!(rendered_html.contains("aria-label=\"Custom Select\""));
    }
}