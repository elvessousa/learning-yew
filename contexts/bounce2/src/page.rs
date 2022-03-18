use crate::components::field::Field;
use crate::context::{Action, State};
use bounce::*;
use yew::prelude::*;

#[function_component(Page)]
pub fn page() -> Html {
    let state = use_slice::<State>();

    let onadd = {
        let state = state.clone();

        Callback::from(move |value: (String, String)| state.dispatch(Action::Add(value.0, value.1)))
    };

    let on_add_name = {
        let state = state.clone();

        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            state.dispatch(Action::ChangeName(input.value()))
        })
    };

    let on_add_address = {
        let state = state.clone();

        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            state.dispatch(Action::ChangeAddress(input.value()))
        })
    };

    html! {
        <section>
            <h1>{"Teum"}</h1>
            <p>{"Lorem ipsum dolor sit amet."}</p>
                 <pre>{format!("{:#?}", &state)}</pre>
            <Field name="name" onadd={onadd.clone()} />
            <Field name="teum" onadd={onadd.clone()} />
            <input oninput={on_add_name} />
            <input oninput={on_add_address} />
        </section>
    }
}
