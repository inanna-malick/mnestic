use std::collections::HashMap;
use std::rc::Rc;

use comrak::{markdown_to_html, Options};
use serde::{Deserialize, Serialize};
use yew::prelude::*;

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
    pub markdown: String,
}

impl AppState {
    pub fn render(&self, page_name: &PageName) -> Result<String, anyhow::Error> {
        let page = self
            .pages
            .get(page_name)
            .ok_or_else(|| anyhow::Error::msg("invalid page name reference"))?;

        // tpdo use broken_link_callback to implement links between pages on site - no need to mutate ast
        let inner = markdown_to_html(&page.markdown, &Options::default());

        Ok(format!(
            r#"
            <!DOCTYPE html>
                <html>
                <head>
                <title>{}</title>
                </head>
                <body>
                {}
                </body>
            </html>
            "#,
            page_name.0, inner
        ))
    }
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
                    .and_modify(move |p| p.markdown = template);
                Self { pages }.into()
            }
        }
    }
}
