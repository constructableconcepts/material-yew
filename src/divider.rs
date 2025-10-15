use yew::prelude::*;

/// Properties for the `Divider` component.
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
    /// The id of the divider.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// The style of the divider.
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// A divider component that separates content.
///
/// [Material Design spec](https://m3.material.io/components/divider/overview)
#[function_component(Divider)]
pub fn divider(props: &Props) -> Html {
    crate::import_material_web_module!("/md-web/divider.js");

    html! {
        <md-divider
            inset={props.inset.then_some(AttrValue::from(""))}
            inset-start={props.inset_start.then_some(AttrValue::from(""))}
            inset-end={props.inset_end.then_some(AttrValue::from(""))}
            id={props.id.clone()}
            style={props.style.clone()}
        />
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gloo_utils::document;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn it_renders_with_insets() {
        let host = document().create_element("div").unwrap();
        let props = Props {
            inset_start: true,
            inset_end: true,
            inset: true,
            id: None,
            style: None,
        };

        yew::Renderer::<Divider>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("inset-start"));
        assert!(rendered_html.contains("inset-end"));
        assert!(rendered_html.contains("inset"));
    }

    #[wasm_bindgen_test]
    fn it_renders_with_custom_style_and_id() {
        let host = document().create_element("div").unwrap();
        let props = Props {
            inset_start: false,
            inset_end: false,
            inset: false,
            id: Some("custom-id".into()),
            style: Some("opacity: 0.5;".into()),
        };

        yew::Renderer::<Divider>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("id=\"custom-id\""));
        assert!(rendered_html.contains("style=\"opacity: 0.5;\""));
    }
}