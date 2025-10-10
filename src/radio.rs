use yew::prelude::*;

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
    #[prop_or_default]
    pub oninput: Callback<InputEvent>,
    #[prop_or_default]
    pub onchange: Callback<Event>,
}

#[function_component]
pub fn Radio(props: &Props) -> Html {
    crate::import_material_web_module!("/md-web/radio.js");
    html! { <md-radio
        disabled={props.disabled}
        value={props.value.clone()}
        checked={props.checked.then_some(AttrValue::from(""))}
        name={props.name.clone()}
        oninput={props.oninput.clone()}
        onchange={props.onchange.clone()}
    /> }
}