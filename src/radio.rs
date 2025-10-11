use crate::form_element::FormElementRef;
use yew::prelude::*;

/// A handle to imperatively control the Radio component.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RadioRef {
    form_element_ref: FormElementRef,
}

impl RadioRef {
    /// Checks the radio's validity.
    pub fn check_validity(&self) -> bool {
        self.form_element_ref.check_validity()
    }

    /// Checks the radio's validity and reports it to the user.
    pub fn report_validity(&self) -> bool {
        self.form_element_ref.report_validity()
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    /// Whether or not the radio is disabled.
    #[prop_or_default]
    pub disabled: bool,
    /// The element value to use in form submission when checked.
    #[prop_or_default]
    pub value: AttrValue,
    /// Whether or not the radio is checked.
    #[prop_or_default]
    pub checked: bool,
    /// The name of the radio group.
    #[prop_or_default]
    pub name: AttrValue,
    /// A handle to allow imperative control of the radio.
    #[prop_or_default]
    pub radio_ref: RadioRef,
    #[prop_or_default]
    pub oninput: Callback<InputEvent>,
    #[prop_or_default]
    pub onchange: Callback<Event>,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[function_component]
pub fn Radio(props: &Props) -> Html {
    let node_ref = props.radio_ref.form_element_ref.node_ref.clone();
    crate::import_material_web_module!("/md-web/radio.js");

    html! { <md-radio
        ref={node_ref}
        disabled={props.disabled}
        value={props.value.clone()}
        checked={props.checked.then_some(AttrValue::from(""))}
        name={props.name.clone()}
        oninput={props.oninput.clone()}
        onchange={props.onchange.clone()}
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
    fn it_renders_with_custom_style_and_id() {
        let host = document().create_element("div").unwrap();
        let props = Props {
            disabled: false,
            value: "value".into(),
            checked: false,
            name: "group".into(),
            radio_ref: RadioRef::default(),
            oninput: Callback::default(),
            onchange: Callback::default(),
            id: Some("custom-id".into()),
            style: Some("color: red;".into()),
        };

        yew::Renderer::<Radio>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("id=\"custom-id\""));
        assert!(rendered_html.contains("style=\"color: red;\""));
    }

    #[wasm_bindgen_test]
    fn it_handles_validation() {
        let radio_ref = RadioRef::default();
        let host = document().create_element("div").unwrap();
        let props = Props {
            disabled: false,
            value: "value".into(),
            checked: false,
            name: "group".into(),
            radio_ref: radio_ref.clone(),
            oninput: Callback::default(),
            onchange: Callback::default(),
            id: None,
            style: None,
        };

        yew::Renderer::<Radio>::with_root_and_props(host.clone(), props).render();

        assert!(radio_ref.check_validity());
    }
}