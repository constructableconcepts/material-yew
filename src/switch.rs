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
    /// When true, require the switch to be selected when participating in form submission.
    #[prop_or_default]
    pub required: bool,
    /// The value associated with this switch on form submission.
    #[prop_or_default]
    pub value: AttrValue,
    /// The name of the switch.
    #[prop_or_default]
    pub name: AttrValue,
    #[prop_or_default]
    pub oninput: Callback<InputEvent>,
    #[prop_or_default]
    pub onchange: Callback<Event>,
}

#[function_component]
pub fn Switch(props: &Props) -> Html {
    crate::import_material_web_module!("/md-web/switch.js");
    html! { <md-switch
        disabled={props.disabled}
        selected={props.selected}
        icons={props.icons.then_some(AttrValue::from(""))}
        show-only-selected-icon={props.show_only_selected_icon.then_some(AttrValue::from(""))}
        required={props.required}
        value={props.value.clone()}
        name={props.name.clone()}
        oninput={props.oninput.clone()}
        onchange={props.onchange.clone()}
    />}
}