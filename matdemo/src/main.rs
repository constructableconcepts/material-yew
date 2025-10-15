use yew::prelude::*;
use log::info;

mod pages;

#[function_component(App)]
fn app() -> Html {
    info!("Rendering App component");
    html! {
        <>
            <h1>{ "Material Yew Component Demos" }</h1>
            <pages::DemoPages />
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();
    info!("Starting matdemo application");
    let renderer = yew::Renderer::<App>::new().render();
    std::mem::forget(renderer);
}