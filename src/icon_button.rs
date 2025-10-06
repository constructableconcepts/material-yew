use js_sys::Reflect;
use wasm_bindgen::JsValue;
use web_sys::HtmlFormElement;
use yew::prelude::*;
#[derive(PartialEq, Clone)]
pub enum IconButtonVariants {
    Standard,
    Filled,
    FilledTonal,
    Outlined,
}

impl IconButtonVariants {
    fn as_tag_name(&self) -> &'static str {
        match self {
            IconButtonVariants::Standard => "md-icon-button",
            IconButtonVariants::Filled => "md-filled-icon-button",
            IconButtonVariants::FilledTonal => "md-filled-tonal-icon-button",
            IconButtonVariants::Outlined => "md-outlined-icon-button",
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    /// Disables the icon button and makes it non-interactive.
    #[prop_or_default]
    pub disabled: bool,
    /// Flips the icon if it is in an RTL context at startup.
    #[prop_or_default]
    pub flip_icon_in_rtl: bool,
    /// Sets the underlying <code>HTMLAnchorElement</code>’s <code>href</code> resource attribute.
    #[prop_or_default]
    pub href: Option<AttrValue>,
    /// Sets the underlying <code>HTMLAnchorElement</code>’s <code>target</code> attribute.
    #[prop_or_default]
    pub target: Option<AttrValue>,
    /// The <code>aria-label</code> of the button when the button is toggleable and selected.
    #[prop_or_default]
    pub aria_label_selected: Option<AttrValue>,
    /// When true, the button will toggle between selected and unselected states
    #[prop_or_default]
    pub toggle: bool,
    /// Sets the selected state. When false, displays the default icon. When true, displays the
    /// selected icon, or the default icon If no <code>slot=&quot;selected&quot;</code> icon is
    /// provided.
    #[prop_or_default]
    pub selected: bool,
    ///
    #[prop_or_default]
    pub typepe: Option<AttrValue>,
    ///
    #[prop_or_default]
    pub value: Option<AttrValue>,
    ///
    #[prop_or_default]
    pub name: Option<AttrValue>,
    #[prop_or_default]
    pub form: Option<HtmlFormElement>,
    /// The variant to use.
    pub variant: IconButtonVariants,
    #[prop_or_default]
    pub children: Html,

    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub oninput: Option<Callback<InputEvent>>,
    #[prop_or_default]
    pub onchange: Option<Callback<Event>>,
}

#[function_component]
pub fn IconButton(props: &Props) -> Html {
    let node_ref = use_node_ref();
    {
        let node_ref = node_ref.clone();
        let props = props.clone();
        use_effect_with(props, move |props| {
            let element = node_ref.get().unwrap();
            let form_value = props.form.as_ref().map(|f| f.into()).unwrap_or(JsValue::NULL);
            Reflect::set(&element, &"form".into(), &form_value).unwrap();

            let type_value = props.typepe.as_ref().map(|t| t.as_str().into()).unwrap_or(JsValue::NULL);
            Reflect::set(&element, &"type".into(), &type_value).unwrap();

            let value_value = props.value.as_ref().map(|v| v.as_str().into()).unwrap_or(JsValue::NULL);
            Reflect::set(&element, &"value".into(), &value_value).unwrap();

            let name_value = props.name.as_ref().map(|n| n.as_str().into()).unwrap_or(JsValue::NULL);
            Reflect::set(&element, &"name".into(), &name_value).unwrap();
            move || {}
        });
    }

    crate::import_material_web_module!("/md-web/icon-button.js");
    html! { <@{props.variant.as_tag_name()}
        ref={node_ref}
        disabled={props.disabled}
        flip-icon-in-rtl={props.flip_icon_in_rtl.then(|| AttrValue::from(""))}
        href={props.href.clone()}
        target={props.target.clone()}
        aria-label-selected={props.aria_label_selected.clone()}
        toggle={props.toggle.then(|| AttrValue::from(""))}
        selected={props.selected}
        onclick={props.onclick.clone()}
        oninput={props.oninput.clone()}
        onchange={props.onchange.clone()}
    > {props.children.clone()} </@> }
}
