use bounce::*;
use std::collections::HashMap;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Debug, Default, Clone, Slice, PartialEq)]
pub struct State {
    pub entries: HashMap<String, String>,
    pub name: String,
    pub address: String,
}

pub enum Action {
    Add(String, String),
    ChangeName(String),
    ChangeAddress(String),
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::Add(name, value) => {
                let mut entries = self.entries.clone();
                let other_fields = &*self.clone();
                entries.insert(name, value);

                Self {
                    entries,
                    ..other_fields.clone()
                }
                .into()
            }
            Action::ChangeName(name) => {
                let other_fields = &*self.clone();

                Self {
                    name,
                    ..other_fields.clone()
                }
                .into()
            }
            Action::ChangeAddress(address) => {
                let other_fields = &*self.clone();

                Self {
                    address,
                    ..other_fields.clone()
                }
                .into()
            }
        }
    }
}
