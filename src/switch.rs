use yew::prelude::*;

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
    /// When true, require the switch to be selected when participating in form submission.<br>https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/checkbox#validation
    #[prop_or_default]
    pub required: bool,
    /// The value associated with this switch on form submission. <code>null</code> is submitted
    /// when <code>selected</code> is <code>false</code>.
    #[prop_or_default]
    pub value: AttrValue,
    ///
    #[prop_or_default]
    pub name: Option<AttrValue>,
    ///
    #[prop_or_default]
    pub validation_message: Option<AttrValue>,
    ///
    #[prop_or_default]
    pub will_validate: bool,
    #[prop_or_default]
    pub oninput: Option<Callback<InputEvent>>,
    #[prop_or_default]
    pub onchange: Option<Callback<Event>>,
}

#[function_component]
pub fn Switch(props: &Props) -> Html {
    crate::import_material_web_module!("/md-web/switch.js");
    html! { <md-switch
        disabled={props.disabled}
        selected={props.selected}
        icons={props.icons.then(|| AttrValue::from(""))}
        show-only-selected-icon={props.show_only_selected_icon.then(|| AttrValue::from(""))}
        required={props.required}
        value={props.value.clone()}
        name={props.name.clone()}
        validation-message={props.validation_message.clone()}
        will-validate={props.will_validate.then(|| AttrValue::from(""))}
        oninput={props.oninput.clone()}
        onchange={props.onchange.clone()}
    />}
}
