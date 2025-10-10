use std::collections::BTreeMap;
use yew::prelude::*;

/// A struct to hold customizable properties for a component.
#[derive(Properties, PartialEq, Clone, Default)]
pub struct CustomizableProps {
    /// The `id` attribute for the component.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// The `style` attribute for the component.
    #[prop_or_default]
    pub style: Option<AttrValue>,
    /// A map of `aria-*` attributes to apply to the component.
    #[prop_or_default]
    pub aria: Option<BTreeMap<String, AttrValue>>,
}