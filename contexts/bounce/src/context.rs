use bounce::*;
use std::collections::HashMap;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Debug, Default, Clone, Slice, PartialEq)]
pub struct State {
    pub entries: HashMap<String, String>,
}

pub enum Action {
    Add(String, String),
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::Add(name, value) => {
                let mut entries = self.entries.clone();
                entries.insert(name, value);

                Self { entries }.into()
            }
        }
    }
}
