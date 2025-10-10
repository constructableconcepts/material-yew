use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub label: AttrValue,
    #[prop_or_default]
    pub value: AttrValue,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub outlined: bool,
    #[prop_or_default]
    pub oninput: Callback<InputEvent>,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[function_component(TextField)]
pub fn textfield(props: &Props) -> Html {
    crate::import_material_web_module!("/md-web/textfield.js");

    let tag = if props.outlined {
        "md-outlined-text-field"
    } else {
        "md-filled-text-field"
    };

    html! {
        <@{tag}
            label={props.label.clone()}
            value={props.value.clone()}
            disabled={props.disabled}
            oninput={props.oninput.clone()}
            id={props.id.clone()}
            style={props.style.clone()}
        />
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gloo_utils::document;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn it_renders_with_custom_style_and_id() {
        let host = document().create_element("div").unwrap();
        let props = Props {
            label: "Label".into(),
            value: "Value".into(),
            disabled: false,
            outlined: false,
            oninput: Callback::default(),
            id: Some("custom-id".into()),
            style: Some("color: blue;".into()),
        };

        yew::Renderer::<TextField>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("id=\"custom-id\""));
        assert!(rendered_html.contains("style=\"color: blue;\""));
    }
}