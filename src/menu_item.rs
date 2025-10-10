use crate::customizable::CustomizableProps;
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    /// Disables the item and makes it non-selectable and non-interactive.
    #[prop_or_default]
    pub disabled: bool,
    /// Sets the behavior and role of the menu item, defaults to “menuitem”.
    #[prop_or(AttrValue::from("menuitem"))]
    pub r#type: AttrValue,
    /// Sets the underlying <code>HTMLAnchorElement</code>’s <code>href</code> resource attribute.
    #[prop_or_default]
    pub href: AttrValue,
    /// Sets the underlying <code>HTMLAnchorElement</code>’s <code>target</code> attribute when
    /// <code>href</code> is set.
    #[prop_or_default]
    pub target: AttrValue,
    /// Keeps the menu open if clicked or keyboard selected.
    #[prop_or_default]
    pub keep_open: bool,
    /// Sets the item in the selected visual state when a submenu is opened.
    #[prop_or_default]
    pub selected: bool,
    /// The text to use for typeahead functionality.
    #[prop_or_default]
    pub typeahead_text: AttrValue,
    /// The value that the menu item represents.
    #[prop_or_default]
    pub value: AttrValue,
    #[prop_or_default]
    pub children: Html,
    /// Customizable properties.
    #[prop_or_default]
    pub customizable: CustomizableProps,
}

#[function_component]
pub fn MenuItem(props: &Props) -> Html {
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
    crate::import_material_web_module!("/md-web/menu-item.js");

    html! { <md-menu-item
       ref={node_ref}
       disabled={props.disabled}
       type={props.r#type.clone()}
       href={props.href.clone()}
       target={props.target.clone()}
       keep-open={props.keep_open.then_some(AttrValue::from(""))}
       selected={props.selected}
       typeahead-text={props.typeahead_text.clone()}
       value={props.value.clone()}
    >
        {props.children.clone()}
    </md-menu-item> }
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
        aria.insert("aria-label".to_string(), "Custom Menu Item".into());
        let props = Props {
            disabled: false,
            r#type: "menuitem".into(),
            href: AttrValue::default(),
            target: AttrValue::default(),
            keep_open: false,
            selected: false,
            typeahead_text: AttrValue::default(),
            children: html! { "Test Menu Item" },
            customizable: CustomizableProps {
                style: Some("color: brown;".into()),
                aria: Some(aria),
            },
        };

        yew::Renderer::<MenuItem>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("style=\"color: brown;\""));
        assert!(rendered_html.contains("aria-label=\"Custom Menu Item\""));
    }
}