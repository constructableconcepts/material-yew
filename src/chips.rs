use yew::prelude::*;

/// A chip set is a collection of chips.
#[derive(Properties, PartialEq)]
pub struct Props {
    /// The chips to display in the set.
    #[prop_or_default]
    pub children: Children,
}

/// A chip set that contains a collection of chips.
///
/// [Material Design spec](https://m3.material.io/components/chips/overview)
///
/// The chip set is a wrapper around the `<md-chip-set>` custom element.
#[function_component(Chips)]
pub fn chip_set(props: &Props) -> Html {
    crate::import_material_web_module!("/md-web/chips.js");
    html! {
        <md-chip-set>
            { for props.children.iter() }
        </md-chip-set>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Chip, ChipVariants}; // Import the Chip component and variants to use in the test
    use gloo_utils::document;
    use wasm_bindgen_test::*;
    use yew::prelude::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn it_renders_children() {
        let host = document().create_element("div").unwrap();
        let props = Props {
            children: Children::new(vec![html! { <Chip label={"Test Chip"} variant={ChipVariants::Assist} /> }]),
        };

        yew::Renderer::<Chips>::with_root_and_props(host.clone(), props).render();

        let rendered_html = host.inner_html();
        assert!(rendered_html.contains("<md-chip-set>"));
        assert!(rendered_html.contains("label=\"Test Chip\""));
        assert!(rendered_html.contains("</md-chip-set>"));
    }
}