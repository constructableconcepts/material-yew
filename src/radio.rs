use crate::customizable::CustomizableProps;
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    /// Whether or not the radio is disabled.
    #[prop_or_default]
    pub disabled: bool,
    /// The element value to use in form submission when checked.
    #[prop_or_default]
    pub value: AttrValue,
    /// Whether or not the radio is checked.
    #[prop_or_default]
    pub checked: bool,
    /// The name of the radio group.
    #[prop_or_default]
    pub name: AttrValue,
    #[prop_or_default]
    pub oninput: Callback<InputEvent>,
    #[prop_or_default]
    pub onchange: Callback<Event>,
    /// Customizable properties.
    #[prop_or_default]
    pub customizable: CustomizableProps,
}

#[function_component]
pub fn Radio(props: &Props) -> Html {
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
    crate::import_material_web_module!("/md-web/radio.js");

    html! { <md-radio
        ref={node_ref}
        disabled={props.disabled}
        value={props.value.clone()}
        checked={props.checked.then_some(AttrValue::from(""))}
        name={props.name.clone()}
        oninput={props.oninput.clone()}
        onchange={props.onchange.clone()}
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
    fn it_renders_with_custom_style_and_aria() {
        let host = document().create_element("div").unwrap();
        let mut aria = BTreeMap::new();
        aria.insert("aria-label".to_string(), "Custom Radio".into());
        let props = Props {
            disabled: false,
            value: "value".into(),
            checked: false,
            name: "group".into(),
            oninput: Callback::default(),
            onchange: Callback::default(),
            customizable: CustomizableProps {
                style: Some("color: red;".into()),
                aria: Some(aria),
            },
        };

        yew::Renderer::<Radio>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("style=\"color: red;\""));
        assert!(rendered_html.contains("aria-label=\"Custom Radio\""));
    }
}