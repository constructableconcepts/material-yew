use crate::customizable::CustomizableProps;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Element, EventTarget};
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    type MdDialog;

    #[wasm_bindgen(method)]
    fn show(this: &MdDialog);

    #[wasm_bindgen(method)]
    fn close(this: &MdDialog);
}

/// A handle to imperatively control the Dialog component.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DialogRef {
    node_ref: NodeRef,
}

impl DialogRef {
    /// Shows the dialog.
    pub fn show(&self) {
        if let Some(element) = self.node_ref.get() {
            let dialog: &MdDialog = element.unchecked_ref();
            dialog.show();
        }
    }

    /// Closes the dialog.
    pub fn close(&self) {
        if let Some(element) = self.node_ref.get() {
            let dialog: &MdDialog = element.unchecked_ref();
            dialog.close();
        }
    }
}

/// Properties for the `Dialog` component.
#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    /// The content to display in the `headline` slot.
    #[prop_or_default]
    pub headline: Html,
    /// The content to display in the main `content` slot.
    #[prop_or_default]
    pub content: Html,
    /// The content to display in the `actions` slot (usually buttons).
    #[prop_or_default]
    pub actions: Html,
    /// The content to display in the `icon` slot.
    #[prop_or_default]
    pub icon: Html,
    /// Opens the dialog.
    #[prop_or_default]
    pub open: bool,
    /// Skips the opening and closing animations.
    #[prop_or_default]
    pub quick: bool,
    /// Gets or sets the dialog's return value.
    #[prop_or_default]
    pub return_value: AttrValue,
    /// The type of dialog.
    #[prop_or_default]
    pub r#type: AttrValue,
    /// Disables focus trapping.
    #[prop_or_default]
    pub no_focus_trap: bool,
    /// A handle to allow imperative control of the dialog.
    #[prop_or_default]
    pub dialog_ref: DialogRef,
    /// Event fired when the dialog is opening.
    #[prop_or_default]
    pub onopen: Callback<Event>,
    /// Event fired when the dialog has opened.
    #[prop_or_default]
    pub onopened: Callback<Event>,
    /// Event fired when the dialog is closing.
    #[prop_or_default]
    pub onclose: Callback<Event>,
    /// Event fired when the dialog has closed.
    #[prop_or_default]
    pub onclosed: Callback<Event>,
    /// Event fired when the dialog is canceled.
    #[prop_or_default]
    pub oncancel: Callback<Event>,
    /// Customizable properties.
    #[prop_or_default]
    pub customizable: CustomizableProps,
}

/// A dialog component that provides a container for content and actions.
///
/// [Material Design spec](https://m3.material.io/components/dialogs/overview)
#[function_component(Dialog)]
pub fn dialog(props: &Props) -> Html {
    let node_ref = props.dialog_ref.node_ref.clone();
    let customizable = props.customizable.clone();
    use_effect_with((node_ref.clone(), customizable), |(node_ref, customizable)| {
        if let Some(element) = node_ref.get() {
            let element = element.dyn_ref::<Element>().unwrap();

            if let Some(style) = &customizable.style {
                element.set_attribute("style", style).unwrap();
            }

            if let Some(aria) = &customizable.aria {
                for (key, value) in aria {
                    if key.starts_with("aria-") {
                        element.set_attribute(key, value).unwrap();
                    }
                }
            }
        }
    });

    use_effect_with(node_ref.clone(), {
        let props = props.clone();
        move |node_ref| {
            let element = node_ref.get().unwrap();
            let target: EventTarget = element.dyn_into().unwrap();
            let mut listeners = Vec::new();

            let events = [
                ("open", props.onopen.clone()),
                ("opened", props.onopened.clone()),
                ("close", props.onclose.clone()),
                ("closed", props.onclosed.clone()),
                ("cancel", props.oncancel.clone()),
            ];

            for (event_name, callback) in events {
                let listener = Closure::<dyn FnMut(_)>::new(move |e| callback.emit(e));
                target
                    .add_event_listener_with_callback(event_name, listener.as_ref().unchecked_ref())
                    .unwrap();
                listeners.push(listener);
            }

            move || {
                for listener in listeners {
                    drop(listener);
                }
            }
        }
    });

    crate::import_material_web_module!("/md-web/dialog.js");
    html! {
        <md-dialog
            ref={node_ref}
            open={props.open}
            quick={props.quick.then_some(AttrValue::from(""))}
            returnValue={props.return_value.clone()}
            type={props.r#type.clone()}
            no-focus-trap={props.no_focus_trap.then_some(AttrValue::from(""))}
        >
            {props.icon.clone()}
            {props.headline.clone()}
            {props.content.clone()}
            {props.actions.clone()}
        </md-dialog>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gloo_utils::document;
    use std::collections::BTreeMap;
    use wasm_bindgen_test::*;
    use yew::prelude::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn it_renders_slots_correctly() {
        let host = document().create_element("div").unwrap();
        let props = Props {
            headline: html! { <h2 slot="headline">{"Headline"}</h2> },
            content: html! { <p slot="content">{"Content"}</p> },
            actions: html! { <div slot="actions"><button>{"Action"}</button></div> },
            icon: html! { <span slot="icon" class="material-icons">{"settings"}</span> },
            open: false,
            quick: false,
            return_value: AttrValue::default(),
            r#type: AttrValue::default(),
            no_focus_trap: false,
            dialog_ref: DialogRef::default(),
            onopen: Callback::default(),
            onopened: Callback::default(),
            onclose: Callback::default(),
            onclosed: Callback::default(),
            oncancel: Callback::default(),
            customizable: CustomizableProps::default(),
        };

        yew::Renderer::<Dialog>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("<span slot=\"icon\""));
        assert!(rendered_html.contains("<h2 slot=\"headline\">Headline</h2>"));
        assert!(rendered_html.contains("<p slot=\"content\">Content</p>"));
        assert!(rendered_html.contains("<div slot=\"actions\"><button>Action</button></div>"));
    }

    #[wasm_bindgen_test]
    fn it_renders_with_custom_style_and_aria() {
        let host = document().create_element("div").unwrap();
        let mut aria = BTreeMap::new();
        aria.insert("aria-label".to_string(), "Custom Dialog".into());
        let props = Props {
            headline: html! {},
            content: html! {},
            actions: html! {},
            icon: html! {},
            open: false,
            quick: false,
            return_value: AttrValue::default(),
            r#type: "alert".into(),
            no_focus_trap: true,
            dialog_ref: DialogRef::default(),
            onopen: Callback::default(),
            onopened: Callback::default(),
            onclose: Callback::default(),
            onclosed: Callback::default(),
            oncancel: Callback::default(),
            customizable: CustomizableProps {
                style: Some("color: red;".into()),
                aria: Some(aria),
            },
        };

        yew::Renderer::<Dialog>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("style=\"color: red;\""));
        assert!(rendered_html.contains("aria-label=\"Custom Dialog\""));
        assert!(rendered_html.contains("type=\"alert\""));
        assert!(rendered_html.contains("no-focus-trap"));
    }
}