use yew::prelude::*;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub label: Option<String>,
    #[prop_or_default]
    pub value: Option<String>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub outlined: bool,
    #[prop_or_default]
    pub oninput: Option<Callback<String>>,
}

#[function_component(TextField)]
pub fn textfield(props: &Props) -> Html {
    crate::import_material_web_module!("/md-web/textfield.js");
    let oninput = props.oninput.clone();
    let oninput_cb = Callback::from(move |e: InputEvent| {
        if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
            if let Some(cb) = &oninput {
                cb.emit(input.value());
            }
        }
    });

    html! {
        <md-text-field
            label={props.label.clone().unwrap_or_default()}
            value={props.value.clone().unwrap_or_default()}
            disabled={props.disabled}
            outlined={props.outlined.to_string()}
            oninput={oninput_cb}
        />
    }
}
