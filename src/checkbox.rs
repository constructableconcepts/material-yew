use yew::prelude::*;

/// Properties for the `Checkbox` component.
#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    /// Whether or not the checkbox is selected.
    #[prop_or_default]
    pub checked: Option<bool>,
    /// Whether or not the checkbox is disabled.
    #[prop_or_default]
    pub disabled: bool,
    /// Whether or not the checkbox is indeterminate.
    #[prop_or_default]
    pub indeterminate: Option<bool>,
    /// When true, require the checkbox to be selected when participating in form submission.
    #[prop_or_default]
    pub required: bool,
    /// The value of the checkbox that is submitted with a form when selected.
    #[prop_or(Some(AttrValue::Static("on")))]
    pub value: Option<AttrValue>,
    /// The name of the checkbox.
    #[prop_or_default]
    pub name: Option<AttrValue>,
    /// A `NodeRef` to allow the parent component to directly access the underlying `md-checkbox` element.
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
}

/// A checkbox component.
///
/// [Material Design spec](https://m3.material.io/components/checkbox/overview)
#[function_component(Checkbox)]
pub fn checkbox(props: &Props) -> Html {
    crate::import_material_web_module!("/md-web/checkbox.js");
    html! { <md-checkbox
        ref={props.node_ref.clone()}
        checked={props.checked.filter(|&v| v).map(|_| AttrValue::from(""))}
        disabled={props.disabled}
        indeterminate={props.indeterminate.filter(|&v| v).map(|_| AttrValue::from(""))}
        required={props.required}
        value={props.value.clone()}
        name={props.name.clone()}
        onclick={props.onclick.clone()}
    /> }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gloo_utils::document;
    use wasm_bindgen_test::*;
    use yew::prelude::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn it_renders_correctly() {
        let host = document().create_element("div").unwrap();
        let props = Props {
            checked: Some(true),
            disabled: true,
            indeterminate: None,
            required: false,
            value: Some("test-value".into()),
            name: Some("test-name".into()),
            node_ref: NodeRef::default(),
            onclick: None,
        };

        yew::Renderer::<Checkbox>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("checked"));
        assert!(rendered_html.contains("disabled"));
        assert!(rendered_html.contains("value=\"test-value\""));
        assert!(rendered_html.contains("name=\"test-name\""));
    }
}