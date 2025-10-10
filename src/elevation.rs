use yew::prelude::*;

/// The `Elevation` component is a visual representation of elevation.
///
/// [Material Design spec](https://m3.material.io/styles/elevation/overview)
///
/// `Elevation` is a visual effect that indicates the separation between surfaces.
/// It is not controlled by props, but rather by CSS custom properties on the
/// parent element.
///
/// To use, add the `Elevation` component to a container that has `position: relative`.
/// The elevation level is controlled by setting the `--md-elevation-level` CSS
/// custom property on the parent.
///
/// ## Example
///
/// ```html
/// <div style="position: relative; --md-elevation-level: 2;">
///   <p>{"This container has elevation level 2."}</p>
///   <Elevation />
/// </div>
/// ```
#[derive(Properties, PartialEq, Clone, Default)]
pub struct Props {
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[function_component(Elevation)]
pub fn elevation(props: &Props) -> Html {
    crate::import_material_web_module!("/md-web/core.js");

    html! {
        <md-elevation id={props.id.clone()} style={props.style.clone()} />
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gloo_utils::document;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn it_renders() {
        let host = document().create_element("div").unwrap();
        let props = Props::default();

        yew::Renderer::<Elevation>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("<md-elevation"));
    }

    #[wasm_bindgen_test]
    fn it_renders_with_custom_style_and_id() {
        let host = document().create_element("div").unwrap();
        let props = Props {
            id: Some("custom-id".into()),
            style: Some("opacity: 0.8;".into()),
        };

        yew::Renderer::<Elevation>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("id=\"custom-id\""));
        assert!(rendered_html.contains("style=\"opacity: 0.8;\""));
    }
}