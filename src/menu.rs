use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::EventTarget;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    /// The ID of the element in the same root node in which the menu should align to. Overrides
    /// setting `anchorElement = elementReference`.<br><strong>NOTE</strong>: anchor or
    /// anchorElement must either be an HTMLElement or resolve to an HTMLElement in order for menu
    /// to open.
    #[prop_or_default]
    pub anchor: AttrValue,
    /// Whether the positioning algorithm should calculate relative to the parent of the anchor
    /// element (absolute) or relative to the window (fixed).
    #[prop_or(AttrValue::from("absolute"))]
    pub positioning: AttrValue,
    /// Skips the opening and closing animations.
    #[prop_or_default]
    pub quick: bool,
    /// Displays overflow content like a submenu.
    #[prop_or_default]
    pub has_overflow: bool,
    /// Opens the menu and makes it visible.
    #[prop_or_default]
    pub open: bool,
    /// Offsets the menu’s inline alignment from the anchor by the given number in pixels.
    #[prop_or_default]
    pub x_offset: i32,
    /// Offsets the menu’s block alignment from the anchor by the given number in pixels.
    #[prop_or_default]
    pub y_offset: i32,
    /// The max time between the keystrokes of the typeahead menu behavior before it clears the
    /// typeahead buffer.
    #[prop_or(200)]
    pub typeahead_delay: u32,
    /// The corner of the anchor which to align the menu.
    #[prop_or(AttrValue::from("end-start"))]
    pub anchor_corner: AttrValue,
    /// The corner of the menu which to align the anchor.
    #[prop_or(AttrValue::from("start-start"))]
    pub menu_corner: AttrValue,
    /// Keeps the user clicks outside the menu.
    #[prop_or_default]
    pub stay_open_on_outside_click: bool,
    /// Keeps the menu open when focus leaves the menu’s composed subtree.
    #[prop_or_default]
    pub stay_open_on_focusout: bool,
    /// After closing, does not restore focus to the last focused element before the menu was
    /// opened.
    #[prop_or_default]
    pub skip_restore_focus: bool,
    /// The element that should be focused by default once opened.
    #[prop_or(AttrValue::from("first-item"))]
    pub default_focus: AttrValue,
    /// Whether or not the current menu is a submenu.
    #[prop_or_default]
    pub is_submenu: bool,
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
pub fn Menu(props: &Props) -> Html {
    let node_ref = use_node_ref();

    // The event handling here is verbose and could be improved with a macro,
    // but for now, we'll leave it as-is to focus on the prop ergonomics task.
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

    crate::import_material_web_module!("/md-web/menu.js");
    html! { <md-menu
       ref={node_ref}
       anchor={props.anchor.clone()}
       positioning={props.positioning.clone()}
       quick={props.quick.then_some(AttrValue::from(""))}
       has-overflow={props.has_overflow.then_some(AttrValue::from(""))}
       open={props.open}
       x-offset={props.x_offset.to_string()}
       y-offset={props.y_offset.to_string()}
       typeahead-delay={props.typeahead_delay.to_string()}
       anchor-corner={props.anchor_corner.clone()}
       menu-corner={props.menu_corner.clone()}
       stay-open-on-outside-click={props.stay_open_on_outside_click.then_some(AttrValue::from(""))}
       stay-open-on-focusout={props.stay_open_on_focusout.then_some(AttrValue::from(""))}
       skip-restore-focus={props.skip_restore_focus.then_some(AttrValue::from(""))}
       default-focus={props.default_focus.clone()}
       is-submenu={props.is_submenu.then_some(AttrValue::from(""))}
    >
        {props.children.clone()}
    </md-menu> }
}