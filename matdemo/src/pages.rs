use yew::prelude::*;
use material_yew::*;
use log::info;

#[function_component(DemoPages)]
pub fn demo_pages() -> Html {
    info!("Rendering DemoPages component");
    html! {
        <div>
            <h2>{ "Button" }</h2>
            <Button variant={ButtonVariants::Filled}>{ "Click me!" }</Button>

            <h2>{ "Checkbox" }</h2>
            <Checkbox />

            <h2>{ "Chip" }</h2>
            <Chip variant={ChipVariants::Assist}>{ "Demo Chip" }</Chip>

            <h2>{ "Chip Set" }</h2>
            <ChipSet>
                <Chip variant={ChipVariants::Assist}>{ "Chip 1" }</Chip>
                <Chip variant={ChipVariants::Assist}>{ "Chip 2" }</Chip>
            </ChipSet>

            <h2>{ "Circular Progress" }</h2>
            <CircularProgress value={50.0} max={100.0} />

            <h2>{ "Color" }</h2>
            <Color value={"#2196f3"} />

            <h2>{ "Dialog" }</h2>
            <Dialog
                headline={html!{ <><h2>{"Dialog"}</h2></> }}
                content={html!{ <><p>{"A standard dialog."}</p></> }}
                actions={html!{ <><Button variant={ButtonVariants::Text}>{ "Close" }</Button></> }}
            />

            <h2>{ "Divider" }</h2>
            <Divider />

            <h2>{ "Elevation" }</h2>
            <Elevation />

            <h2>{ "FAB" }</h2>
            <Fab style={FabStyle::Standard} label={"Add"} icon={html!{<Icon icon={"add"} />}} />

            <h2>{ "Field" }</h2>
            <Field label={"Field Label"}>{ "Field content" }</Field>

            <h2>{ "Focus" }</h2>
            <Focus>{ "Focusable content" }</Focus>

            <h2>{ "Icon" }</h2>
            <Icon icon={"star"} />

            <h2>{ "Icon Button" }</h2>
            <IconButton variant={IconButtonVariants::Standard}>{ "favorite" }</IconButton>

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
            <Menu>
                <MenuItem>{ "Menu Item 1" }</MenuItem>
                <MenuItem>{ "Menu Item 2" }</MenuItem>
            </Menu>

            <h2>{ "Menu Item" }</h2>
            <MenuItem>{ "Standalone Menu Item" }</MenuItem>

            <h2>{ "Progress" }</h2>
            <Progress />

            <h2>{ "Radio" }</h2>
            <Radio />

            <h2>{ "Ripple" }</h2>
            <Ripple>{ "Rippled content" }</Ripple>

            <h2>{ "Select" }</h2>
            <Select label={"Select Label"}>
                <MenuItem>{ "Option 1" }</MenuItem>
                <MenuItem>{ "Option 2" }</MenuItem>
            </Select>

            <h2>{ "Slider" }</h2>
            <Slider />

            <h2>{ "Sub Menu" }</h2>
            <SubMenu>{ "Sub Menu Content" }</SubMenu>

            <h2>{ "Switch" }</h2>
            <Switch selected={true} />

            <h2>{ "Tabs" }</h2>
            <Tabs>{ "Tab content" }</Tabs>

            <h2>{ "TextField" }</h2>
            <TextField label={"Text Field"} />
        </div>
    }
}