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
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[function_component]
pub fn MenuItem(props: &Props) -> Html {
    crate::import_material_web_module!("/md-web/menu-item.js");

    html! { <md-menu-item
       disabled={props.disabled}
       type={props.r#type.clone()}
       href={props.href.clone()}
       target={props.target.clone()}
       keep-open={props.keep_open.then_some(AttrValue::from(""))}
       selected={props.selected}
       typeahead-text={props.typeahead_text.clone()}
       value={props.value.clone()}
       id={props.id.clone()}
       style={props.style.clone()}
    >
        {props.children.clone()}
    </md-menu-item> }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gloo_utils::document;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn it_renders_with_custom_style_and_id() {
        let host = document().create_element("div").unwrap();
        let props = Props {
            disabled: false,
            r#type: "menuitem".into(),
            href: AttrValue::default(),
            target: AttrValue::default(),
            keep_open: false,
            selected: false,
            typeahead_text: AttrValue::default(),
            value: AttrValue::default(),
            children: html! { "Test Menu Item" },
            id: Some("custom-id".into()),
            style: Some("color: brown;".into()),
        };

        yew::Renderer::<MenuItem>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("id=\"custom-id\""));
        assert!(rendered_html.contains("style=\"color: brown;\""));
    }
}