use yew::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub enum FabType {
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

#[derive(PartialEq, Clone, Copy, Default)]
pub enum FabVariant {
    #[default]
    Surface,
    Primary,
    Secondary,
    Tertiary,
}

impl From<FabVariant> for AttrValue {
    fn from(val: FabVariant) -> Self {
        AttrValue::from(match val {
            FabVariant::Surface => "surface",
            FabVariant::Primary => "primary",
            FabVariant::Secondary => "secondary",
            FabVariant::Tertiary => "tertiary",
        })
    }
}

#[derive(PartialEq, Clone, Copy, Default)]
pub enum FabSize {
    #[default]
    Medium,
    Small,
    Large,
}

impl From<FabSize> for AttrValue {
    fn from(val: FabSize) -> Self {
        AttrValue::from(match val {
            FabSize::Small => "small",
            FabSize::Medium => "medium",
            FabSize::Large => "large",
        })
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    /// The FAB color variant to render.
    #[prop_or_default]
    pub variant: Option<FabVariant>,
    /// The size of the FAB.<br>NOTE: Branded FABs cannot be sized to <code>small</code>, and
    /// Extended FABs do not have different sizes.
    #[prop_or_default]
    pub size: Option<FabSize>,
    /// The text to display on the FAB.
    #[prop_or_default]
    pub label: Option<AttrValue>,
    /// Lowers the FAB’s elevation.
    #[prop_or_default]
    pub lowered: bool,
    /// The type of FAB to use.
    pub fab_type: FabType,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn Fab(props: &Props) -> Html {
    crate::import_material_web_module!("/md-web/fab.js");
    html! { <@{props.fab_type.as_tag_name()}
        variant={props.variant.map(|v| AttrValue::from(v))}
        size={props.size.map(|v| AttrValue::from(v))}
        label={props.label.clone()}
        lowered={props.lowered.then(|| AttrValue::from(""))}
    > {props.children.clone()} </@> }
}
