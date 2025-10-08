use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::EventTarget;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    /// The ID of the element in the same root node in which the menu should align to. Overrides
    /// setting <code>anchorElement = elementReference</code>.<br><strong>NOTE</strong>: anchor or
    /// anchorElement must either be an HTMLElement or resolve to an HTMLElement in order for menu
    /// to open.
    #[prop_or_default]
    pub anchor: Option<AttrValue>,
    /// Whether the positioning algorithm should calculate relative to the parent of the anchor
    /// element (absolute) or relative to the window (fixed).<br>Examples for <code>position =
    /// 'fixed'</code>:<br>- If there is no <code>position:relative</code> in the given parent tree
    /// and the surface is <code>position:absolute</code> - If the surface is
    /// <code>position:fixed</code> - If the surface is in the “top layer” - The anchor and the
    /// surface do not share a common <code>position:relative</code> ancestor<br>When using
    /// positioning = fixed, in most cases, the menu should position itself above most other
    /// <code>position:absolute</code> or <code>position:fixed</code> elements when placed inside
    /// of them. e.g. using a menu inside of an <code>md-dialog</code>.<br><strong>NOTE</strong>:
    /// Fixed menus will not scroll with the page and will be fixed to the window instead.
    #[prop_or_default]
    pub positioning: Option<AttrValue>,
    /// Skips the opening and closing animations.
    #[prop_or_default]
    pub quick: bool,
    /// Displays overflow content like a submenu.<br><strong>NOTE</strong>: This may cause adverse
    /// effects if you set <code>md-menu {max-height:...}</code> and have items overflowing items
    /// in the “y” direction.
    #[prop_or_default]
    pub has_overflow: bool,
    /// Opens the menu and makes it visible. Alternative to the <code>.show()</code> and
    /// <code>.close()</code> methods
    #[prop_or_default]
    pub open: bool,
    /// Offsets the menu’s inline alignment from the anchor by the given number in pixels. This
    /// value is direction aware and will follow the LTR / RTL direction.<br>e.g. LTR: positive
    /// -&gt; right, negative -&gt; left RTL: positive -&gt; left, negative -&gt; right
    #[prop_or_default]
    pub x_offset: usize,
    /// Offsets the menu’s block alignment from the anchor by the given number in pixels.<br>e.g.
    /// positive -&gt; down, negative -&gt; up
    #[prop_or_default]
    pub y_offset: usize,
    /// The max time between the keystrokes of the typeahead menu behavior before it clears the
    /// typeahead buffer.
    #[prop_or(200)]
    pub typepeahead_delay: usize,
    /// The corner of the anchor which to align the menu in the standard logical property style of
    /// <block>-<inline> e.g. <code>'end-start'</code>.<br>NOTE: This value may not be respected by
    /// the menu positioning algorithm if the menu would render outisde the viewport.
    #[prop_or_default]
    pub anchor_corner: Option<AttrValue>,
    /// The corner of the menu which to align the anchor in the standard logical property style of
    /// <block>-<inline> e.g. <code>'start-start'</code>.<br>NOTE: This value may not be respected
    /// by the menu positioning algorithm if the menu would render outisde the viewport.
    #[prop_or_default]
    pub menu_corner: Option<AttrValue>,
    /// Keeps the user clicks outside the menu.<br>NOTE: clicking outside may still cause focusout
    /// to close the menu so see <code>stayOpenOnFocusout</code>.
    #[prop_or_default]
    pub stay_open_on_outside_click: bool,
    /// Keeps the menu open when focus leaves the menu’s composed subtree.<br>NOTE: Focusout
    /// behavior will stop propagation of the focusout event. Set this property to true to opt-out
    /// of menu’s focuout handling altogether.
    #[prop_or_default]
    pub stay_open_on_focusout: bool,
    /// After closing, does not restore focus to the last focused element before the menu was
    /// opened.
    #[prop_or_default]
    pub skip_restore_focus: bool,
    /// The element that should be focused by default once opened.<br>NOTE: When setting default
    /// focus to ‘LIST_ROOT’, remember to change <code>tabindex</code> to <code>0</code> and change
    /// md-menu’s display to something other than <code>display: contents</code> when necessary.
    #[prop_or_default]
    pub default_focus: Option<AttrValue>,
    /// Whether or not the current menu is a submenu and should not handle specific navigation
    /// keys.
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
       quick={props.quick.then(|| AttrValue::from(""))}
       has-overflow={props.has_overflow.then(|| AttrValue::from(""))}
       open={props.open}
       x-offset={props.x_offset.to_string()}
       y-offset={props.y_offset.to_string()}
       typeahead-delay={props.typepeahead_delay.to_string()}
       anchor-corner={props.anchor_corner.clone()}
       menu-corner={props.menu_corner.clone()}
       stay-open-on-outside-click={props.stay_open_on_outside_click.then(|| AttrValue::from(""))}
       stay-open-on-focusout={props.stay_open_on_focusout.then(|| AttrValue::from(""))}
       skip-restore-focus={props.skip_restore_focus.then(|| AttrValue::from(""))}
       default-focus={props.default_focus.clone()}
       is-submenu={props.is_submenu.then(|| AttrValue::from(""))}
    >
        {props.children.clone()}
    </md-menu> }
}
