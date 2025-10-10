use crate::customizable::CustomizableProps;
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;

/// A component that displays a set of chips.
///
/// [Material Design Docs](https://m3.material.io/components/chips/overview)
#[derive(Properties, PartialEq, Clone)]
pub struct ChipSetProps {
    #[prop_or_default]
    pub children: Children,
    /// Customizable properties.
    #[prop_or_default]
    pub customizable: CustomizableProps,
}

#[function_component(ChipSet)]
pub fn chip_set(props: &ChipSetProps) -> Html {
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
        <md-chip-set ref={node_ref}>
            { for props.children.iter() }
        </md-chip-set>
    }
}