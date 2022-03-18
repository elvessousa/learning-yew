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

    html! {
        <section>
            <h1>{"Teum"}</h1>
            <p>{"Lorem ipsum dolor sit amet."}</p>
                 <pre>{format!("{:#?}", &state)}</pre>
            <Field name="name" onadd={onadd.clone()} />
            <Field name="teum" onadd={onadd.clone()} />
        </section>
    }
}
