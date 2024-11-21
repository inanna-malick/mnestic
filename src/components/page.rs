use serde_json::Value;
use web_sys::{HtmlInputElement, MouseEvent};
use yew::events::{Event, FocusEvent, KeyboardEvent};
use yew::prelude::*;
use yew_autoprops::autoprops;

use crate::components::value::{ValueInput, ValueView};
use crate::hooks::use_bool_toggle::use_bool_toggle;
use crate::state::{Page, PageName, ValueName};

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
) -> Html {
    let mut class = classes!("row");

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
              <div class={classes!("col", "s12", "teal", "lighten-5")} >
                { page_name.clone().0 }
                <button class={classes!("btn", "waves-effect", "waves-light")} onclick={on_page_remove}>{"delete page"}</button>
            </div>
            <div class={classes!("col", "s4", "teal", "lighten-5")} >
                <div ondblclick={move |_| edit_toggle.clone().toggle()}>
                    { &page.template }
                </div>
                <TemplateEditView page_name={page_name.clone()} page={page.clone()} on_edit={on_edit_t} editing={is_editing} />
            </div>
            <div class={classes!("col", "s4", "teal", "lighten-5")} >
                    <ValueInput
                        page_name={page_name.clone()}
                        on_add={&on_add_value}
                    />
                    <br/>

                    <div class="divider"></div>
                    
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

            <div class={classes!("col", "s4", "teal", "lighten-5")} >
                <RenderedPageView page_name={page_name.clone()} page={page.clone()}/>
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
            e.target_unchecked_into::<HtmlInputElement>()
                .focus()
                .unwrap_or_default();
        };

        let id = format!("{}-edit-template", page_name.0);

        html! {
            <div>
                <i class="material-icons prefix">{"mode_edit"}</i>
                <textarea
                    id={id.clone()}
                    class="materialize-textarea"
                    {onmouseover}
                    {onblur}
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
pub fn page(page_name: &PageName, page: &Page) -> Html {
    use tera::*;

    let mut tera = Tera::default();
    if let Err(e) = tera.add_raw_template(&page_name.0, &page.template) {
        return html! {
            <div>{format!("template error: {e:?}")}</div>
        };
    }

    // Prepare the context with some data
    let mut context = tera::Context::new();
    for (value_name, value) in page.values.iter() {
        context.insert(value_name.0.clone(), value);
    }

    // Render the template with the given context
    match tera.render(&page_name.0, &context) {
        Ok(rendered) => {
            html! {
                <iframe srcdoc={rendered}></iframe>
            }
        }
        Err(e) => {
            html! {
                <div>{format!("render error: {e:?}")}</div>
            }
        }
    }
}
