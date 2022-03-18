use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct FieldProps {
    pub onadd: Callback<(String, String)>,
    pub name: String,
}

#[function_component(Field)]
pub fn field(props: &FieldProps) -> Html {
    let onkeypress = {
        let onadd = props.onadd.clone();
        let name = props.name.clone();

        move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            onadd.emit((name.clone(), value.clone()))
        }
    };

    html! {
        <input oninput={onkeypress} />
    }
}
