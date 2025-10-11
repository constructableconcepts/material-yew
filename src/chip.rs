use yew::prelude::*;

#[derive(PartialEq)]
pub enum ChipVariants {
    Assist,
    Filter,
    Input,
    Suggestion,
}

impl ChipVariants {
    fn as_tag_name(&self) -> &'static str {
        match self {
            ChipVariants::Assist => "md-assist-chip",
            ChipVariants::Filter => "md-filter-chip",
            ChipVariants::Input => "md-input-chip",
            ChipVariants::Suggestion => "md-suggestion-chip",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    /// Whether the chip is elevated.
    #[prop_or_default]
    pub elevated: bool,
    /// The URL that the chip links to.
    #[prop_or_default]
    pub href: AttrValue,
    /// The target of the link.
    #[prop_or_default]
    pub target: AttrValue,
    /// Tells the browser to download the linked file instead of navigating to it.
    #[prop_or_default]
    pub download: AttrValue,
    /// Whether or not the chip is disabled.
    #[prop_or_default]
    pub disabled: bool,
    /// When true, allow disabled chips to be focused with arrow keys.
    #[prop_or_default]
    pub always_focusable: bool,
    /// Whether the chip is selected.
    #[prop_or_default]
    pub selected: bool,
    /// Whether the chip is removable.
    #[prop_or_default]
    pub removable: bool,
    /// Whether the chip has an avatar.
    #[prop_or_default]
    pub avatar: bool,
    /// The variant to use.
    pub variant: ChipVariants,
    /// The content of the chip's label.
    #[prop_or_default]
    pub children: Html,
    /// The icon to display in the chip.
    #[prop_or_default]
    pub icon: Html,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[function_component]
pub fn Chip(props: &Props) -> Html {
    crate::import_material_web_module!("/md-web/chip.js");

    html! { <@{props.variant.as_tag_name()}
        elevated={props.elevated.then_some(AttrValue::from(""))}
        href={props.href.clone()}
        target={props.target.clone()}
        download={props.download.clone()}
        disabled={props.disabled}
        always-focusable={props.always_focusable.then_some(AttrValue::from(""))}
        selected={props.selected}
        removable={props.removable.then_some(AttrValue::from(""))}
        avatar={props.avatar.then_some(AttrValue::from(""))}
        id={props.id.clone()}
        style={props.style.clone()}
    >
        {props.children.clone()}
        if props.icon != html! {} {
            <span slot="icon">{props.icon.clone()}</span>
        }
    </@> }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gloo_utils::document;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn it_renders_with_download() {
        let host = document().create_element("div").unwrap();
        let props = Props {
            elevated: false,
            href: AttrValue::default(),
            target: AttrValue::default(),
            download: "file.txt".into(),
            disabled: false,
            always_focusable: false,
            selected: false,
            removable: false,
            avatar: false,
            variant: ChipVariants::Assist,
            children: html! { "Test Chip" },
            icon: html! {},
            id: None,
            style: None,
        };

        yew::Renderer::<Chip>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("download=\"file.txt\""));
        assert!(rendered_html.contains("Test Chip"));
    }

    #[wasm_bindgen_test]
    fn it_renders_with_custom_style_and_id() {
        let host = document().create_element("div").unwrap();
        let props = Props {
            elevated: false,
            href: AttrValue::default(),
            target: AttrValue::default(),
            download: AttrValue::default(),
            disabled: false,
            always_focusable: false,
            selected: false,
            removable: false,
            avatar: false,
            variant: ChipVariants::Assist,
            children: html! { "Test Chip" },
            icon: html! {},
            id: Some("custom-id".into()),
            style: Some("color: green;".into()),
        };

        yew::Renderer::<Chip>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("id=\"custom-id\""));
        assert!(rendered_html.contains("style=\"color: green;\""));
    }

    #[wasm_bindgen_test]
    fn it_renders_filter_variant_with_selected_and_removable() {
        let host = document().create_element("div").unwrap();
        let props = Props {
            elevated: false,
            href: AttrValue::default(),
            target: AttrValue::default(),
            download: AttrValue::default(),
            disabled: false,
            always_focusable: false,
            selected: true,
            removable: true,
            avatar: false,
            variant: ChipVariants::Filter,
            children: html! { "Test Chip" },
            icon: html! {},
            id: None,
            style: None,
        };

        yew::Renderer::<Chip>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("selected"));
        assert!(rendered_html.contains("removable"));
    }

    #[wasm_bindgen_test]
    fn it_renders_input_variant_with_avatar() {
        let host = document().create_element("div").unwrap();
        let props = Props {
            elevated: false,
            href: AttrValue::default(),
            target: AttrValue::default(),
            download: AttrValue::default(),
            disabled: false,
            always_focusable: false,
            selected: false,
            removable: false,
            avatar: true,
            variant: ChipVariants::Input,
            children: html! { "Test Chip" },
            icon: html! {},
            id: None,
            style: None,
        };

        yew::Renderer::<Chip>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("avatar"));
    }

    #[wasm_bindgen_test]
    fn it_renders_with_icon() {
        let host = document().create_element("div").unwrap();
        let props = Props {
            elevated: false,
            href: AttrValue::default(),
            target: AttrValue::default(),
            download: AttrValue::default(),
            disabled: false,
            always_focusable: false,
            selected: false,
            removable: false,
            avatar: false,
            variant: ChipVariants::Assist,
            children: html! { "Test Chip" },
            icon: html! { <i class="material-icons">{"favorite"}</i> },
            id: None,
            style: None,
        };

        yew::Renderer::<Chip>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("<span slot=\"icon\">"));
        assert!(rendered_html.contains("favorite"));
    }
}