use yew::prelude::*;
#[derive(Properties, PartialEq)]
pub struct Props {
    /// Disables the item and makes it non-selectable and non-interactive.
    #[prop_or_default]
    pub disabled: bool,
    /// Sets the behavior of the list item, defaults to “text”. Change to “link” or “button” for
    /// interactive items.
    #[prop_or_default]
    pub typepe: Option<AttrValue>,
    /// Sets the underlying <code>HTMLAnchorElement</code>’s <code>href</code> resource attribute.
    #[prop_or_default]
    pub href: Option<AttrValue>,
    /// Sets the underlying <code>HTMLAnchorElement</code>’s <code>target</code> attribute when
    /// <code>href</code> is set.
    #[prop_or_default]
    pub target: Option<AttrValue>,
    #[prop_or_default]
    pub children: Html,

    #[prop_or_default]
    pub onfocus: Option<Callback<FocusEvent>>,
}

#[function_component]
pub fn ListItem(props: &Props) -> Html {
    crate::import_material_web_module!("/md-web/list-item.js");
    html! { <md-list-item
        disabled={props.disabled}
        type={props.typepe.clone()}
        href={props.href.clone()}
        target={props.target.clone()}
        onfocus={props.onfocus.clone()}
    > {props.children.clone()} </md-list-item> }
}
