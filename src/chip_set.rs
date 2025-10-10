use yew::prelude::*;

/// A component that displays a set of chips.
///
/// [Material Design Docs](https://m3.material.io/components/chips/overview)
#[derive(Properties, PartialEq, Clone)]
pub struct ChipSetProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[function_component(ChipSet)]
pub fn chip_set(props: &ChipSetProps) -> Html {
    html! {
        <md-chip-set id={props.id.clone()} style={props.style.clone()}>
            { for props.children.iter() }
        </md-chip-set>
    }
}