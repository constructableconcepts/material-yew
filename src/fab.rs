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

#[derive(Display, PartialEq, Clone, Copy, Default)]
#[strum(serialize_all = "lowercase")]
pub enum FabVariant {
    #[default]
    Surface,
    Primary,
    Secondary,
    Tertiary,
}

#[derive(Display, PartialEq, Clone, Copy, Default)]
#[strum(serialize_all = "lowercase")]
pub enum FabSize {
    Small,
    #[default]
    Medium,
    Large,
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
    pub label: AttrValue,
    /// Lowers the FAB’s elevation.
    #[prop_or_default]
    pub lowered: bool,
    /// The style of the FAB to use.
    pub fab_style: FabStyle,
    /// Disables the FAB.
    #[prop_or_default]
    pub disabled: bool,
    /// The icon to display in the FAB.
    #[prop_or_default]
    pub icon: Html,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[function_component]
pub fn Fab(props: &Props) -> Html {
    crate::import_material_web_module!("/md-web/fab.js");

    html! { <@{props.fab_style.as_tag_name()}
        variant={props.variant.to_string()}
        size={props.size.to_string()}
        label={props.label.clone()}
        lowered={props.lowered.then_some(AttrValue::from(""))}
        disabled={props.disabled}
        id={props.id.clone()}
        style={props.style.clone()}
    >
        <span slot="icon">{ props.icon.clone() }</span>
    </@> }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Icon;
    use gloo_utils::document;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn it_renders_with_enums() {
        let host = document().create_element("div").unwrap();
        let props = Props {
            variant: FabVariant::Primary,
            size: FabSize::Large,
            label: AttrValue::default(),
            lowered: false,
            fab_style: FabStyle::Standard,
            disabled: false,
            icon: html! {},
            id: None,
            style: None,
        };

        yew::Renderer::<Fab>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("variant=\"primary\""));
        assert!(rendered_html.contains("size=\"large\""));
    }

    #[wasm_bindgen_test]
    fn it_renders_icon_slot() {
        let host = document().create_element("div").unwrap();
        let props = Props {
            variant: FabVariant::default(),
            size: FabSize::default(),
            label: AttrValue::default(),
            lowered: false,
            fab_style: FabStyle::Standard,
            disabled: false,
            icon: html! { <Icon icon={"star".to_string()} /> },
            id: None,
            style: None,
        };

        yew::Renderer::<Fab>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("<span slot=\"icon\">"));
        assert!(rendered_html.contains("<md-icon>star</md-icon>"));
        assert!(rendered_html.contains("</span>"));
    }

    #[wasm_bindgen_test]
    fn it_renders_with_custom_style_and_id() {
        let host = document().create_element("div").unwrap();
        let props = Props {
            variant: FabVariant::default(),
            size: FabSize::default(),
            label: AttrValue::default(),
            lowered: false,
            fab_style: FabStyle::Standard,
            disabled: false,
            icon: html! {},
            id: Some("custom-id".into()),
            style: Some("color: purple;".into()),
        };

        yew::Renderer::<Fab>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("id=\"custom-id\""));
        assert!(rendered_html.contains("style=\"color: purple;\""));
    }

    #[wasm_bindgen_test]
    fn it_renders_as_disabled() {
        let host = document().create_element("div").unwrap();
        let props = Props {
            variant: FabVariant::default(),
            size: FabSize::default(),
            label: AttrValue::default(),
            lowered: false,
            fab_style: FabStyle::Standard,
            disabled: true,
            icon: html! {},
            id: None,
            style: None,
        };

        yew::Renderer::<Fab>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("disabled"));
    }
}