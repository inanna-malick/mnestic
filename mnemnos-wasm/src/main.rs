use std::collections::HashMap;
use std::ops::Deref;

use gloo::storage::{LocalStorage, Storage};
use gloo_net::http::Request;
use mnemnos_types::{Action, AppState, Page, PageName};
use web_sys::console;
use web_sys::wasm_bindgen::JsValue;
use yew::platform::spawn_local;
use yew::prelude::*;

mod components;
mod hooks;

use components::header_input::HeaderInput;
use components::page::PageView;
use yew::suspense::use_future;

const KEY: &str = "yew.functiontodomvc.self";

#[function_component(App)]
fn app() -> Html {
    let fallback = html! {<div>{"Loading state..."}</div>};

    html! {
        <Suspense {fallback}>
        <LoadAndRun/>
        </Suspense>
    }
}

#[function_component(LoadAndRun)]
fn load_and_run() -> HtmlResult {
    let res = use_future(|| async {
        let res = Request::get("/api/state")
            .send()
            .await?
            .json::<AppState>()
            .await?;

        Ok::<AppState, gloo_net::Error>(res)
    })?;
    // .expect("failed unwrapping result of api call to get state");

    let state = match *res {
        Ok(ref res) => res.clone(),
        Err(ref failure) => {
            console::log_1(&JsValue::from_str(&failure.to_string())); // todo: something less janky
            AppState {
                pages: HashMap::new(),
            }
        }
    };

    let state = use_reducer(|| state);

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

    let state2 = state.clone();
    let state3 = state.clone();

    Ok(html! {
        <div class="container">
            <h1>{ "Pages" }</h1>
            <div class="row">
                <div class="col s12">
                    <HeaderInput {on_add} />
                </div>
                <label for="toggle-all" />
                <button class={classes!("btn", "waves-effect", "waves-light")} onclick={move |_|
                    {
                        let state = state2.clone();
                        spawn_local(async move {
                    // let res = Request::post("/api/state").body(serde_json::to_value(state.borrow_mut()).unwrap() ).send().await;
                    let res = Request::post("/api/state").json(
                    state.deref() ).expect("failure sending json to api/state endpoint").send().await;
                    console::log_1(&JsValue::from_str(&format!("save res: {:?}", res))); // todo: something less janky
                })
            }
                }>{"save state to r2"}</button>

            </div>

                { for state.pages.iter().map(|(page_name, page)|
                    html! {
                        <PageView
                            page_name={page_name.clone()}
                            page={page.clone()}
                            on_page_remove={&on_page_remove}
                            on_edit_template={&on_edit_template}
                            state={state3.clone()}
                        />
                }) }
        </div>
    })
}

fn main() {
    yew::Renderer::<App>::new().render();
}
