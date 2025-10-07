use yew::prelude::*;

/// A component for elevation.
///
/// Elevation is controlled by applying the `--md-elevation-level` CSS custom
/// property to the parent container.
#[derive(Properties, PartialEq, Clone)]
pub struct Props {}

#[function_component(Elevation)]
pub fn elevation(_props: &Props) -> Html {
    crate::import_material_web_module!("/md-web/elevation.js");
    html! {
        <md-elevation />
    }
}