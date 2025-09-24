use yew::prelude::*;
use material_yew::*;

#[function_component(DemoPages)]
pub fn demo_pages() -> Html {
    html! {
        <div>
            <h2>{ "Button Demo" }</h2>
            <Button label="Click me!" />

            <h2>{ "Checkbox Demo" }</h2>
            <Checkbox checked={true} />

            <h2>{ "Chip Demo" }</h2>
            <Chip label="Demo Chip" />

            <h2>{ "FAB Demo" }</h2>
            <Fab icon="add" />

            <h2>{ "Icon Button Demo" }</h2>
            <IconButton icon="favorite" />

            <h2>{ "Linear Progress Demo" }</h2>
            <LinearProgress progress={0.5} />

            <h2>{ "List Demo" }</h2>
            <List>
                <ListItem>{ "Item 1" }</ListItem>
                <ListItem>{ "Item 2" }</ListItem>
            </List>

            <h2>{ "Menu Demo" }</h2>
            <Menu>
                <MenuItem>{ "Menu Item 1" }</MenuItem>
                <MenuItem>{ "Menu Item 2" }</MenuItem>
            </Menu>

            <h2>{ "Radio Demo" }</h2>
            <Radio checked={true} />

            <h2>{ "Slider Demo" }</h2>
            <Slider value={50} min={0} max={100} />

            <h2>{ "Switch Demo" }</h2>
            <Switch checked={true} />
        </div>
    }
}
