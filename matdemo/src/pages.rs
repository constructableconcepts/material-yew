use yew::prelude::*;
use material_yew::*;
use log::info;

#[function_component(DemoPages)]
pub fn demo_pages() -> Html {
    info!("Rendering DemoPages component");

    let dialog_open = use_state(|| false);
    let open_dialog = {
        let dialog_open = dialog_open.clone();
        Callback::from(move |_| dialog_open.set(true))
    };
    let close_dialog = {
        let dialog_open = dialog_open.clone();
        Callback::from(move |_| dialog_open.set(false))
    };

    let menu_open = use_state(|| false);
    let toggle_menu = {
        let menu_open = menu_open.clone();
        Callback::from(move |_| menu_open.set(!*menu_open))
    };

    html! {
        <div style="--md-sys-color-primary: #2196f3;">
            <h2>{ "Button" }</h2>
            <Button variant={ButtonVariants::Filled}>{ "Click me!" }</Button>

            <h2>{ "Checkbox" }</h2>
            <Checkbox />

            <h2>{ "Chip" }</h2>
            <AssistChip>{ "Demo Chip" }</AssistChip>

            <h2>{ "Chip Set" }</h2>
            <ChipSet>
                <AssistChip>{ "Chip 1" }</AssistChip>
                <AssistChip>{ "Chip 2" }</AssistChip>
            </ChipSet>

            <h2>{ "Circular Progress" }</h2>
            <CircularProgress value={0.5} />

            <h2>{ "Dialog" }</h2>
            <>
                <Button variant={ButtonVariants::Filled} onclick={open_dialog}>{ "Open Dialog" }</Button>
                <Dialog
                    open={*dialog_open}
                    headline={html!{ <h2 slot="headline">{"Dialog"}</h2> }}
                    content={html!{ <p slot="content">{"A standard dialog."}</p> }}
                    actions={html!{
                        <div slot="actions">
                            <Button variant={ButtonVariants::Text} onclick={close_dialog}>{ "Close" }</Button>
                        </div>
                    }}
                />
            </>

            <h2>{ "Divider" }</h2>
            <Divider />

            <h2>{ "Elevation" }</h2>
            <Elevation />

            <h2>{ "FAB" }</h2>
            <Fab fab_style={FabStyle::Standard} variant={FabVariant::Primary} label="Add" icon={html!{ <Icon icon="add" /> }} />

            <h2>{ "Field" }</h2>
            <Field label="Field Label">{ "Field content" }</Field>

            <h2>{ "Icon" }</h2>
            <Icon icon="star" />

            <h2>{ "Icon Button" }</h2>
            <IconButton variant={IconButtonVariants::Standard}>{ html!{ <Icon icon="favorite" /> } }</IconButton>

            <h2>{ "Linear Progress" }</h2>
            <LinearProgress />

            <h2>{ "List" }</h2>
            <List>
                <ListItem>{ "Item 1" }</ListItem>
                <ListItem>{ "Item 2" }</ListItem>
            </List>

            <h2>{ "List Item" }</h2>
            <ListItem>{ "Standalone List Item" }</ListItem>

            <h2>{ "Menu" }</h2>
            <div style="position: relative;">
                <Button
                    variant={ButtonVariants::Outlined}
                    id="menu-anchor"
                    onclick={toggle_menu.clone()}
                >{ "Open Menu" }</Button>
                <Menu anchor="menu-anchor" open={*menu_open}>
                    <MenuItem>{ "Menu Item 1" }</MenuItem>
                    <MenuItem>{ "Menu Item 2" }</MenuItem>
                </Menu>
            </div>

            <h2>{ "Menu Item" }</h2>
            <MenuItem>{ "Standalone Menu Item" }</MenuItem>

            <h2>{ "Radio" }</h2>
            <Radio />

            <h2>{ "Ripple" }</h2>
            <div style="position: relative; width: 200px; height: 50px; border: 1px solid black; display: flex; align-items: center; justify-content: center;">
                <Ripple>{ "Rippled content" }</Ripple>
            </div>

            <h2>{ "Select" }</h2>
            <Select label="Select Label">
                <MenuItem value="1">{ "Option 1" }</MenuItem>
                <MenuItem value="2">{ "Option 2" }</MenuItem>
            </Select>

            <h2>{ "Slider" }</h2>
            <Slider />

            <h2>{ "Switch" }</h2>
            <Switch selected={true} />

            <h2>{ "Tabs" }</h2>
            <Tabs>
                <Tab>{ "Tab 1" }</Tab>
                <Tab>{ "Tab 2" }</Tab>
            </Tabs>

            <h2>{ "TextField" }</h2>
            <TextField label="Text Field" />
        </div>
    }
}