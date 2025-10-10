use crate::customizable::CustomizableProps;
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    /// Disables the switch and makes it non-interactive.
    #[prop_or_default]
    pub disabled: bool,
    /// Puts the switch in the selected state and sets the form submission value to the
    /// <code>value</code> property.
    #[prop_or_default]
    pub selected: bool,
    /// Shows both the selected and deselected icons.
    #[prop_or_default]
    pub icons: bool,
    /// Shows only the selected icon, and not the deselected icon. If <code>true</code>, overrides
    /// the behavior of the <code>icons</code> property.
    #[prop_or_default]
    pub show_only_selected_icon: bool,
    /// When true, require the switch to be selected when participating in form submission.
    #[prop_or_default]
    pub required: bool,
    /// The value associated with this switch on form submission.
    #[prop_or_default]
    pub value: AttrValue,
    /// The name of the switch.
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
pub fn Switch(props: &Props) -> Html {
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

    crate::import_material_web_module!("/md-web/switch.js");

    html! { <md-switch
        ref={node_ref}
        disabled={props.disabled}
        selected={props.selected}
        icons={props.icons.then_some(AttrValue::from(""))}
        show-only-selected-icon={props.show_only_selected_icon.then_some(AttrValue::from(""))}
        required={props.required}
        value={props.value.clone()}
        name={props.name.clone()}
        oninput={props.oninput.clone()}
        onchange={props.onchange.clone()}
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
        aria.insert("aria-label".to_string(), "Custom Switch".into());
        let props = Props {
            disabled: false,
            selected: false,
            icons: false,
            show_only_selected_icon: false,
            required: false,
            value: AttrValue::default(),
            name: AttrValue::default(),
            oninput: Callback::default(),
            onchange: Callback::default(),
            customizable: CustomizableProps {
                style: Some("color: hotpink;".into()),
                aria: Some(aria),
            },
        };

        yew::Renderer::<Switch>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("style=\"color: hotpink;\""));
        assert!(rendered_html.contains("aria-label=\"Custom Switch\""));
    }
}