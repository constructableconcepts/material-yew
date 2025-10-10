use crate::customizable::CustomizableProps;
use wasm_bindgen::JsCast;
use web_sys::Element;
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
#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    /// Customizable properties.
    #[prop_or_default]
    pub customizable: CustomizableProps,
}

#[function_component(Elevation)]
pub fn elevation(props: &Props) -> Html {
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

    crate::import_material_web_module!("/md-web/core.js");

    html! {
        <md-elevation ref={node_ref} />
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
    fn it_renders() {
        let host = document().create_element("div").unwrap();
        let props = Props {
            customizable: CustomizableProps::default(),
        };

        yew::Renderer::<Elevation>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("<md-elevation"));
    }

    #[wasm_bindgen_test]
    fn it_renders_with_custom_style_and_aria() {
        let host = document().create_element("div").unwrap();
        let mut aria = BTreeMap::new();
        aria.insert("aria-hidden".to_string(), "true".into());
        let props = Props {
            customizable: CustomizableProps {
                style: Some("opacity: 0.8;".into()),
                aria: Some(aria),
            },
        };

        yew::Renderer::<Elevation>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("style=\"opacity: 0.8;\""));
        assert!(rendered_html.contains("aria-hidden=\"true\""));
    }
}