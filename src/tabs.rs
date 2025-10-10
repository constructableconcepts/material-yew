use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or(0)]
    pub active_index: u32,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Tabs)]
pub fn tabs(props: &Props) -> Html {
    html! {
        <md-tabs active-index={props.active_index.to_string()}>
            { for props.children.iter() }
        </md-tabs>
    }
}