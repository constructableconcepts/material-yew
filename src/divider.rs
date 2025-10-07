use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    /// Indents the divider with equal padding on both sides.
    #[prop_or_default]
    pub inset: bool,
    /// Indents the divider with padding on the leading side.
    #[prop_or_default]
    pub inset_start: bool,
    /// Indents the divider with padding on the trailing side.
    #[prop_or_default]
    pub inset_end: bool,
}

#[function_component(Divider)]
pub fn divider(props: &Props) -> Html {
    crate::import_material_web_module!("/md-web/divider.js");
    html! {
        <md-divider
            inset={props.inset.then(|| AttrValue::from(""))}
            inset-start={props.inset_start.then(|| AttrValue::from(""))}
            inset-end={props.inset_end.then(|| AttrValue::from(""))}
        />
    }
}
