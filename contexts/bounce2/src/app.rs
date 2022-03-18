use crate::page::Page;
use bounce::*;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BounceRoot>
            <Page />
        </BounceRoot>
    }
}
