use yew::prelude::*;

#[derive(Properties, PartialEq, Default)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[function_component]
pub fn List(props: &Props) -> Html {
    crate::import_material_web_module!("/md-web/list.js");

    html! {
        <md-list id={props.id.clone()} style={props.style.clone()}>
            { for props.children.iter() }
        </md-list>
    }
}