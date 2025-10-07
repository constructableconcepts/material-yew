use yew::prelude::*;

/// A chip set component that can contain a set of chips.
///
/// It manages focus and keyboard navigation for the chips within it.
#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    /// The `Chip` components to be rendered within the set.
    #[prop_or_default]
    pub children: Children,
}

/// A chip set component. This is a container for `Chip` components.
#[function_component(ChipSet)]
pub fn chip_set(props: &Props) -> Html {
    // The chip set JS module is required for keyboard navigation and focus management.
    crate::import_material_web_module!("/md-web/chips.js");
    html! {
        <md-chip-set>
            { for props.children.iter() }
        </md-chip-set>
    }
}