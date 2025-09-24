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
        <md-tabs active-index={props.active_index.map(|i| i.to_string())}>
            { for props.children.iter() }
        </md-tabs>
    }
}
