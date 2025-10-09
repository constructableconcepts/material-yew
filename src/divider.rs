use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    /// Increases the leading space of the divider.
    #[prop_or_default]
    pub inset_start: bool,
    /// Increases the trailing space of the divider.
    #[prop_or_default]
    pub inset_end: bool,
}

#[function_component(Divider)]
pub fn divider(props: &Props) -> Html {
    crate::import_material_web_module!("/md-web/divider.js");
    html! {
        <md-divider
            inset-start={props.inset_start.then(|| AttrValue::from(""))}
            inset-end={props.inset_end.then(|| AttrValue::from(""))}
        />
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gloo_utils::document;
    use wasm_bindgen_test::*;
    use yew::prelude::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn it_renders_with_insets() {
        let host = document().create_element("div").unwrap();
        let props = Props {
            inset_start: true,
            inset_end: true,
        };

        yew::Renderer::<Divider>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("inset-start"));
        assert!(rendered_html.contains("inset-end"));
    }
}
