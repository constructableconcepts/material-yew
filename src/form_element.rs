use wasm_bindgen::{prelude::*, JsCast};
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    type FormElement;

    #[wasm_bindgen(method)]
    fn checkValidity(this: &FormElement) -> bool;

    #[wasm_bindgen(method)]
    fn reportValidity(this: &FormElement) -> bool;
}

/// A handle to imperatively control a form element.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FormElementRef {
    pub node_ref: NodeRef,
}

impl FormElementRef {
    /// Checks the element's validity.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/checkValidity
    pub fn check_validity(&self) -> bool {
        if let Some(element) = self.node_ref.get() {
            let form_element: &FormElement = element.unchecked_ref();
            return form_element.checkValidity();
        }
        false
    }

    /// Checks the element's validity and reports it to the user.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/reportValidity
    pub fn report_validity(&self) -> bool {
        if let Some(element) = self.node_ref.get() {
            let form_element: &FormElement = element.unchecked_ref();
            return form_element.reportValidity();
        }
        false
    }
}