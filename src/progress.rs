use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Default)]
pub struct Props {
    #[prop_or_default]
    pub progress: f32,
    #[prop_or_default]
    pub indeterminate: bool,
    #[prop_or_default]
    pub four_color: bool,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[function_component(Progress)]
pub fn progress(props: &Props) -> Html {
    html! {
        <md-progress
            progress={props.progress.to_string()}
            indeterminate={props.indeterminate.then_some(AttrValue::from(""))}
            four-color={props.four_color.then_some(AttrValue::from(""))}
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
    fn it_renders_with_custom_style_and_id() {
        let host = document().create_element("div").unwrap();
        let props = Props {
            progress: 0.5,
            indeterminate: false,
            four_color: false,
            id: Some("custom-id".into()),
            style: Some("width: 200px;".into()),
        };

        yew::Renderer::<Progress>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("id=\"custom-id\""));
        assert!(rendered_html.contains("style=\"width: 200px;\""));
    }
}