use js_sys::Reflect;
use wasm_bindgen::JsValue;
use web_sys::HtmlFormElement as HTMLFormElement;
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
    #[prop_or(Some(false))]
    pub disabled: Option<bool>,
    /// The URL that the link button points to.
    #[prop_or(Some(AttrValue::Static("")))]
    pub href: Option<AttrValue>,
    /// Where to display the linked <code>href</code> URL for a link button. Common options include
    /// <code>_blank</code> to open in a new tab.
    #[prop_or(Some(AttrValue::Static("")))]
    pub target: Option<AttrValue>,
    /// Whether to render the icon at the inline end of the label rather than the inline
    /// start.<br><em>Note:</em> Link buttons cannot have trailing icons.
    #[prop_or(Some(false))]
    pub trailing_icon: Option<bool>,
    /// Whether to display the icon or not.
    #[prop_or(Some(false))]
    pub has_icon: Option<bool>,
    ///
    #[prop_or(Some(AttrValue::Static("submit")))]
    pub r#type: Option<AttrValue>,
    ///
    #[prop_or(Some(AttrValue::Static("")))]
    pub value: Option<AttrValue>,
    ///
    #[prop_or(Some(AttrValue::Static("")))]
    pub name: Option<AttrValue>,
    ///
    #[prop_or_default]
    pub form: Option<HTMLFormElement>,
    /// The variant to use.
    pub variant: ButtonVariants,
    pub children: Html,
    #[prop_or(None)]
    pub onclick: Option<Callback<MouseEvent>>,
}

#[function_component]
pub fn Button(props: &Props) -> Html {
    let node_ref = use_node_ref();

    {
        let node_ref = node_ref.clone();
        let form = props.form.clone();
        use_effect_with(form, move |form| {
            let element = node_ref.get().unwrap();
            let value = form
                .as_ref()
                .map(|f| f.into())
                .unwrap_or(JsValue::NULL);
            Reflect::set(&element, &"form".into(), &value).unwrap();
            move || {}
        });
    }

    crate::import_material_web_module!("/md-web/button.js");
    html! { <@{props.variant.as_tag_name()}
        ref={node_ref}
        disabled={props.disabled.unwrap_or(false)}
        href={props.href.clone()}
        target={props.target.clone()}
        trailingIcon={props.trailing_icon.filter(|&v| v).map(|_| AttrValue::from(""))}
        hasIcon={props.has_icon.filter(|&v| v).map(|_| AttrValue::from(""))}
        type={props.r#type.clone()}
        value={props.value.clone().unwrap_or_default()}
        name={props.name.clone()}
        onclick={props.onclick.clone()}
    > {props.children.clone()} </@> }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gloo_utils::document;
    use wasm_bindgen_test::*;
    use yew::prelude::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn it_renders_with_type() {
        let host = document().create_element("div").unwrap();
        let props = Props {
            disabled: Some(false),
            href: None,
            target: None,
            trailing_icon: Some(false),
            has_icon: Some(false),
            r#type: Some("button".into()),
            value: None,
            name: None,
            form: None,
            variant: ButtonVariants::Filled,
            children: html! { "Test Button" },
            onclick: None,
        };

        yew::Renderer::<Button>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("type=\"button\""));
    }
}
