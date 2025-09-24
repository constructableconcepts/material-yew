use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub active_index: Option<usize>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Tabs)]
pub fn tabs(props: &Props) -> Html {
    html! {
        <md-tabs active_index={props.active_index.unwrap_or(0)}>
            { for props.children.iter() }
        </md-tabs>
    }
}
