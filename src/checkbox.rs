use wasm_bindgen::JsValue;
use web_sys::{HtmlFormElement as HTMLFormElement, NodeList};
use yew::prelude::*;

type ValidityState = JsValue;

#[derive(Properties, PartialEq)]
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
    crate::import_material_web_module!("/md-web/checkbox.js");
    html! { <md-checkbox
        checked={props.checked.filter(|&v| v).map(|_| AttrValue::from(""))}
        disabled={props.disabled}
        indeterminate={props.indeterminate.filter(|&v| v).map(|_| AttrValue::from(""))}
        required={props.required}
        value={props.value.clone()}
        name={props.name.clone()}
        // ~form={crate::js_value_or_null(props.form.clone())}
        // ~labels={crate::js_value_or_null(props.labels.clone())}
        // ~validity={crate::js_value_or_null(props.validitype.clone())}
        // ~validationMessage={crate::js_value_from_string_or_null(props.validation_message.as_ref())}
        // ~willValidate={crate::js_value_or_null(props.will_validate.clone())}
        onclick={props.onclick.clone()}
    /> }
}
