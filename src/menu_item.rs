use yew::prelude::*;
#[derive(Properties, PartialEq)]
pub struct Props {
    /// Disables the item and makes it non-selectable and non-interactive.
    #[prop_or_default]
    pub disabled: bool,
    /// Sets the behavior and role of the menu item, defaults to “menuitem”.
    #[prop_or_default]
    pub typepe: Option<AttrValue>,
    /// Sets the underlying <code>HTMLAnchorElement</code>’s <code>href</code> resource attribute.
    #[prop_or_default]
    pub href: Option<AttrValue>,
    /// Sets the underlying <code>HTMLAnchorElement</code>’s <code>target</code> attribute when
    /// <code>href</code> is set.
    #[prop_or_default]
    pub target: Option<AttrValue>,
    /// Keeps the menu open if clicked or keyboard selected.
    #[prop_or_default]
    pub keep_open: bool,
    /// Sets the item in the selected visual state when a submenu is opened.
    #[prop_or_default]
    pub selected: bool,
    ///
    #[prop_or_default]
    pub typepeahead_text: Option<AttrValue>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn MenuItem(props: &Props) -> Html {
    crate::import_material_web_module!("/md-web/menu-item.js");
    html! { <md-menu-item
       disabled={props.disabled}
       type={props.typepe.clone()}
       href={props.href.clone()}
       target={props.target.clone()}
       keep-open={props.keep_open.then(|| AttrValue::from(""))}
       selected={props.selected}
       typeahead-text={props.typepeahead_text.clone()}
    >
        {props.children.clone()}
    </md-menu-item> }
}
