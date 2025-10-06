use yew::prelude::*;

#[derive(PartialEq)]
pub enum FabVariants {
    Standard,
    Branded,
}

impl FabVariants {
    fn as_tag_name(&self) -> &'static str {
        match self {
            FabVariants::Standard => "md-fab",
            FabVariants::Branded => "md-branded-fab",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    /// The FAB color variant to render.
    #[prop_or_default]
    pub kind: Option<AttrValue>,
    /// The size of the FAB.<br>NOTE: Branded FABs cannot be sized to <code>small</code>, and
    /// Extended FABs do not have different sizes.
    #[prop_or_default]
    pub size: Option<AttrValue>,
    /// The text to display on the FAB.
    #[prop_or_default]
    pub label: Option<AttrValue>,
    /// Lowers the FAB’s elevation.
    #[prop_or_default]
    pub lowered: bool,
    /// The variant to use.
    pub variant: FabVariants,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn Fab(props: &Props) -> Html {
    crate::import_material_web_module!("/md-web/fab.js");
    html! { <@{props.variant.as_tag_name()}
        variant={props.kind.clone()}
        size={props.size.clone()}
        label={props.label.clone()}
        lowered={props.lowered.then(|| AttrValue::from(""))}
    > {props.children.clone()} </@> }
}
