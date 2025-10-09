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
    ///
    #[prop_or_default]
    pub elevated: bool,
    ///
    #[prop_or_default]
    pub href: Option<AttrValue>,
    ///
    #[prop_or_default]
    pub target: Option<AttrValue>,
    /// Tells the browser to download the linked file instead of navigating to it.
    #[prop_or_default]
    pub download: Option<AttrValue>,
    /// Whether or not the chip is disabled.<br>Disabled chips are not focusable, unless
    /// <code>always-focusable</code> is set.
    #[prop_or_default]
    pub disabled: bool,
    /// When true, allow disabled chips to be focused with arrow keys.<br>Add this when a chip needs increased visibility when disabled. See https://www.w3.org/WAI/ARIA/apg/practices/keyboard-interface/#kbd_disabled_controls for more guidance on when this is needed.
    #[prop_or_default]
    pub always_focusable: bool,
    /// The label of the chip.
    #[prop_or_default]
    pub label: Option<AttrValue>,
    /// The variant to use.
    pub variant: ChipVariants,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn Chip(props: &Props) -> Html {
    crate::import_material_web_module!("/md-web/chip.js");
    html! { <@{props.variant.as_tag_name()}
        elevated={props.elevated.then(|| AttrValue::from(""))}
        href={props.href.clone()}
        target={props.target.clone()}
        download={props.download.clone()}
        disabled={props.disabled}
        always-focusable={props.always_focusable.then(|| AttrValue::from(""))}
        label={props.label.clone()}
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
            href: None,
            target: None,
            download: Some("file.txt".into()),
            disabled: false,
            always_focusable: false,
            label: Some("Test Chip".into()),
            variant: ChipVariants::Assist,
            children: html! {},
        };

        yew::Renderer::<Chip>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("download=\"file.txt\""));
    }
}
