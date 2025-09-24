
use yew::prelude::*;
use material_yew::*;

#[function_component(DemoPages)]
pub fn demo_pages() -> Html {
    html! {
        <div>
            <h2>{ "Button" }</h2>
            <Button variant={ButtonVariants::Filled}>{ "Click me!" }</Button>

            <h2>{ "Checkbox" }</h2>
            <Checkbox />

            <h2>{ "Chip" }</h2>
            <Chip variant={ChipVariants::Assist} label={"Demo Chip"} />

            <h2>{ "Chips" }</h2>
            <Chips label={Some("Chips Group".to_string())} />

            <h2>{ "Circular Progress" }</h2>
            <CircularProgress value={50} max={100} />

            <h2>{ "Color" }</h2>
            <Color value={Some("#2196f3".to_string())} />

            <h2>{ "Dialog" }</h2>
            <Dialog open={false} heading={Some("Dialog Heading".to_string())}>{ "Dialog content here." }</Dialog>

            <h2>{ "Divider" }</h2>
            <Divider />

            <h2>{ "Elevation" }</h2>
            <Elevation level={Some(3)} />

            <h2>{ "FAB" }</h2>
            <Fab variant={FabVariants::Standard} label={"Add"} />

            <h2>{ "Field" }</h2>
            <Field label={Some("Field Label".to_string())}>{ "Field content" }</Field>

            <h2>{ "Focus" }</h2>
            <Focus>{ "Focusable content" }</Focus>

            <h2>{ "Icon" }</h2>
            <Icon icon={Some("star".to_string())} />

            <h2>{ "Icon Button" }</h2>
            <IconButton variant={IconButtonVariants::Standard}>{ "favorite" }</IconButton>

            <h2>{ "Linear Progress" }</h2>
            <LinearProgress value={50} max={100} />

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
            <Progress progress={Some(0.7)} />

            <h2>{ "Radio" }</h2>
            <Radio />

            <h2>{ "Ripple" }</h2>
            <Ripple>{ "Rippled content" }</Ripple>

            <h2>{ "Select" }</h2>
            <Select label={Some("Select Label".to_string())}>
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
            <Tabs active_index={Some(0)}>{ "Tab content" }</Tabs>

            <h2>{ "TextField" }</h2>
            <TextField label={Some("Text Field".to_string())} />
        </div>
    }
}
