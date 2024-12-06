use std::rc::Rc;

use serde_json::Value;
use tera::Tera;
use web_sys::wasm_bindgen::JsValue;
use web_sys::{console, HtmlInputElement, MouseEvent};
use yew::events::{Event, FocusEvent, KeyboardEvent};
use yew::prelude::*;
use yew_autoprops::autoprops;

use crate::components::value::{ValueInput, ValueView};
use crate::hooks::use_bool_toggle::use_bool_toggle;
use crate::TeraWrapper;
use mnemnos_types::{Page, PageName, ValueName};

#[autoprops]
#[function_component(PageView)]
pub fn page(
    page_name: &PageName,
    page: &Page,
    on_page_remove: Callback<PageName>,
    on_edit_template: Callback<(PageName, String)>,
    on_add_value: Callback<(PageName, ValueName, Value)>,
    on_edit_value: Callback<(PageName, ValueName, Value)>,
    on_remove_value: Callback<(PageName, ValueName)>,
    tera: Rc<Result<TeraWrapper, String>>,
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
            <div class={classes!("col", "s4")} >
                <div style="white-space: pre-wrap"  ondblclick={move |_| edit_toggle.clone().toggle()}>
                    { &page.template }
                </div>
                <br/>
                <div class="divider"></div>
                <br/>
                <TemplateEditView page_name={page_name.clone()} page={page.clone()} on_edit={on_edit_t} editing={is_editing} />
            </div>
            <div class={classes!("col", "s2")} >
                    <ValueInput
                        page_name={page_name.clone()}
                        on_add={&on_add_value}
                    />
                    <br/>


                    <div class="section">
                        { for page.values.iter().map(|(value_name, value)|
                            html! {
                                <ValueView
                                    page_name={page_name.clone()}
                                    value_name={value_name.clone()}
                                    value={value.clone()}
                                    on_edit_value={&on_edit_value}
                                    on_remove_value={&on_remove_value}
                                />
                        }) }
                    </div>
            </div>

            <div class={classes!("col", "s6")} style={"height:100%;"}  >
                <RenderedPageView page_name={page_name.clone()} page={page.clone()} tera={tera.clone()}/>
            </div>
        </div>
    }
}

#[autoprops]
#[function_component(TemplateEditView)]
pub fn page_edit(
    page_name: &PageName,
    page: &Page,
    on_edit: Callback<(PageName, String)>,
    editing: bool,
) -> Html {
    if editing {
        let name = page_name.clone();

        let target_input_value = |e: &Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            input.value()
        };

        let onblur = {
            let edit = on_edit.clone();
            let name = name.clone();

            move |e: FocusEvent| {
                let value = target_input_value(&e);
                edit.emit((name.clone(), value))
            }
        };

        let onkeypress = {
            let edit = on_edit.clone();
            let name = name.clone();

            move |e: KeyboardEvent| {
                if e.key() == "Enter" {
                    let value = target_input_value(&e);
                    edit.emit((name.clone(), value))
                }
            }
        };

        let onmouseover = |e: MouseEvent| {
            let x = e.target_unchecked_into::<HtmlInputElement>();
            x.focus().unwrap_or_default();
        };

        // todo fix this later i guess
        // let onkeyup = |e: KeyboardEvent| {
        //     console::log_1(&JsValue::from_str("KEYUP"));
        //     let x = e.target_unchecked_into::<HtmlInputElement>();
        //     if (x.scroll_height() > x.client_height()) {
        //         let style = x.style();

        //         style.set_property("height", &format!("{} + px", x.scroll_height())).unwrap();
        //     }
        // };

        // textarea.noscrollbars {
        // overflow: hidden;
        // width: 300px;
        // height: 100px;
        // }

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
                    value={page.template.clone()}
                >

                </textarea>
                <label for={id}>{"Template Contents"}</label>
            </div>
        }
    } else {
        html! { <input type="hidden" /> }
    }
}

#[autoprops]
#[function_component(RenderedPageView)]
pub fn page(page_name: &PageName, page: &Page, tera: Rc<Result<TeraWrapper, String>>) -> Html {
    let tera = match tera.as_ref() {
        Ok(t) => t,
        Err(e) => {
            return html! {
                <div>{format!("template error: {e:?}")}</div>
            };
        }
    };

    // Prepare the context with some data
    let mut context = tera::Context::new();
    for (value_name, value) in page.values.iter() {
        context.insert(value_name.0.clone(), value);
    }

    // Render the template with the given context
    match tera.render(&page_name, &context) {
        Ok(rendered) => {
            html! {
                <iframe srcdoc={rendered} style={"width:100%;height:100%;"} ></iframe>
            }
        }
        Err(e) => {
            html! {
                <div>{format!("render error: {e:?}")}</div>
            }
        }
    }
}
