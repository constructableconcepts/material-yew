use crate::customizable::CustomizableProps;
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    /// Disables the item and makes it non-selectable and non-interactive.
    #[prop_or_default]
    pub disabled: bool,
    /// Sets the behavior of the list item, defaults to “text”. Change to “link” or “button” for
    /// interactive items.
    #[prop_or(AttrValue::from("text"))]
    pub r#type: AttrValue,
    /// Sets the underlying <code>HTMLAnchorElement</code>’s <code>href</code> resource attribute.
    #[prop_or_default]
    pub href: AttrValue,
    /// Sets the underlying <code>HTMLAnchorElement</code>’s <code>target</code> attribute when
    /// <code>href</code> is set.
    #[prop_or_default]
    pub target: AttrValue,
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub onfocus: Callback<FocusEvent>,
    /// Customizable properties.
    #[prop_or_default]
    pub customizable: CustomizableProps,
}

#[function_component]
pub fn ListItem(props: &Props) -> Html {
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
    crate::import_material_web_module!("/md-web/list-item.js");

    html! { <md-list-item
        ref={node_ref}
        disabled={props.disabled}
        type={props.r#type.clone()}
        href={props.href.clone()}
        target={props.target.clone()}
        onfocus={props.onfocus.clone()}
    > {props.children.clone()} </md-list-item> }
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
        aria.insert("aria-label".to_string(), "Custom List Item".into());
        let props = Props {
            disabled: false,
            r#type: "button".into(),
            href: AttrValue::default(),
            target: AttrValue::default(),
            children: html! { "Test List Item" },
            onfocus: Callback::default(),
            customizable: CustomizableProps {
                style: Some("color: orange;".into()),
                aria: Some(aria),
            },
        };

        yew::Renderer::<ListItem>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("style=\"color: orange;\""));
        assert!(rendered_html.contains("aria-label=\"Custom List Item\""));
    }
}