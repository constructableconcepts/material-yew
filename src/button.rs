use crate::customizable::CustomizableProps;
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;

#[derive(PartialEq)]
pub enum ButtonVariants {
    Elevated,
    Filled,
    FilledTonal,
    Outlined,
    Text,
}

impl ButtonVariants {
    fn as_tag_name(&self) -> &'static str {
        match self {
            ButtonVariants::Elevated => "md-elevated-button",
            ButtonVariants::Filled => "md-filled-button",
            ButtonVariants::FilledTonal => "md-filled-tonal-button",
            ButtonVariants::Outlined => "md-outlined-button",
            ButtonVariants::Text => "md-text-button",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    /// Whether or not the button is disabled.
    #[prop_or_default]
    pub disabled: bool,
    /// When true, the button's ripple and state are disabled, but the button remains focusable.
    #[prop_or_default]
    pub soft_disabled: bool,
    /// The URL that the link button points to.
    #[prop_or_default]
    pub href: AttrValue,
    /// Where to display the linked `href` URL for a link button. Common options include
    /// `_blank` to open in a new tab.
    #[prop_or_default]
    pub target: AttrValue,
    /// Tells the browser to download the linked file instead of navigating to it.
    #[prop_or_default]
    pub download: AttrValue,
    /// Whether to render the icon at the inline end of the label rather than the inline
    /// start.<br><em>Note:</em> Link buttons cannot have trailing icons.
    #[prop_or_default]
    pub trailing_icon: bool,
    /// Whether to display the icon or not.
    #[prop_or_default]
    pub has_icon: bool,
    /// The default behavior of the button. May be "button", "reset", or "submit" (default).
    #[prop_or(AttrValue::from("submit"))]
    pub r#type: AttrValue,
    /// The value added to a form with the button's name when the button submits a form.
    #[prop_or_default]
    pub value: AttrValue,
    /// The name of the button.
    #[prop_or_default]
    pub name: AttrValue,
    /// The id of the form the button is associated with.
    #[prop_or_default]
    pub form: AttrValue,
    /// The variant to use.
    pub variant: ButtonVariants,
    pub children: Html,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    /// Customizable properties.
    #[prop_or_default]
    pub customizable: CustomizableProps,
}

#[function_component]
pub fn Button(props: &Props) -> Html {
    let node_ref = use_node_ref();
    let customizable = props.customizable.clone();
    use_effect_with((node_ref.clone(), customizable), |(node_ref, customizable)| {
        if let Some(element) = node_ref.get() {
            let element = element.dyn_ref::<Element>().unwrap();

            if let Some(id) = &customizable.id {
                element.set_attribute("id", id).unwrap();
            }

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

    crate::import_material_web_module!("/md-web/button.js");

    html! { <@{props.variant.as_tag_name()}
        ref={node_ref}
        disabled={props.disabled}
        soft-disabled={props.soft_disabled.then_some(AttrValue::from(""))}
        href={props.href.clone()}
        target={props.target.clone()}
        download={props.download.clone()}
        trailing-icon={props.trailing_icon.then_some(AttrValue::from(""))}
        has-icon={props.has_icon.then_some(AttrValue::from(""))}
        type={props.r#type.clone()}
        value={props.value.clone()}
        name={props.name.clone()}
        form={props.form.clone()}
        onclick={props.onclick.clone()}
    > {props.children.clone()} </@> }
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
    fn it_renders_with_type() {
        let host = document().create_element("div").unwrap();
        let props = Props {
            disabled: false,
            soft_disabled: false,
            href: AttrValue::default(),
            target: AttrValue::default(),
            download: AttrValue::default(),
            trailing_icon: false,
            has_icon: false,
            r#type: "button".into(),
            value: AttrValue::default(),
            name: AttrValue::default(),
            form: "my-form".into(),
            variant: ButtonVariants::Filled,
            children: html! { "Test Button" },
            onclick: Callback::default(),
            customizable: CustomizableProps::default(),
        };

        yew::Renderer::<Button>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("type=\"button\""));
        assert!(rendered_html.contains("form=\"my-form\""));
    }

    #[wasm_bindgen_test]
    fn it_renders_with_soft_disabled_and_download() {
        let host = document().create_element("div").unwrap();
        let props = Props {
            disabled: false,
            soft_disabled: true,
            href: AttrValue::default(),
            target: AttrValue::default(),
            download: "file.txt".into(),
            trailing_icon: false,
            has_icon: false,
            r#type: "button".into(),
            value: AttrValue::default(),
            name: AttrValue::default(),
            form: AttrValue::default(),
            variant: ButtonVariants::Filled,
            children: html! { "Test Button" },
            onclick: Callback::default(),
            customizable: CustomizableProps::default(),
        };

        yew::Renderer::<Button>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("soft-disabled"));
        assert!(rendered_html.contains("download=\"file.txt\""));
    }

    #[wasm_bindgen_test]
    fn it_renders_with_custom_style_and_aria() {
        let host = document().create_element("div").unwrap();
        let mut aria = BTreeMap::new();
        aria.insert("aria-label".to_string(), "Custom Label".into());
        let props = Props {
            disabled: false,
            soft_disabled: false,
            href: AttrValue::default(),
            target: AttrValue::default(),
            download: AttrValue::default(),
            trailing_icon: false,
            has_icon: false,
            r#type: "button".into(),
            value: AttrValue::default(),
            name: AttrValue::default(),
            form: AttrValue::default(),
            variant: ButtonVariants::Filled,
            children: html! { "Test Button" },
            onclick: Callback::default(),
            customizable: CustomizableProps {
                style: Some("color: red;".into()),
                aria: Some(aria),
            },
        };

        yew::Renderer::<Button>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("style=\"color: red;\""));
        assert!(rendered_html.contains("aria-label=\"Custom Label\""));
    }
}