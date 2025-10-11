use crate::form_element::FormElementRef;
use yew::prelude::*;

/// A handle to imperatively control the Select component.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SelectRef {
    form_element_ref: FormElementRef,
}

impl SelectRef {
    /// Checks the select's validity.
    pub fn check_validity(&self) -> bool {
        self.form_element_ref.check_validity()
    }

    /// Checks the select's validity and reports it to the user.
    pub fn report_validity(&self) -> bool {
        self.form_element_ref.report_validity()
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub label: AttrValue,
    #[prop_or_default]
    pub value: AttrValue,
    #[prop_or_default]
    pub disabled: bool,
    /// A handle to allow imperative control of the select.
    #[prop_or_default]
    pub select_ref: SelectRef,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[function_component(Select)]
pub fn select(props: &Props) -> Html {
    let node_ref = props.select_ref.form_element_ref.node_ref.clone();
    html! {
        <md-select
            ref={node_ref}
            label={props.label.clone()}
            value={props.value.clone()}
            disabled={props.disabled}
            id={props.id.clone()}
            style={props.style.clone()}
        >
            { for props.children.iter() }
        </md-select>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list_item::ListItem;
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
            select_ref: SelectRef::default(),
            children: Children::new(vec![]),
            id: Some("custom-id".into()),
            style: Some("color: purple;".into()),
        };

        yew::Renderer::<Select>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("id=\"custom-id\""));
        assert!(rendered_html.contains("style=\"color: purple;\""));
    }

    #[wasm_bindgen_test]
    fn it_handles_validation() {
        let select_ref = SelectRef::default();
        let host = document().create_element("div").unwrap();
        let props = Props {
            label: "Label".into(),
            value: "Value".into(),
            disabled: false,
            select_ref: select_ref.clone(),
            children: Children::new(vec![html! { <ListItem>{"Item"}</ListItem> }]),
            id: None,
            style: None,
        };

        yew::Renderer::<Select>::with_root_and_props(host.clone(), props).render();

        assert!(select_ref.check_validity());
    }
}