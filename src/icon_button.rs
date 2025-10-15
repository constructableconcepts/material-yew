use yew::prelude::*;

/// The variants of the icon button.
#[derive(PartialEq, Clone)]
pub enum IconButtonVariants {
    /// The standard icon button.
    Standard,
    /// The filled icon button.
    Filled,
    /// The filled tonal icon button.
    FilledTonal,
    /// The outlined icon button.
    Outlined,
}

impl IconButtonVariants {
    fn as_tag_name(&self) -> &'static str {
        match self {
            IconButtonVariants::Standard => "md-icon-button",
            IconButtonVariants::Filled => "md-filled-icon-button",
            IconButtonVariants::FilledTonal => "md-filled-tonal-icon-button",
            IconButtonVariants::Outlined => "md-outlined-icon-button",
        }
    }
}

/// Properties for the `IconButton` component.
#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    /// Disables the icon button and makes it non-interactive.
    #[prop_or_default]
    pub disabled: bool,
    /// Flips the icon if it is in an RTL context at startup.
    #[prop_or_default]
    pub flip_icon_in_rtl: bool,
    /// Sets the underlying `HTMLAnchorElement`’s `href` resource attribute.
    #[prop_or_default]
    pub href: AttrValue,
    /// Sets the underlying `HTMLAnchorElement`’s `target` attribute.
    #[prop_or_default]
    pub target: AttrValue,
    /// The `aria-label` of the button when the button is toggleable and selected.
    #[prop_or_default]
    pub aria_label_selected: AttrValue,
    /// When true, the button will toggle between selected and unselected states
    #[prop_or_default]
    pub toggle: bool,
    /// Sets the selected state. When false, displays the default icon. When true, displays the
    /// selected icon, or the default icon If no `slot="selected"` icon is
    /// provided.
    #[prop_or_default]
    pub selected: bool,
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
    pub variant: IconButtonVariants,
    /// The content of the icon button.
    #[prop_or_default]
    pub children: Html,
    /// The callback to be called when the icon button is clicked.
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    /// The callback to be called when the icon button receives input.
    #[prop_or_default]
    pub oninput: Callback<InputEvent>,
    /// The callback to be called when the icon button's value changes.
    #[prop_or_default]
    pub onchange: Callback<Event>,
    /// The id of the icon button.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// The style of the icon button.
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// An icon button component.
///
/// [Material Design spec](https://m3.material.io/components/icon-buttons/overview)
#[function_component]
pub fn IconButton(props: &Props) -> Html {
    crate::import_material_web_module!("/md-web/icon-button.js");
    html! { <@{props.variant.as_tag_name()}
        disabled={props.disabled}
        flip-icon-in-rtl={props.flip_icon_in_rtl.then_some(AttrValue::from(""))}
        href={props.href.clone()}
        target={props.target.clone()}
        aria-label-selected={props.aria_label_selected.clone()}
        toggle={props.toggle.then_some(AttrValue::from(""))}
        selected={props.selected}
        type={props.r#type.clone()}
        value={props.value.clone()}
        name={props.name.clone()}
        form={props.form.clone()}
        onclick={props.onclick.clone()}
        oninput={props.oninput.clone()}
        onchange={props.onchange.clone()}
        id={props.id.clone()}
        style={props.style.clone()}
    > {props.children.clone()} </@> }
}