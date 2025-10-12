use yew::prelude::*;

/// Properties for the `Tabs` component.
#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    /// The index of the active tab.
    #[prop_or(0)]
    pub active_index: u32,
    /// The tabs to display.
    #[prop_or_default]
    pub children: Children,
    /// The id of the tabs.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// The style of the tabs.
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// A tabs component.
///
/// [Material Design spec](https://m3.material.io/components/tabs/overview)
#[function_component(Tabs)]
pub fn tabs(props: &Props) -> Html {
    crate::import_material_web_module!("/md-web/tabs.js");

    html! {
        <md-tabs
            active-index={props.active_index.to_string()}
            id={props.id.clone()}
            style={props.style.clone()}
        >
            { for props.children.iter() }
        </md-tabs>
    }
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
            active_index: 0,
            children: Children::new(vec![html! { <div /> }]),
            id: Some("custom-id".into()),
            style: Some("background-color: yellow;".into()),
        };

        yew::Renderer::<Tabs>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("id=\"custom-id\""));
        assert!(rendered_html.contains("style=\"background-color: yellow;\""));
    }
}