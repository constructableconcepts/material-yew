use yew::prelude::*;

mod pages;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{ "Material Yew Component Demos" }</h1>
            <pages::DemoPages />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
