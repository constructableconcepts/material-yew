use yew::prelude::*;

/// Properties for the `Icon` component.
#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    /// The icon to display.
    #[prop_or_default]
    pub icon: AttrValue,
    /// The id of the icon.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// The style of the icon.
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// An icon component.
///
/// [Material Design spec](https://m3.material.io/styles/icons/overview)
#[function_component(Icon)]
pub fn icon(props: &Props) -> Html {
    html! {
        <md-icon id={props.id.clone()} style={props.style.clone()}>{ props.icon.clone() }</md-icon>
    }
}