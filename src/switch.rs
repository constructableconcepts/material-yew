use crate::form_element::FormElementRef;
use yew::prelude::*;

/// A handle to imperatively control the Switch component.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SwitchRef {
    form_element_ref: FormElementRef,
}

impl SwitchRef {
    /// Checks the switch's validity.
    pub fn check_validity(&self) -> bool {
        self.form_element_ref.check_validity()
    }

    /// Checks the switch's validity and reports it to the user.
    pub fn report_validity(&self) -> bool {
        self.form_element_ref.report_validity()
    }
}

/// Properties for the `Switch` component.
#[derive(Properties, PartialEq)]
pub struct Props {
    /// Disables the switch and makes it non-interactive.
    #[prop_or_default]
    pub disabled: bool,
    /// Puts the switch in the selected state and sets the form submission value to the
    /// <code>value</code> property.
    #[prop_or_default]
    pub selected: bool,
    /// Shows both the selected and deselected icons.
    #[prop_or_default]
    pub icons: bool,
    /// Shows only the selected icon, and not the deselected icon. If <code>true</code>, overrides
    /// the behavior of the <code>icons</code> property.
    #[prop_or_default]
    pub show_only_selected_icon: bool,
    /// When true, require the switch to be selected when participating in form submission.
    #[prop_or_default]
    pub required: bool,
    /// The value associated with this switch on form submission.
    #[prop_or_default]
    pub value: AttrValue,
    /// The name of the switch.
    #[prop_or_default]
    pub name: AttrValue,
    /// A handle to allow imperative control of the switch.
    #[prop_or_default]
    pub switch_ref: SwitchRef,
    /// The callback to be called when the switch receives input.
    #[prop_or_default]
    pub oninput: Callback<InputEvent>,
    /// The callback to be called when the switch's value changes.
    #[prop_or_default]
    pub onchange: Callback<Event>,
    /// The id of the switch.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// The style of the switch.
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// A switch component.
///
/// [Material Design spec](https://m3.material.io/components/switch/overview)
#[function_component]
pub fn Switch(props: &Props) -> Html {
    let node_ref = props.switch_ref.form_element_ref.node_ref.clone();
    crate::import_material_web_module!("/md-web/switch.js");

    html! { <md-switch
        ref={node_ref}
        disabled={props.disabled}
        selected={props.selected}
        icons={props.icons.then_some(AttrValue::from(""))}
        show-only-selected-icon={props.show_only_selected_icon.then_some(AttrValue::from(""))}
        required={props.required}
        value={props.value.clone()}
        name={props.name.clone()}
        oninput={props.oninput.clone()}
        onchange={props.onchange.clone()}
        id={props.id.clone()}
        style={props.style.clone()}
    />}
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
            selected: false,
            icons: false,
            show_only_selected_icon: false,
            required: false,
            value: AttrValue::default(),
            name: AttrValue::default(),
            switch_ref: SwitchRef::default(),
            oninput: Callback::default(),
            onchange: Callback::default(),
            id: Some("custom-id".into()),
            style: Some("color: hotpink;".into()),
        };

        yew::Renderer::<Switch>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("id=\"custom-id\""));
        assert!(rendered_html.contains("style=\"color: hotpink;\""));
    }

    #[wasm_bindgen_test]
    fn it_handles_validation() {
        let switch_ref = SwitchRef::default();
        let host = document().create_element("div").unwrap();
        let props = Props {
            disabled: false,
            selected: false,
            icons: false,
            show_only_selected_icon: false,
            required: true,
            value: AttrValue::default(),
            name: AttrValue::default(),
            switch_ref: switch_ref.clone(),
            oninput: Callback::default(),
            onchange: Callback::default(),
            id: None,
            style: None,
        };

        yew::Renderer::<Switch>::with_root_and_props(host.clone(), props).render();

        assert!(!switch_ref.check_validity());
    }
}