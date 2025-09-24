use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub inset: bool,
    #[prop_or_default]
    pub vertical: bool,
}

#[function_component(Divider)]
pub fn divider(props: &Props) -> Html {
    crate::import_material_web_module!("/md-web/divider.js");
    html! {
        <md-divider
            inset={props.inset.then(|| AttrValue::from(""))}
            vertical={props.vertical.then(|| AttrValue::from(""))}
        />
    }
}
