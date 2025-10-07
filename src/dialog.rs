use yew::prelude::*;

/// The Dialog component.
///
/// [Material Design Docs](https://m3.material.io/components/dialogs/overview)
///
/// Dialogs provide important prompts in a user flow. They can require users
/// to make a decision, or they can present multiple tasks without interrupting
/// the current flow.
#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    /// Opens the dialog when set to `true` and closes it when set to `false`.
    #[prop_or_default]
    pub open: bool,

    /// Gets or sets the dialog's return value, usually to indicate which button
    /// a user pressed to close it.
    ///
    /// https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/returnValue
    #[prop_or_default]
    pub return_value: Option<AttrValue>,

    /// Skips the opening and closing animations.
    #[prop_or_default]
    pub quick: bool,

    /// The type of dialog for accessibility. Set this to `alert` to announce a
    /// dialog as an alert dialog.
    #[prop_or_default]
    pub r#type: Option<AttrValue>,

    /// Disables focus trapping, which by default keeps keyboard Tab navigation
    /// within the dialog.
    #[prop_or_default]
    pub no_focus_trap: bool,

    /// The content to be rendered inside the dialog. This should include
    /// elements with `slot="headline"`, `slot="content"`, and `slot="actions"`.
    #[prop_or_default]
    pub children: Children,
    // TODO: Add callbacks for `open`, `opened`, `close`, `closed`, `cancel` events.
}

/// A dialog component.
#[function_component(Dialog)]
pub fn dialog(props: &Props) -> Html {
    crate::import_material_web_module!("/md-web/dialog.js");
    html! {
        <md-dialog
            open={props.open}
            returnValue={props.return_value.clone()}
            quick={props.quick.then(|| AttrValue::from(""))}
            type={props.r#type.clone()}
            no-focus-trap={props.no_focus_trap.then(|| AttrValue::from(""))}
        >
            { for props.children.iter() }
        </md-dialog>
    }
}