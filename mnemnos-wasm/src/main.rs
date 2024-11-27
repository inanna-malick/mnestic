use std::collections::HashMap;
use std::rc::Rc;

use gloo::storage::{LocalStorage, Storage};
use mnemnos_types::{Action, Page, PageName, AppState};
use tera::Tera;
use yew::prelude::*;

mod components;
mod hooks;

use components::header_input::HeaderInput;
use components::page::PageView;

const KEY: &str = "yew.functiontodomvc.self";

#[function_component(App)]
fn app() -> Html {
    let state = use_reducer(|| AppState {
        pages: LocalStorage::get(KEY).unwrap_or_else(|_| HashMap::new()),
    });

    // Effect
    use_effect_with(state.clone(), |state| {
        LocalStorage::set(KEY, &state.clone().pages).expect("failed to set");
    });

    fn make_callback<E, F>(state: &UseReducerHandle<AppState>, f: F) -> Callback<E>
    where
        F: Fn(E) -> Action + 'static,
    {
        let state = state.clone();
        Callback::from(move |e: E| state.dispatch(f(e)))
    }

    let on_add = make_callback(&state, |(page_name, page): (PageName, Page)| {
        Action::AddPage { page_name, page }
    });

    let on_page_remove = make_callback(&state, |page_name: PageName| Action::RemovePage {
        page_name,
    });

    let on_edit_template = make_callback(&state, |(page_name, template): (PageName, String)| {
        Action::EditTemplate {
            page_name,
            template,
        }
    });

    let on_add_value = make_callback(&state, |(page_name, value_name, value)| Action::AddValue {
        page_name,
        value_name,
        value,
    });

    let on_edit_value = make_callback(&state, |(page_name, value_name, value)| Action::EditValue {
        page_name,
        value_name,
        value,
    });

    let on_remove_value = make_callback(&state, |(page_name, value_name)| Action::RemoveValue {
        page_name,
        value_name,
    });

    let tera: Rc<Result<TeraWrapper, String>> =
        use_memo(state.pages.clone(), |pages| TeraWrapper::new(pages.clone()));


    

    html! {
        <div class="container">
            <h1>{ "Pages" }</h1>
            <div class="row">
                <div class="col s12">
                    <HeaderInput {on_add} />
                </div>
                <label for="toggle-all" />
                <button class={classes!("btn", "waves-effect", "waves-light")} onclick={|_| todo!()}>{"delete page"}</button>

            </div>

                { for state.pages.iter().map(|(page_name, page)|
                    html! {
                        <PageView
                            page_name={page_name.clone()}
                            page={page.clone()}
                            on_page_remove={&on_page_remove}
                            on_edit_template={&on_edit_template}
                            on_add_value={&on_add_value}
                            on_edit_value={&on_edit_value}
                            on_remove_value={&on_remove_value}
                            tera={&tera}
                        />
                }) }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

#[derive(Clone, Debug)]
struct TeraWrapper {
    source: HashMap<PageName, Page>,
    compiled: Tera,
}

impl TeraWrapper {
    pub fn new(source: HashMap<PageName, Page>) -> Result<Self, String> {
        let mut tera = Tera::default();
        match tera.add_raw_templates(
            source
                .iter()
                .map(|(k, v)| (k.0.clone(), v.template.clone())),
        ) {
            Ok(()) => Ok(Self {
                source,
                compiled: tera,
            }),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn render(&self, page_name: &PageName, context: &tera::Context) -> Result<String, String> {
        self.compiled
            .render(&page_name.0, context)
            .map_err(|e| e.to_string())
    }
}

impl PartialEq for TeraWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source
    }
}

impl Eq for TeraWrapper {}
