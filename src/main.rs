use std::collections::HashMap;

use gloo::storage::{LocalStorage, Storage};
use state::{Action, Page, PageName, State};
use strum::IntoEnumIterator;
use yew::prelude::*;

mod components;
mod hooks;
mod state;

use components::header_input::HeaderInput;
use components::page::Page as PageComponent;

const KEY: &str = "yew.functiontodomvc.self";

#[function_component(App)]
fn app() -> Html {
    let state = use_reducer(|| State {
        pages: LocalStorage::get(KEY).unwrap_or_else(|_| HashMap::new()),
    });

    // Effect
    use_effect_with(state.clone(), |state| {
        LocalStorage::set(KEY, &state.clone().pages).expect("failed to set");
    });

    fn make_callback<E, F>(state: &UseReducerHandle<State>, f: F) -> Callback<E>
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

    // let hidden_class = if state.entries.is_empty() {
    //     "hidden"
    // } else {
    //     ""
    // };

    html! {
        <div class="todomvc-wrapper">
            <section>
                <header class="header">
                    <h1>{ "Pages" }</h1>
                    <HeaderInput {on_add} />
                </header>
                <section>
                    <label for="toggle-all" />
                    <ul class="todo-list">
                        { for state.pages.iter().map(|(page_name, page)|
                            html! {
                                <PageComponent
                                    page_name={page_name.clone()}
                                    page={page.clone()}
                                    on_page_remove={&on_page_remove}
                                    on_edit_template={&on_edit_template}
                                    on_add_value={&on_add_value}
                                    on_edit_value={&on_edit_value}
                                    on_remove_value={&on_remove_value}
                                />
                        }) }
                    </ul>
                </section>
            </section>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
