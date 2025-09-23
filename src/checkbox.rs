use js_sys::Reflect;
use wasm_bindgen::JsValue;
use web_sys::{HtmlFormElement as HTMLFormElement, NodeList};
use yew::prelude::*;

type ValidityState = JsValue;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    /// Whether or not the checkbox is selected.
    #[prop_or_default]
    pub checked: Option<bool>,
    /// Whether or not the checkbox is disabled.
    #[prop_or_default]
    pub disabled: bool,
    /// Whether or not the checkbox is indeterminate.<br>https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/checkbox#indeterminate_state_checkboxes
    #[prop_or_default]
    pub indeterminate: Option<bool>,
    /// When true, require the checkbox to be selected when participating in form submission.<br>https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/checkbox#validation
    #[prop_or_default]
    pub required: bool,
    /// The value of the checkbox that is submitted with a form when selected.<br>https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/checkbox#value
    #[prop_or(Some(AttrValue::Static("on")))]
    pub value: Option<AttrValue>,
    ///
    #[prop_or_default]
    pub name: Option<AttrValue>,
    ///
    #[prop_or_default]
    pub form: Option<HTMLFormElement>,
    ///
    #[prop_or_default]
    pub labels: Option<NodeList>,
    ///
    #[prop_or_default]
    pub validitype: Option<ValidityState>,
    ///
    #[prop_or_default]
    pub validation_message: Option<AttrValue>,
    ///
    #[prop_or_default]
    pub will_validate: Option<bool>,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
}

#[function_component]
pub fn Checkbox(props: &Props) -> Html {
    let node_ref = use_node_ref();

    {
        let node_ref = node_ref.clone();
        let props = props.clone();
        use_effect_with(props, move |props| {
            let element = node_ref.get().unwrap();
            let form_value = props.form.as_ref().map(|f| f.into()).unwrap_or(JsValue::NULL);
            Reflect::set(&element, &"form".into(), &form_value).unwrap();

            let labels_value = props.labels.as_ref().map(|l| l.into()).unwrap_or(JsValue::NULL);
            Reflect::set(&element, &"labels".into(), &labels_value).unwrap();

            let validity_value = props.validitype.as_ref().map(|v| v.into()).unwrap_or(JsValue::NULL);
            Reflect::set(&element, &"validity".into(), &validity_value).unwrap();

            let validation_message_value = props.validation_message.as_ref().map(|m| m.as_str().into()).unwrap_or(JsValue::NULL);
            Reflect::set(&element, &"validationMessage".into(), &validation_message_value).unwrap();

            let will_validate_value = props.will_validate.map(|v| v.into()).unwrap_or(JsValue::NULL);
            Reflect::set(&element, &"willValidate".into(), &will_validate_value).unwrap();

            move || {}
        });
    }

    crate::import_material_web_module!("/md-web/checkbox.js");
    html! { <md-checkbox
        ref={node_ref}
        checked={props.checked.filter(|&v| v).map(|_| AttrValue::from(""))}
        disabled={props.disabled}
        indeterminate={props.indeterminate.filter(|&v| v).map(|_| AttrValue::from(""))}
        required={props.required}
        value={props.value.clone()}
        name={props.name.clone()}
        onclick={props.onclick.clone()}
    /> }
}
