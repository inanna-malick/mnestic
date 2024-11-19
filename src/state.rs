use std::collections::HashMap;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};
use yew::html::IntoPropValue;
use yew::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct State {
    pub pages: HashMap<String, Page>,
}


#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Page {
    pub template: String,
    pub values: HashMap<String, serde_json::Value>,
    pub visible: bool,
}


#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Entry {
    pub id: usize,
    pub description: String,
    pub completed: bool,
}

pub enum Action {
    AddPage{page_name: String, template: String, values: HashMap<String, serde_json::Value>},
    RemovePage{page_name: String},
    EditTemplate{page_name: String, template: String},
    AddValue{page_name: String, value_name: String, value: serde_json::Value},
    RemoveValue{page_name: String, value_name: String},
    EditValue{page_name: String, value_name: String, value: serde_json::Value},
    HideAll,
    ShowAll,
    ToggleVisible{page_name: String},
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {


        match action {
            Action::AddPage { page_name, template, values } => {
                let mut pages = self.pages.clone();
                pages.insert(page_name, Page { template, values, visible: true});
                Self {
                    pages
                }.into()
            },
            Action::RemovePage { page_name } => {
                let mut pages = self.pages.clone();
                pages.remove(&page_name);
                Self {
                    pages
                }.into()
            },
            Action::EditTemplate { page_name, template } => {
                let mut pages = self.pages.clone();
                pages.entry(page_name).and_modify(move |p| p.template = template );
                Self {
                    pages
                }.into()
            },
            Action::AddValue { page_name, value_name, value } => {
                let mut pages = self.pages.clone();
                pages.entry(page_name).and_modify(move |p| {
                    p.values.insert(value_name, value);
                } );
                Self {
                    pages
                }.into()
            },
            Action::RemoveValue { page_name, value_name } => {
                let mut pages = self.pages.clone();
                pages.entry(page_name).and_modify(move |p| {
                    p.values.remove(&value_name);
                } );
                Self {
                    pages
                }.into()
            },
            Action::EditValue { page_name, value_name, value } => {
                let mut pages = self.pages.clone();
                pages.entry(page_name).and_modify(move |p| {
                    p.values.entry(value_name).and_modify(|v| *v = value);
                } );
                Self {
                    pages
                }.into()
            },
            Action::HideAll => {
                let mut pages = self.pages.clone();
                for (_page_name, page) in pages.iter_mut() {
                    page.visible = false;
                }
                Self {
                    pages
                }.into()
            },
            Action::ShowAll => {
                let mut pages = self.pages.clone();
                for (_page_name, page) in pages.iter_mut() {
                    page.visible = true;
                }
                Self {
                    pages
                }.into()

            },
            Action::ToggleVisible { page_name } => {
                let mut pages = self.pages.clone();
                pages.entry(page_name).and_modify(move |p| {
                    p.visible = !p.visible;
                } );
                Self {
                    pages
                }.into()
            },
        }
    }
}
