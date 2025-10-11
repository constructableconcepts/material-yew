// ## TODOs
//
// ### Missing Components
// - [`md-chip-set`](https://material-web.dev/components/chip/#mdchipset-lessmd-chip-setgreater)
// - [`md-dialog`](https://material-web.dev/components/dialog/)
// - [`md-select`](https://material-web.dev/components/select/)
// - [`md-sub-menu` events](https://material-web.dev/components/menu/#events-2)
mod chip_set;
mod color;
mod divider;
mod field;
mod focus;
mod progress;
mod ripple;
mod select;
mod tabs;
mod tab;

pub use chip_set::ChipSet;
pub use color::Color;
pub use divider::Divider;
pub use field::Field;
pub use focus::Focus;
pub use progress::Progress;
pub use ripple::Ripple;
pub use select::Select;
pub use tabs::Tabs;
pub use tab::Tab;
mod button;
mod checkbox;
mod chip;
mod circular_progress;
mod fab;
mod icon_button;
mod linear_progress;
mod list;
pub mod form_element;
mod list_item;
mod menu;
mod menu_item;
mod radio;
mod slider;
mod sub_menu;
mod switch;
mod textfield;
mod elevation;
mod icon;
mod dialog;

pub use button::{Button, ButtonVariants};
pub use checkbox::Checkbox;
pub use chip::{AssistChip, FilterChip, InputChip, SuggestionChip};
pub use circular_progress::CircularProgress;
pub use fab::{Fab, FabStyle, FabVariant, FabSize};
pub use icon_button::{IconButton, IconButtonVariants};
pub use linear_progress::LinearProgress;
pub use list::List;
pub use list_item::ListItem;
pub use menu::Menu;
pub use menu_item::MenuItem;
pub use radio::Radio;
pub use slider::Slider;
pub use sub_menu::SubMenu;
pub use switch::Switch;
pub use textfield::TextField;
pub use dialog::Dialog;
pub use elevation::Elevation;
pub use icon::Icon;

macro_rules! import_material_web_module {
    ($module:literal) => {{
        #[wasm_bindgen::prelude::wasm_bindgen(module = $module)]
        extern "C" {
            #[wasm_bindgen(getter)]
            fn __dummy_loader() -> wasm_bindgen::JsValue;
        }

        #[allow(dead_code)]
        static LOADED: std::sync::Once = std::sync::Once::new();
        {
            LOADED.call_once(|| {
                __dummy_loader();
            });
        }
    }};
}

pub(crate) use import_material_web_module;

pub fn load_core() {
    import_material_web_module!("/md-web/core.js");
}

