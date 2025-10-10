use wasm_bindgen::{prelude::*, JsCast};
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    type MdCheckbox;

    #[wasm_bindgen(method)]
    fn checkValidity(this: &MdCheckbox) -> bool;

    #[wasm_bindgen(method)]
    fn reportValidity(this: &MdCheckbox) -> bool;
}

/// A handle to imperatively control the Checkbox component.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CheckboxRef {
    node_ref: NodeRef,
}

impl CheckboxRef {
    /// Checks the checkbox's validity.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/checkValidity
    pub fn check_validity(&self) -> bool {
        if let Some(element) = self.node_ref.get() {
            let checkbox: &MdCheckbox = element.unchecked_ref();
            return checkbox.checkValidity();
        }
        false
    }

    /// Checks the checkbox's validity and reports it to the user.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/reportValidity
    pub fn report_validity(&self) -> bool {
        if let Some(element) = self.node_ref.get() {
            let checkbox: &MdCheckbox = element.unchecked_ref();
            return checkbox.reportValidity();
        }
        false
    }
}

/// Properties for the `Checkbox` component.
#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    /// Whether or not the checkbox is selected.
    #[prop_or_default]
    pub checked: bool,
    /// Whether or not the checkbox is disabled.
    #[prop_or_default]
    pub disabled: bool,
    /// Whether or not the checkbox is indeterminate.
    #[prop_or_default]
    pub indeterminate: bool,
    /// When true, require the checkbox to be selected when participating in form submission.
    #[prop_or_default]
    pub required: bool,
    /// The value of the checkbox that is submitted with a form when selected.
    #[prop_or(AttrValue::from("on"))]
    pub value: AttrValue,
    /// The name of the checkbox.
    #[prop_or_default]
    pub name: AttrValue,
    /// A handle to allow imperative control of the checkbox.
    #[prop_or_default]
    pub checkbox_ref: CheckboxRef,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// A checkbox component.
///
/// [Material Design spec](https://m3.material.io/components/checkbox/overview)
#[function_component(Checkbox)]
pub fn checkbox(props: &Props) -> Html {
    let node_ref = props.checkbox_ref.node_ref.clone();
    crate::import_material_web_module!("/md-web/checkbox.js");

    html! { <md-checkbox
        ref={node_ref}
        checked={props.checked.then_some(AttrValue::from(""))}
        disabled={props.disabled}
        indeterminate={props.indeterminate.then_some(AttrValue::from(""))}
        required={props.required}
        value={props.value.clone()}
        name={props.name.clone()}
        onclick={props.onclick.clone()}
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
    fn it_renders_correctly() {
        let host = document().create_element("div").unwrap();
        let props = Props {
            checked: true,
            disabled: true,
            indeterminate: false,
            required: false,
            value: "test-value".into(),
            name: "test-name".into(),
            checkbox_ref: CheckboxRef::default(),
            onclick: Callback::default(),
            id: None,
            style: None,
        };

        yew::Renderer::<Checkbox>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("checked"));
        assert!(rendered_html.contains("disabled"));
        assert!(rendered_html.contains("value=\"test-value\""));
        assert!(rendered_html.contains("name=\"test-name\""));
    }

    #[wasm_bindgen_test]
    fn it_renders_with_custom_style_and_id() {
        let host = document().create_element("div").unwrap();
        let props = Props {
            checked: false,
            disabled: false,
            indeterminate: false,
            required: false,
            value: "on".into(),
            name: AttrValue::default(),
            checkbox_ref: CheckboxRef::default(),
            onclick: Callback::default(),
            id: Some("custom-id".into()),
            style: Some("color: blue;".into()),
        };

        yew::Renderer::<Checkbox>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("id=\"custom-id\""));
        assert!(rendered_html.contains("style=\"color: blue;\""));
    }
}