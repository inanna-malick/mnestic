use web_sys::wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, MouseEvent};
use yew::events::{Event, KeyboardEvent};
use yew::prelude::*;
use yew_autoprops::autoprops;

use crate::hooks::use_bool_toggle::use_bool_toggle;
use mnemnos_types::{AppState, Page, PageName};

#[autoprops]
#[function_component(PageView)]
pub fn page(
    page_name: &PageName,
    page: &Page,
    state: &UseReducerHandle<AppState>,
    on_page_remove: Callback<PageName>,
    on_edit_template: Callback<(PageName, String)>,
) -> Html {
    let mut class = classes!("row", "teal", "lighten-5");

    // We use the `use_bool_toggle` hook and set the default value to `false`
    // as the default we are not editing the entry. When we want to edit the
    // entry we can call the toggle method on the `UseBoolToggleHandle`
    // which will trigger a re-render with the toggle value being `true` for that
    // render and after that render the value of toggle will be flipped back to
    // its default (`false`).
    // We are relying on the behavior of `onblur` and `onkeypress` to cause
    // another render so that this component will render again with the
    // default value of toggle.
    let edit_toggle = use_bool_toggle(false);
    let is_editing = *edit_toggle;

    if is_editing {
        class.push("editing");
    }

    let on_page_remove = {
        let on_page_remove = on_page_remove.clone();
        let name = page_name.clone();
        move |_| on_page_remove.emit(name.clone())
    };

    let on_edit_t = {
        let on_edit = on_edit_template.clone();
        let edit_toggle = edit_toggle.clone();
        move |value| {
            edit_toggle.clone().toggle();
            on_edit.emit(value)
        }
    };

    html! {
        <div {class}>
              <div class={classes!("col", "s12")} >
                { page_name.clone().0 }
                <button class={classes!("btn", "waves-effect", "waves-light")} onclick={on_page_remove}>{"delete page"}</button>
            </div>
            <br/>
            <div class="divider"></div>
            <br/>
            <div class={classes!("col", "s6")} >
                { if is_editing {
                    html!{
                        <TemplateEditView page_name={page_name.clone()} page={page.clone()} on_edit={on_edit_t} />
                    }
                } else {
                    html!{
                        <div style="white-space: pre-wrap"  ondblclick={move |_| edit_toggle.clone().toggle()}>
                            { &page.markdown }
                        </div>
                    }
                }
            }
            </div>

            <div class={classes!("col", "s6")} style={"height:100%;"}  >
                <RenderedPageView page_name={page_name.clone()} state={state.clone()}/>
            </div>
        </div>
    }
}

#[autoprops]
#[function_component(TemplateEditView)]
pub fn page_edit(page_name: &PageName, page: &Page, on_edit: Callback<(PageName, String)>) -> Html {
    let name = page_name.clone();

    let target_input_value = |e: &Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        input.value()
    };

    let onkeypress = {
        let edit = on_edit.clone();
        let name = name.clone();

        move |e: KeyboardEvent| {
            if e.key() == "Enter" && e.shift_key() {
                let value = target_input_value(&e);
                edit.emit((name.clone(), value))
            }
        }
    };

    let on_save = |id: String| {
        let edit = on_edit.clone();
        let name = name.clone();

        move |_| {
            let document = web_sys::window().unwrap().document().unwrap();

            let element = document.get_element_by_id(&id).unwrap();
            let value: HtmlInputElement = element.unchecked_into();
            edit.emit((name.clone(), value.value()))
        }
    };

    let onmouseover = |e: MouseEvent| {
        let x = e.target_unchecked_into::<HtmlInputElement>();
        x.focus().unwrap_or_default();
    };

    let id = format!("{}-edit-template", page_name.0);

    html! {
        <div>
            <i class="material-icons prefix">{"mode_edit"}</i>
            <textarea
                id={id.clone()}
                class="materialize-textarea"
                rows={"10"}
                {onmouseover}
                // {onkeyup}
                // {onblur}
                {onkeypress}
                value={page.markdown.clone()}
            >

            </textarea>
            <label for={id.clone()}>{"editing markdown"}</label>


            <button class={classes!("btn", "waves-effect", "waves-light")} onclick={on_save(id.clone())}>{"save changes"}</button>

        </div>
    }
}

#[autoprops]
#[function_component(RenderedPageView)]
pub fn page(page_name: &PageName, state: &UseReducerHandle<AppState>) -> Html {
    // TODO: render markdown here

    // Render the template with the given context
    match state.render(page_name) {
        Ok(rendered) => {
            html! {
                <div class="resizer">
                <iframe class="resized" srcdoc={rendered}  ></iframe>
                </div>
            }
        }
        Err(e) => {
            html! {
                <div>{format!("render error: {e:?}")}</div>
            }
        }
    }
}
