use yew::prelude::*;

/// Properties for the `AssistChip` component.
#[derive(Properties, PartialEq, Clone)]
pub struct AssistChipProps {
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
    /// The content of the chip's label.
    #[prop_or_default]
    pub children: Html,
    /// The icon to display in the chip.
    #[prop_or_default]
    pub icon: Html,
    /// The id of the chip.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// The style of the chip.
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// Properties for the `FilterChip` component.
#[derive(Properties, PartialEq, Clone)]
pub struct FilterChipProps {
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
    /// The content of the chip's label.
    #[prop_or_default]
    pub children: Html,
    /// The icon to display in the chip.
    #[prop_or_default]
    pub icon: Html,
    /// The id of the chip.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// The style of the chip.
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// Properties for the `InputChip` component.
#[derive(Properties, PartialEq, Clone)]
pub struct InputChipProps {
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
    /// Whether the chip has an avatar.
    #[prop_or_default]
    pub avatar: bool,
    /// The content of the chip's label.
    #[prop_or_default]
    pub children: Html,
    /// The icon to display in the chip.
    #[prop_or_default]
    pub icon: Html,
    /// The id of the chip.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// The style of the chip.
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// Properties for the `SuggestionChip` component.
#[derive(Properties, PartialEq, Clone)]
pub struct SuggestionChipProps {
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
    /// The content of the chip's label.
    #[prop_or_default]
    pub children: Html,
    /// The icon to display in the chip.
    #[prop_or_default]
    pub icon: Html,
    /// The id of the chip.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// The style of the chip.
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// An assist chip component.
///
/// [Material Design spec](https://m3.material.io/components/chips/overview)
#[function_component]
pub fn AssistChip(props: &AssistChipProps) -> Html {
    html! {
        <RawChip
            tag="md-assist-chip"
            elevated={props.elevated}
            href={props.href.clone()}
            target={props.target.clone()}
            download={props.download.clone()}
            disabled={props.disabled}
            always_focusable={props.always_focusable}
            children={props.children.clone()}
            icon={props.icon.clone()}
            id={props.id.clone()}
            style={props.style.clone()}
        />
    }
}

/// A filter chip component.
///
/// [Material Design spec](https://m3.material.io/components/chips/overview)
#[function_component]
pub fn FilterChip(props: &FilterChipProps) -> Html {
    html! {
        <RawChip
            tag="md-filter-chip"
            elevated={props.elevated}
            href={props.href.clone()}
            target={props.target.clone()}
            download={props.download.clone()}
            disabled={props.disabled}
            always_focusable={props.always_focusable}
            selected={props.selected}
            removable={props.removable}
            children={props.children.clone()}
            icon={props.icon.clone()}
            id={props.id.clone()}
            style={props.style.clone()}
        />
    }
}

/// An input chip component.
///
/// [Material Design spec](https://m3.material.io/components/chips/overview)
#[function_component]
pub fn InputChip(props: &InputChipProps) -> Html {
    html! {
        <RawChip
            tag="md-input-chip"
            href={props.href.clone()}
            target={props.target.clone()}
            download={props.download.clone()}
            disabled={props.disabled}
            always_focusable={props.always_focusable}
            selected={props.selected}
            avatar={props.avatar}
            children={props.children.clone()}
            icon={props.icon.clone()}
            id={props.id.clone()}
            style={props.style.clone()}
        />
    }
}

/// A suggestion chip component.
///
/// [Material Design spec](https://m3.material.io/components/chips/overview)
#[function_component]
pub fn SuggestionChip(props: &SuggestionChipProps) -> Html {
    html! {
        <RawChip
            tag="md-suggestion-chip"
            elevated={props.elevated}
            href={props.href.clone()}
            target={props.target.clone()}
            download={props.download.clone()}
            disabled={props.disabled}
            always_focusable={props.always_focusable}
            children={props.children.clone()}
            icon={props.icon.clone()}
            id={props.id.clone()}
            style={props.style.clone()}
        />
    }
}

#[derive(Properties, PartialEq)]
struct RawChipProps {
    tag: &'static str,
    #[prop_or_default]
    elevated: bool,
    #[prop_or_default]
    href: AttrValue,
    #[prop_or_default]
    target: AttrValue,
    #[prop_or_default]
    download: AttrValue,
    #[prop_or_default]
    disabled: bool,
    #[prop_or_default]
    always_focusable: bool,
    #[prop_or_default]
    selected: bool,
    #[prop_or_default]
    removable: bool,
    #[prop_or_default]
    avatar: bool,
    #[prop_or_default]
    children: Html,
    #[prop_or_default]
    icon: Html,
    #[prop_or_default]
    id: Option<AttrValue>,
    #[prop_or_default]
    style: Option<AttrValue>,
}

#[function_component]
fn RawChip(props: &RawChipProps) -> Html {
    crate::import_material_web_module!("/md-web/chip.js");

    html! {
        <@{props.tag}
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
        </@>
    }
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
        let props = AssistChipProps {
            elevated: false,
            href: AttrValue::default(),
            target: AttrValue::default(),
            download: "file.txt".into(),
            disabled: false,
            always_focusable: false,
            children: html! { "Test Chip" },
            icon: html! {},
            id: None,
            style: None,
        };

        yew::Renderer::<AssistChip>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("download=\"file.txt\""));
        assert!(rendered_html.contains("Test Chip"));
    }

    #[wasm_bindgen_test]
    fn it_renders_with_custom_style_and_id() {
        let host = document().create_element("div").unwrap();
        let props = AssistChipProps {
            elevated: false,
            href: AttrValue::default(),
            target: AttrValue::default(),
            download: AttrValue::default(),
            disabled: false,
            always_focusable: false,
            children: html! { "Test Chip" },
            icon: html! {},
            id: Some("custom-id".into()),
            style: Some("color: green;".into()),
        };

        yew::Renderer::<AssistChip>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("id=\"custom-id\""));
        assert!(rendered_html.contains("style=\"color: green;\""));
    }

    #[wasm_bindgen_test]
    fn it_renders_filter_variant_with_selected_and_removable() {
        let host = document().create_element("div").unwrap();
        let props = FilterChipProps {
            elevated: false,
            href: AttrValue::default(),
            target: AttrValue::default(),
            download: AttrValue::default(),
            disabled: false,
            always_focusable: false,
            selected: true,
            removable: true,
            children: html! { "Test Chip" },
            icon: html! {},
            id: None,
            style: None,
        };

        yew::Renderer::<FilterChip>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("selected"));
        assert!(rendered_html.contains("removable"));
    }

    #[wasm_bindgen_test]
    fn it_renders_input_variant_with_avatar() {
        let host = document().create_element("div").unwrap();
        let props = InputChipProps {
            href: AttrValue::default(),
            target: AttrValue::default(),
            download: AttrValue::default(),
            disabled: false,
            always_focusable: false,
            selected: false,
            avatar: true,
            children: html! { "Test Chip" },
            icon: html! {},
            id: None,
            style: None,
        };

        yew::Renderer::<InputChip>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("avatar"));
    }

    #[wasm_bindgen_test]
    fn it_renders_with_icon() {
        let host = document().create_element("div").unwrap();
        let props = AssistChipProps {
            elevated: false,
            href: AttrValue::default(),
            target: AttrValue::default(),
            download: AttrValue::default(),
            disabled: false,
            always_focusable: false,
            children: html! { "Test Chip" },
            icon: html! { <i class="material-icons">{"favorite"}</i> },
            id: None,
            style: None,
        };

        yew::Renderer::<AssistChip>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("<span slot=\"icon\">"));
        assert!(rendered_html.contains("favorite"));
    }
}