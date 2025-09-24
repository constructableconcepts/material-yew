use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub open: bool,
    #[prop_or_default]
    pub heading: Option<String>,
    #[prop_or_default]
    pub content: Option<String>,
    // #[prop_or_default]
    // pub onclosed: Option<Callback<()>>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Dialog)]
pub fn dialog(props: &Props) -> Html {
    html! {
        <md-dialog open={props.open} heading={props.heading.clone().unwrap_or_default()}>
            { for props.children.iter() }
        </md-dialog>
    }
}
