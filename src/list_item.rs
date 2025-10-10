use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    /// Disables the item and makes it non-selectable and non-interactive.
    #[prop_or_default]
    pub disabled: bool,
    /// Sets the behavior of the list item, defaults to “text”. Change to “link” or “button” for
    /// interactive items.
    #[prop_or(AttrValue::from("text"))]
    pub r#type: AttrValue,
    /// Sets the underlying <code>HTMLAnchorElement</code>’s <code>href</code> resource attribute.
    #[prop_or_default]
    pub href: AttrValue,
    /// Sets the underlying <code>HTMLAnchorElement</code>’s <code>target</code> attribute when
    /// <code>href</code> is set.
    #[prop_or_default]
    pub target: AttrValue,
    #[prop_or_default]
    pub children: Html,

    #[prop_or_default]
    pub onfocus: Callback<FocusEvent>,
}

#[function_component]
pub fn ListItem(props: &Props) -> Html {
    crate::import_material_web_module!("/md-web/list-item.js");
    html! { <md-list-item
        disabled={props.disabled}
        type={props.r#type.clone()}
        href={props.href.clone()}
        target={props.target.clone()}
        onfocus={props.onfocus.clone()}
    > {props.children.clone()} </md-list-item> }
}