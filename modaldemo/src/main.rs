
use yew::prelude::*;
use material_yew::{Button, ButtonVariants, Dialog, TextField};
use wasm_logger;

#[function_component(App)]
fn app() -> Html {
    let dialog_open = use_state(|| false);

    let open_dialog = {
        let dialog_open = dialog_open.clone();
        Callback::from(move |_| dialog_open.set(true))
    };

    let close_dialog = {
        let dialog_open = dialog_open.clone();
        Callback::from(move |_| dialog_open.set(false))
    };

    html! {
        <div>
            <h1>{ "Modal Demo" }</h1>
            <Button variant={ButtonVariants::Filled} onclick={open_dialog}>{ "Save File" }</Button>

            <Dialog
                open={*dialog_open}
                headline={html!{ <h2 slot="headline">{ "Save File" }</h2> }}
                content={html!{
                    <div slot="content">
                        <p>{ "Enter a filename to save the file." }</p>
                        <TextField label="Filename" value="my-file.txt" />
                    </div>
                }}
                actions={html!{
                    <div slot="actions">
                        <Button variant={ButtonVariants::Text} onclick={close_dialog.clone()}>{ "Cancel" }</Button>
                        <Button variant={ButtonVariants::Text} onclick={close_dialog}>{ "Save" }</Button>
                    </div>
                }}
            />
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
