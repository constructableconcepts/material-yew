use strum::Display;
use yew::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub enum FabStyle {
    Standard,
    Branded,
}

impl FabStyle {
    fn as_tag_name(&self) -> &'static str {
        match self {
            FabStyle::Standard => "md-fab",
            FabStyle::Branded => "md-branded-fab",
        }
    }
}

#[derive(Display, PartialEq, Clone, Copy)]
#[strum(serialize_all = "lowercase")]
pub enum FabVariant {
    Surface,
    Primary,
    Secondary,
    Tertiary,
}

#[derive(Display, PartialEq, Clone, Copy)]
#[strum(serialize_all = "lowercase")]
pub enum FabSize {
    Small,
    Medium,
    Large,
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
    /// The style of the FAB to use.
    pub style: FabStyle,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn Fab(props: &Props) -> Html {
    crate::import_material_web_module!("/md-web/fab.js");
    html! { <@{props.style.as_tag_name()}
        variant={props.variant.as_ref().map(|v| v.to_string())}
        size={props.size.as_ref().map(|s| s.to_string())}
        label={props.label.clone()}
        lowered={props.lowered.then(|| AttrValue::from(""))}
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
    fn it_renders_with_enums() {
        let host = document().create_element("div").unwrap();
        let props = Props {
            variant: Some(FabVariant::Primary),
            size: Some(FabSize::Large),
            label: None,
            lowered: false,
            style: FabStyle::Standard,
            children: html! {},
        };

        yew::Renderer::<Fab>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("variant=\"primary\""));
        assert!(rendered_html.contains("size=\"large\""));
    }
}
