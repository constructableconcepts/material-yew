use yew::prelude::*;

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
    #[prop_or_default]
    pub oninput: Callback<InputEvent>,
}

#[function_component(TextField)]
pub fn textfield(props: &Props) -> Html {
    crate::import_material_web_module!("/md-web/textfield.js");

    let tag = if props.outlined {
        "md-outlined-text-field"
    } else {
        "md-filled-text-field"
    };
    html! {
        <@{tag}
            label={props.label.clone()}
            value={props.value.clone()}
            disabled={props.disabled}
            oninput={props.oninput.clone()}
        />
    }
}