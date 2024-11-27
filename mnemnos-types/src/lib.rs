use std::collections::HashMap;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ValueName(pub String);

impl From<String> for ValueName {
    fn from(value: String) -> Self {
        ValueName(value)
    }
}

impl<'a> From<&'a str> for ValueName {
    fn from(value: &'a str) -> Self {
        ValueName(value.into())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PageName(pub String);

impl From<String> for PageName {
    fn from(value: String) -> Self {
        PageName(value)
    }
}

impl<'a> From<&'a str> for PageName {
    fn from(value: &'a str) -> Self {
        PageName(value.into())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct AppState {
    pub pages: HashMap<PageName, Page>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Page {
    pub template: String,
    pub values: HashMap<ValueName, serde_json::Value>,
}

pub enum Action {
    AddPage {
        page_name: PageName,
        page: Page,
    },
    RemovePage {
        page_name: PageName,
    },
    EditTemplate {
        page_name: PageName,
        template: String,
    },
    AddValue {
        page_name: PageName,
        value_name: ValueName,
        value: serde_json::Value,
    },
    RemoveValue {
        page_name: PageName,
        value_name: ValueName,
    },
    EditValue {
        page_name: PageName,
        value_name: ValueName,
        value: serde_json::Value,
    },
}

impl Reducible for AppState {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::AddPage { page_name, page } => {
                let mut pages = self.pages.clone();
                pages.insert(page_name, page);
                Self { pages }.into()
            }
            Action::RemovePage { page_name } => {
                let mut pages = self.pages.clone();
                pages.remove(&page_name);
                Self { pages }.into()
            }
            Action::EditTemplate {
                page_name,
                template,
            } => {
                let mut pages = self.pages.clone();
                pages
                    .entry(page_name)
                    .and_modify(move |p| p.template = template);
                Self { pages }.into()
            }
            Action::AddValue {
                page_name,
                value_name,
                value,
            } => {
                let mut pages = self.pages.clone();
                pages.entry(page_name).and_modify(move |p| {
                    p.values.insert(value_name, value);
                });
                Self { pages }.into()
            }
            Action::RemoveValue {
                page_name,
                value_name,
            } => {
                let mut pages = self.pages.clone();
                pages.entry(page_name).and_modify(move |p| {
                    p.values.remove(&value_name);
                });
                Self { pages }.into()
            }
            Action::EditValue {
                page_name,
                value_name,
                value,
            } => {
                let mut pages = self.pages.clone();
                pages.entry(page_name).and_modify(move |p| {
                    p.values.entry(value_name).and_modify(|v| *v = value);
                });
                Self { pages }.into()
            }
        }
    }
}
