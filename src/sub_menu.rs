use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::EventTarget;
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
    pub onclosing: Callback<Event>,
    #[prop_or_default]
    pub onopening: Callback<Event>,
    #[prop_or_default]
    pub onopened: Callback<Event>,
    #[prop_or_default]
    pub onclosed: Callback<Event>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SubMenu(props: &Props) -> Html {
    let node_ref = use_node_ref();
    {
        let node_ref = node_ref.clone();
        let onclosing = props.onclosing.clone();
        use_effect_with(onclosing, move |onclosing_ref| {
            let onclosing = onclosing_ref.clone();
            let element = node_ref.get().unwrap();
            let target = element.dyn_ref::<EventTarget>().unwrap().clone();
            let listener = Closure::<dyn FnMut(_)>::new(move |event| onclosing.emit(event));
            target
                .add_event_listener_with_callback("closing", listener.as_ref().unchecked_ref())
                .unwrap();
            move || {
                target
                    .remove_event_listener_with_callback("closing", listener.as_ref().unchecked_ref())
                    .unwrap();
            }
        });
    }
    {
        let node_ref = node_ref.clone();
        let onopening = props.onopening.clone();
        use_effect_with(onopening, move |onopening_ref| {
            let onopening = onopening_ref.clone();
            let element = node_ref.get().unwrap();
            let target = element.dyn_ref::<EventTarget>().unwrap().clone();
            let listener = Closure::<dyn FnMut(_)>::new(move |event| onopening.emit(event));
            target
                .add_event_listener_with_callback("opening", listener.as_ref().unchecked_ref())
                .unwrap();
            move || {
                target
                    .remove_event_listener_with_callback("opening", listener.as_ref().unchecked_ref())
                    .unwrap();
            }
        });
    }
    {
        let node_ref = node_ref.clone();
        let onopened = props.onopened.clone();
        use_effect_with(onopened, move |onopened_ref| {
            let onopened = onopened_ref.clone();
            let element = node_ref.get().unwrap();
            let target = element.dyn_ref::<EventTarget>().unwrap().clone();
            let listener = Closure::<dyn FnMut(_)>::new(move |event| onopened.emit(event));
            target
                .add_event_listener_with_callback("opened", listener.as_ref().unchecked_ref())
                .unwrap();
            move || {
                target
                    .remove_event_listener_with_callback("opened", listener.as_ref().unchecked_ref())
                    .unwrap();
            }
        });
    }
    {
        let node_ref = node_ref.clone();
        let onclosed = props.onclosed.clone();
        use_effect_with(onclosed, move |onclosed_ref| {
            let onclosed = onclosed_ref.clone();
            let element = node_ref.get().unwrap();
            let target = element.dyn_ref::<EventTarget>().unwrap().clone();
            let listener = Closure::<dyn FnMut(_)>::new(move |event| onclosed.emit(event));
            target
                .add_event_listener_with_callback("closed", listener.as_ref().unchecked_ref())
                .unwrap();
            move || {
                target
                    .remove_event_listener_with_callback("closed", listener.as_ref().unchecked_ref())
                    .unwrap();
            }
        });
    }

    crate::import_material_web_module!("/md-web/sub-menu.js");
    html! { <md-sub-menu
        ref={node_ref}
       anchor-corner={props.anchor_corner.clone()}
       menu-corner={props.menu_corner.clone()}
       hover-open-delay={props.hover_open_delay.to_string()}
       hover-close-delay={props.hover_close_delay.to_string()}
       is-sub-menu={props.is_sub_menu.then(|| AttrValue::from(""))}
    >
        {props.children.clone()}
    </md-sub-menu> }
}
