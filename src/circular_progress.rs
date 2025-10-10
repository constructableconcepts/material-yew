use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    /// Progress to display, a fraction between 0 and <code>max</code>.
    #[prop_or_default]
    pub value: f32,
    /// Maximum progress to display, defaults to 1.
    #[prop_or(1.0)]
    pub max: f32,
    /// Whether or not to display indeterminate progress, which gives no indication to how long an
    /// activity will take.
    #[prop_or_default]
    pub indeterminate: bool,
    /// Whether or not to render indeterminate mode using 4 colors instead of one.
    #[prop_or_default]
    pub four_color: bool,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[function_component]
pub fn CircularProgress(props: &Props) -> Html {
    crate::import_material_web_module!("/md-web/circular-progress.js");

    html! { <md-circular-progress
        value={props.value.to_string()}
        max={props.max.to_string()}
        indeterminate={props.indeterminate.then_some(AttrValue::from(""))}
        four-color={props.four_color.then_some(AttrValue::from(""))}
        id={props.id.clone()}
        style={props.style.clone()}
    /> }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gloo_utils::document;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn it_renders_with_f32() {
        let host = document().create_element("div").unwrap();
        let props = Props {
            value: 0.5,
            max: 2.0,
            indeterminate: false,
            four_color: false,
            id: None,
            style: None,
        };

        yew::Renderer::<CircularProgress>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("value=\"0.5\""));
        assert!(rendered_html.contains("max=\"2\""));
    }

    #[wasm_bindgen_test]
    fn it_renders_with_custom_style_and_id() {
        let host = document().create_element("div").unwrap();
        let props = Props {
            value: 0.5,
            max: 2.0,
            indeterminate: false,
            four_color: false,
            id: Some("custom-id".into()),
            style: Some("width: 100px;".into()),
        };

        yew::Renderer::<CircularProgress>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("id=\"custom-id\""));
        assert!(rendered_html.contains("style=\"width: 100px;\""));
    }
}