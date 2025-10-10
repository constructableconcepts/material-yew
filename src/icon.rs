use crate::customizable::CustomizableProps;
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub icon: AttrValue,
    /// Customizable properties.
    #[prop_or_default]
    pub customizable: CustomizableProps,
}

#[function_component(Icon)]
pub fn icon(props: &Props) -> Html {
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
        <md-icon ref={node_ref}>{ props.icon.clone() }</md-icon>
    }
}