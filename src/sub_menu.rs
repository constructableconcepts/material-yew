use yew::prelude::*;
#[derive(Properties, PartialEq)]
pub struct Props {
    /// The anchorCorner to set on the submenu.
    #[prop_or_default]
    pub anchor_corner: Option<AttrValue>,
    /// The menuCorner to set on the submenu.
    #[prop_or_default]
    pub menu_corner: Option<AttrValue>,
    /// The delay between mouseenter and submenu opening.
    #[prop_or(400)]
    pub hover_open_delay: usize,
    /// The delay between ponterleave and the submenu closing.
    #[prop_or(400)]
    pub hover_close_delay: usize,
    /// READONLY: self-identifies as a menu item and sets its identifying attribute
    #[prop_or(true)]
    pub is_sub_menu: bool,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SubMenu(props: &Props) -> Html {
    crate::import_material_web_module!("/md-web/sub-menu.js");
    html! { <md-sub-menu
       anchor-corner={props.anchor_corner.clone()}
       menu-corner={props.menu_corner.clone()}
       hover-open-delay={props.hover_open_delay.to_string()}
       hover-close-delay={props.hover_close_delay.to_string()}
       is-sub-menu={props.is_sub_menu.then(|| AttrValue::from(""))}
    >
        {props.children.clone()}
    </md-sub-menu> }
}
