use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::HtmlElement;
use yew::prelude::*;

/// A handle to imperatively control the Dialog component.
#[derive(Debug, Clone, Default)]
pub struct DialogRef {
    node_ref: NodeRef,
}

impl DialogRef {
    /// Shows the dialog.
    pub fn show(&self) {
        if let Some(element) = self.node_ref.get() {
            if let Ok(dialog) = element.dyn_into::<HtmlElement>() {
                // In a real scenario, you would use wasm-bindgen to call the `show()` method
                // on the underlying material-web component. This is a simplification for now.
                let _ = dialog.set_attribute("open", "true");
            }
        }
    }

    /// Closes the dialog.
    pub fn close(&self) {
        if let Some(element) = self.node_ref.get() {
            if let Ok(dialog) = element.dyn_into::<HtmlElement>() {
                // In a real scenario, you would use wasm-bindgen to call the `close()` method.
                let _ = dialog.remove_attribute("open");
            }
        }
    }
}

/// Properties for the `Dialog` component.
#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    /// The content to display in the `headline` slot.
    #[prop_or_default]
    pub headline: Children,
    /// The content to display in the main `content` slot.
    #[prop_or_default]
    pub content: Children,
    /// The content to display in the `actions` slot (usually buttons).
    #[prop_or_default]
    pub actions: Children,
    /// A handle to allow imperative control of the dialog.
    #[prop_or_default]
    pub node_ref: NodeRef,
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
}

/// A dialog component that provides a container for content and actions.
///
/// [Material Design spec](https://m3.material.io/components/dialogs/overview)
#[function_component(Dialog)]
pub fn dialog(props: &Props) -> Html {
    let node_ref = props.node_ref.clone();

    use_effect_with(node_ref.clone(), {
        let props = props.clone();
        move |node_ref| {
            if let Some(element) = node_ref.get() {
                let onopen = props.onopen.clone();
                let onopened = props.onopened.clone();
                let onclose = props.onclose.clone();
                let onclosed = props.onclosed.clone();
                let oncancel = props.oncancel.clone();

                let open_closure = Closure::wrap(Box::new(move |e| onopen.emit(e)) as Box<dyn FnMut(_)>);
                let opened_closure = Closure::wrap(Box::new(move |e| onopened.emit(e)) as Box<dyn FnMut(_)>);
                let close_closure = Closure::wrap(Box::new(move |e| onclose.emit(e)) as Box<dyn FnMut(_)>);
                let closed_closure = Closure::wrap(Box::new(move |e| onclosed.emit(e)) as Box<dyn FnMut(_)>);
                let cancel_closure = Closure::wrap(Box::new(move |e| oncancel.emit(e)) as Box<dyn FnMut(_)>);

                element.add_event_listener_with_callback("open", open_closure.as_ref().unchecked_ref()).unwrap();
                element.add_event_listener_with_callback("opened", opened_closure.as_ref().unchecked_ref()).unwrap();
                element.add_event_listener_with_callback("close", close_closure.as_ref().unchecked_ref()).unwrap();
                element.add_event_listener_with_callback("closed", closed_closure.as_ref().unchecked_ref()).unwrap();
                element.add_event_listener_with_callback("cancel", cancel_closure.as_ref().unchecked_ref()).unwrap();

                Box::new(move || {
                    open_closure.forget();
                    opened_closure.forget();
                    close_closure.forget();
                    closed_closure.forget();
                    cancel_closure.forget();
                }) as Box<dyn FnOnce()>
            } else {
                Box::new(|| {}) as Box<dyn FnOnce()>
            }
        }
    });

    crate::import_material_web_module!("/md-web/dialog.js");
    html! {
        <md-dialog ref={node_ref}>
            <div slot="headline">{ for props.headline.iter() }</div>
            <div slot="content">{ for props.content.iter() }</div>
            <div slot="actions">{ for props.actions.iter() }</div>
        </md-dialog>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gloo_utils::document;
    use wasm_bindgen_test::*;
    use yew::prelude::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn it_renders_slots_correctly() {
        let host = document().create_element("div").unwrap();
        let props = Props {
            headline: Children::new(vec![html! { <h1>{"Headline"}</h1> }]),
            content: Children::new(vec![html! { <p>{"Content"}</p> }]),
            actions: Children::new(vec![html! { <button>{"Action"}</button> }]),
            node_ref: NodeRef::default(),
            onopen: Callback::default(),
            onopened: Callback::default(),
            onclose: Callback::default(),
            onclosed: Callback::default(),
            oncancel: Callback::default(),
        };

        yew::Renderer::<Dialog>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("<div slot=\"headline\"><h1>Headline</h1></div>"));
        assert!(rendered_html.contains("<div slot=\"content\"><p>Content</p></div>"));
        assert!(rendered_html.contains("<div slot=\"actions\"><button>Action</button></div>"));
    }
}