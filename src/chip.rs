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
    /// The variant to use.
    pub variant: ChipVariants,
    /// The content of the chip's label.
    #[prop_or_default]
    pub children: Html,
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
    fn it_renders_with_download() {
        let host = document().create_element("div").unwrap();
        let props = Props {
            elevated: false,
            href: AttrValue::default(),
            target: AttrValue::default(),
            download: "file.txt".into(),
            disabled: false,
            always_focusable: false,
            variant: ChipVariants::Assist,
            children: html! { "Test Chip" },
        };

        yew::Renderer::<Chip>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("download=\"file.txt\""));
        assert!(rendered_html.contains("Test Chip"));
    }
}