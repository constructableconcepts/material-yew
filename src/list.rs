use yew::prelude::*;

/// Properties for the `List` component.
#[derive(Properties, PartialEq, Default)]
pub struct Props {
    /// The items to display in the list.
    #[prop_or_default]
    pub children: Children,
    /// The id of the list.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// The style of the list.
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// A list component.
///
/// [Material Design spec](https://m3.material.io/components/lists/overview)
#[function_component]
pub fn List(props: &Props) -> Html {
    crate::import_material_web_module!("/md-web/list.js");

    html! {
        <md-list id={props.id.clone()} style={props.style.clone()}>
            { for props.children.iter() }
        </md-list>
    }
}