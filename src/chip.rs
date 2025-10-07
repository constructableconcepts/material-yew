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
    /// The filename to use when downloading the linked resource.
    /// If not specified, the browser will determine a filename.
    /// This is only applicable when the chip is used as a link (`href` is set).
    #[prop_or_default]
    pub download: Option<AttrValue>,
    /// Whether or not the chip is disabled.<br>Disabled chips are not focusable, unless
    /// <code>always-focusable</code> is set.
    #[prop_or_default]
    pub disabled: bool,
    /// Whether or not the chip is "soft-disabled" (disabled but still
    /// focusable).
    #[prop_or_default]
    pub soft_disabled: bool,
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
        soft-disabled={props.soft_disabled.then(|| AttrValue::from(""))}
        always-focusable={props.always_focusable.then(|| AttrValue::from(""))}
        label={props.label.clone()}
    > {props.children.clone()} </@> }
}
