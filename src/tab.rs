use yew::prelude::*;

/// Properties for the `Tab` component.
#[derive(Properties, PartialEq, Default)]
pub struct Props {
    /// Whether or not the tab is disabled.
    #[prop_or_default]
    pub disabled: bool,
    /// Whether or not the tab is active.
    #[prop_or_default]
    pub active: bool,
    /// The content of the tab.
    #[prop_or_default]
    pub children: Children,
    /// The id of the tab.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// The style of the tab.
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// A tab component.
///
/// [Material Design spec](https://m3.material.io/components/tabs/overview)
#[function_component]
pub fn Tab(props: &Props) -> Html {
    html! {
        <md-tab
            disabled={props.disabled}
            active={props.active.then_some(AttrValue::from(""))}
            id={props.id.clone()}
            style={props.style.clone()}
        >
            { for props.children.iter() }
        </md-tab>
    }
}