use yew::prelude::*;

/// Defines the variants of the button component.
#[derive(PartialEq)]
pub enum ButtonVariants {
    /// A button with a shadow effect.
    Elevated,
    /// A button with a solid background color.
    Filled,
    /// A button with a less prominent background color.
    FilledTonal,
    /// A button with a border and no background color.
    Outlined,
    /// A button with no border or background color.
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
    /// The icon to display in the button.
    #[prop_or_default]
    pub icon: Html,
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
    /// The content of the button.
    pub children: Html,
    /// The callback to be called when the button is clicked.
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    /// The id of the button.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// The style of the button.
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// A button component that can be used for various actions.
#[function_component]
pub fn Button(props: &Props) -> Html {
    crate::import_material_web_module!("/md-web/button.js");

    let icon = if props.icon != html! {} {
        html! { <span slot="icon">{props.icon.clone()}</span> }
    } else {
        html! {}
    };

    html! { <@{props.variant.as_tag_name()}
        disabled={props.disabled}
        soft-disabled={props.soft_disabled.then_some(AttrValue::from(""))}
        href={props.href.clone()}
        target={props.target.clone()}
        download={props.download.clone()}
        trailing-icon={props.trailing_icon.then_some(AttrValue::from(""))}
        type={props.r#type.clone()}
        value={props.value.clone()}
        name={props.name.clone()}
        form={props.form.clone()}
        onclick={props.onclick.clone()}
        id={props.id.clone()}
        style={props.style.clone()}
    >
        {icon}
        {props.children.clone()}
    </@> }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Icon;
    use gloo_utils::document;
    use wasm_bindgen_test::*;

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
            icon: html! {},
            r#type: "button".into(),
            value: AttrValue::default(),
            name: AttrValue::default(),
            form: "my-form".into(),
            variant: ButtonVariants::Filled,
            children: html! { "Test Button" },
            onclick: Callback::default(),
            id: None,
            style: None,
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
            icon: html! {},
            r#type: "button".into(),
            value: AttrValue::default(),
            name: AttrValue::default(),
            form: AttrValue::default(),
            variant: ButtonVariants::Filled,
            children: html! { "Test Button" },
            onclick: Callback::default(),
            id: None,
            style: None,
        };

        yew::Renderer::<Button>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("soft-disabled"));
        assert!(rendered_html.contains("download=\"file.txt\""));
    }

    #[wasm_bindgen_test]
    fn it_renders_with_custom_style_and_id() {
        let host = document().create_element("div").unwrap();
        let props = Props {
            disabled: false,
            soft_disabled: false,
            href: AttrValue::default(),
            target: AttrValue::default(),
            download: AttrValue::default(),
            trailing_icon: false,
            icon: html! {},
            r#type: "button".into(),
            value: AttrValue::default(),
            name: AttrValue::default(),
            form: AttrValue::default(),
            variant: ButtonVariants::Filled,
            children: html! { "Test Button" },
            onclick: Callback::default(),
            id: Some("custom-id".into()),
            style: Some("color: red;".into()),
        };

        yew::Renderer::<Button>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("id=\"custom-id\""));
        assert!(rendered_html.contains("style=\"color: red;\""));
    }

    #[wasm_bindgen_test]
    fn it_renders_with_icon() {
        let host = document().create_element("div").unwrap();
        let props = Props {
            disabled: false,
            soft_disabled: false,
            href: AttrValue::default(),
            target: AttrValue::default(),
            download: AttrValue::default(),
            trailing_icon: false,
            icon: html! { <Icon icon="star" /> },
            r#type: "button".into(),
            value: AttrValue::default(),
            name: AttrValue::default(),
            form: AttrValue::default(),
            variant: ButtonVariants::Filled,
            children: html! { "Test Button" },
            onclick: Callback::default(),
            id: None,
            style: None,
        };

        yew::Renderer::<Button>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("<span slot=\"icon\">"));
        assert!(rendered_html.contains("<md-icon>star</md-icon>"));
        assert!(rendered_html.contains("</span>"));
    }
}