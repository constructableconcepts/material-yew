use crate::form_element::FormElementRef;
use yew::prelude::*;

/// A handle to imperatively control the TextField component.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TextFieldRef {
    form_element_ref: FormElementRef,
}

impl TextFieldRef {
    /// Checks the textfield's validity.
    pub fn check_validity(&self) -> bool {
        self.form_element_ref.check_validity()
    }

    /// Checks the textfield's validity and reports it to the user.
    pub fn report_validity(&self) -> bool {
        self.form_element_ref.report_validity()
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub label: AttrValue,
    #[prop_or_default]
    pub value: AttrValue,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub outlined: bool,
    /// A handle to allow imperative control of the textfield.
    #[prop_or_default]
    pub textfield_ref: TextFieldRef,
    #[prop_or_default]
    pub oninput: Callback<InputEvent>,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[function_component(TextField)]
pub fn textfield(props: &Props) -> Html {
    let node_ref = props.textfield_ref.form_element_ref.node_ref.clone();
    crate::import_material_web_module!("/md-web/textfield.js");

    let tag = if props.outlined {
        "md-outlined-text-field"
    } else {
        "md-filled-text-field"
    };

    html! {
        <@{tag}
            ref={node_ref}
            label={props.label.clone()}
            value={props.value.clone()}
            disabled={props.disabled}
            oninput={props.oninput.clone()}
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
            label: "Label".into(),
            value: "Value".into(),
            disabled: false,
            outlined: false,
            textfield_ref: TextFieldRef::default(),
            oninput: Callback::default(),
            id: Some("custom-id".into()),
            style: Some("color: blue;".into()),
        };

        yew::Renderer::<TextField>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("id=\"custom-id\""));
        assert!(rendered_html.contains("style=\"color: blue;\""));
    }

    #[wasm_bindgen_test]
    fn it_handles_validation() {
        let textfield_ref = TextFieldRef::default();
        let host = document().create_element("div").unwrap();
        let props = Props {
            label: "Label".into(),
            value: "Value".into(),
            disabled: false,
            outlined: false,
            textfield_ref: textfield_ref.clone(),
            oninput: Callback::default(),
            id: None,
            style: None,
        };

        yew::Renderer::<TextField>::with_root_and_props(host.clone(), props).render();

        assert!(textfield_ref.check_validity());
    }
}