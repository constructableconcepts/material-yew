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
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
}

#[function_component]
pub fn Checkbox(props: &Props) -> Html {
    let node_ref = use_node_ref();

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
