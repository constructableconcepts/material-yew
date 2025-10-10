use crate::customizable::CustomizableProps;
use strum::Display;
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub enum FabStyle {
    Standard,
    Branded,
}

impl FabStyle {
    fn as_tag_name(&self) -> &'static str {
        match self {
            FabStyle::Standard => "md-fab",
            FabStyle::Branded => "md-branded-fab",
        }
    }
}

#[derive(Display, PartialEq, Clone, Copy, Default)]
#[strum(serialize_all = "lowercase")]
pub enum FabVariant {
    #[default]
    Surface,
    Primary,
    Secondary,
    Tertiary,
}

#[derive(Display, PartialEq, Clone, Copy, Default)]
#[strum(serialize_all = "lowercase")]
pub enum FabSize {
    Small,
    #[default]
    Medium,
    Large,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    /// The FAB color variant to render.
    #[prop_or_default]
    pub variant: FabVariant,
    /// The size of the FAB.<br>NOTE: Branded FABs cannot be sized to <code>small</code>, and
    /// Extended FABs do not have different sizes.
    #[prop_or_default]
    pub size: FabSize,
    /// The text to display on the FAB.
    #[prop_or_default]
    pub label: AttrValue,
    /// Lowers the FAB’s elevation.
    #[prop_or_default]
    pub lowered: bool,
    /// The style of the FAB to use.
    pub fab_style: FabStyle,
    /// The icon to display in the FAB.
    #[prop_or_default]
    pub icon: Html,
    /// Customizable properties.
    #[prop_or_default]
    pub customizable: CustomizableProps,
}

#[function_component]
pub fn Fab(props: &Props) -> Html {
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

    crate::import_material_web_module!("/md-web/fab.js");

    html! { <@{props.fab_style.as_tag_name()}
        ref={node_ref}
        variant={props.variant.to_string()}
        size={props.size.to_string()}
        label={props.label.clone()}
        lowered={props.lowered.then_some(AttrValue::from(""))}
    >
        <span slot="icon">{ props.icon.clone() }</span>
    </@> }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Icon;
    use gloo_utils::document;
    use std::collections::BTreeMap;
    use wasm_bindgen_test::*;
    use yew::prelude::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn it_renders_with_enums() {
        let host = document().create_element("div").unwrap();
        let props = Props {
            variant: FabVariant::Primary,
            size: FabSize::Large,
            label: AttrValue::default(),
            lowered: false,
            fab_style: FabStyle::Standard,
            icon: html! {},
            customizable: CustomizableProps::default(),
        };

        yew::Renderer::<Fab>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("variant=\"primary\""));
        assert!(rendered_html.contains("size=\"large\""));
    }

    #[wasm_bindgen_test]
    fn it_renders_icon_slot() {
        let host = document().create_element("div").unwrap();
        let props = Props {
            variant: FabVariant::default(),
            size: FabSize::default(),
            label: AttrValue::default(),
            lowered: false,
            fab_style: FabStyle::Standard,
            icon: html! { <Icon icon={"star".to_string()} /> },
            customizable: CustomizableProps::default(),
        };

        yew::Renderer::<Fab>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("<span slot=\"icon\">"));
        assert!(rendered_html.contains("<md-icon>star</md-icon>"));
        assert!(rendered_html.contains("</span>"));
    }

    #[wasm_bindgen_test]
    fn it_renders_with_custom_style_and_aria() {
        let host = document().create_element("div").unwrap();
        let mut aria = BTreeMap::new();
        aria.insert("aria-label".to_string(), "Custom Fab".into());
        let props = Props {
            variant: FabVariant::default(),
            size: FabSize::default(),
            label: AttrValue::default(),
            lowered: false,
            fab_style: FabStyle::Standard,
            icon: html! {},
            customizable: CustomizableProps {
                style: Some("color: purple;".into()),
                aria: Some(aria),
            },
        };

        yew::Renderer::<Fab>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("style=\"color: purple;\""));
        assert!(rendered_html.contains("aria-label=\"Custom Fab\""));
    }
}