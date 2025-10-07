use yew::prelude::*;

/// The type of FAB tag to render.
#[derive(PartialEq, Clone, Default)]
pub enum FabType {
    #[default]
    Standard,
    Branded,
}

impl FabType {
    fn as_tag_name(&self) -> &'static str {
        match self {
            FabType::Standard => "md-fab",
            FabType::Branded => "md-branded-fab",
        }
    }
}

/// The color variant to render for the FAB.
#[derive(PartialEq, Clone, Default)]
pub enum FabVariant {
    #[default]
    Surface,
    Primary,
    Secondary,
    Tertiary,
}

impl FabVariant {
    fn as_str(&self) -> &'static str {
        match self {
            FabVariant::Surface => "surface",
            FabVariant::Primary => "primary",
            FabVariant::Secondary => "secondary",
            FabVariant::Tertiary => "tertiary",
        }
    }
}

/// The size of the FAB.
#[derive(PartialEq, Clone, Default)]
pub enum FabSize {
    #[default]
    Medium,
    Small,
    Large,
}

impl FabSize {
    fn as_str(&self) -> &'static str {
        match self {
            FabSize::Medium => "medium",
            FabSize::Small => "small",
            FabSize::Large => "large",
        }
    }
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
    pub label: Option<AttrValue>,
    /// Lowers the FAB’s elevation.
    #[prop_or_default]
    pub lowered: bool,
    /// The type of FAB tag to use.
    #[prop_or_default]
    pub fab_type: FabType,
    /// The icon to display in the FAB.
    #[prop_or_default]
    pub icon: Html,
}

#[function_component]
pub fn Fab(props: &Props) -> Html {
    crate::import_material_web_module!("/md-web/fab.js");
    html! { <@{props.fab_type.as_tag_name()}
        variant={props.variant.as_str()}
        size={props.size.as_str()}
        label={props.label.clone()}
        lowered={props.lowered.then(|| AttrValue::from(""))}
    >
        <span slot="icon">{props.icon.clone()}</span>
    </@> }
}