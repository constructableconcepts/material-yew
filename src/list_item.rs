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
    /// The content of the list item.
    #[prop_or_default]
    pub children: Html,
    /// The callback to be called when the list item is focused.
    #[prop_or_default]
    pub onfocus: Callback<FocusEvent>,
    /// The id of the list item.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// The style of the list item.
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// A list item component.
///
/// [Material Design spec](https://m3.material.io/components/lists/overview)
#[function_component]
pub fn ListItem(props: &Props) -> Html {
    crate::import_material_web_module!("/md-web/list-item.js");

    html! { <md-list-item
        disabled={props.disabled}
        type={props.r#type.clone()}
        href={props.href.clone()}
        target={props.target.clone()}
        onfocus={props.onfocus.clone()}
        id={props.id.clone()}
        style={props.style.clone()}
    > {props.children.clone()} </md-list-item> }
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
            r#type: "button".into(),
            href: AttrValue::default(),
            target: AttrValue::default(),
            children: html! { "Test List Item" },
            onfocus: Callback::default(),
            id: Some("custom-id".into()),
            style: Some("color: orange;".into()),
        };

        yew::Renderer::<ListItem>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("id=\"custom-id\""));
        assert!(rendered_html.contains("style=\"color: orange;\""));
    }
}