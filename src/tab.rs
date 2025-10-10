use crate::customizable::CustomizableProps;
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub active: bool,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub customizable: CustomizableProps,
}

#[function_component]
pub fn Tab(props: &Props) -> Html {
    let node_ref = use_node_ref();
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

    html! {
        <md-tab
            ref={node_ref}
            disabled={props.disabled}
            active={props.active.then_some(AttrValue::from(""))}
        >
            { for props.children.iter() }
        </md-tab>
    }
}