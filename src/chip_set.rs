use yew::prelude::*;

/// Properties for the `ChipSet` component.
#[derive(Properties, PartialEq, Clone)]
pub struct ChipSetProps {
    /// The chips to display in the set.
    #[prop_or_default]
    pub children: Children,
    /// The id of the chip set.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// The style of the chip set.
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// A component that displays a set of chips.
///
/// [Material Design Docs](https://m3.material.io/components/chips/overview)
#[function_component(ChipSet)]
pub fn chip_set(props: &ChipSetProps) -> Html {
    crate::import_material_web_module!("/md-web/chips.js");

    html! {
        <md-chip-set id={props.id.clone()} style={props.style.clone()}>
            { for props.children.iter() }
        </md-chip-set>
    }
}