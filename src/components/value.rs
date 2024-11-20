// use _ValueProps::on_edit_template;
use serde_json::Value;
use web_sys::{HtmlInputElement, MouseEvent};
use yew::events::{Event, FocusEvent, KeyboardEvent};
use yew::prelude::*;
use yew_autoprops::autoprops;

use crate::hooks::use_bool_toggle::use_bool_toggle;
use crate::state::{PageName, ValueName};

#[autoprops]
#[function_component(ValueView)]
pub fn value(
    page_name: &PageName,
    value_name: &ValueName,
    value: &serde_json::Value,
    on_edit_value: Callback<(PageName, ValueName, Value)>,
    on_remove_value: Callback<(PageName, ValueName)>,
) -> Html {
    let mut class = Classes::from("todo");

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

    let on_edit = {
        let on_edit = on_edit_value.clone();
        let edit_toggle = edit_toggle.clone();
        move |value| {
            edit_toggle.clone().toggle();
            on_edit.emit(value)
        }
    };

    let on_remove_value = {
        let on_remove_value = on_remove_value.clone();
        let name = page_name.clone();
        let value_name = value_name.clone();
        move |_| on_remove_value.emit((name.clone(), value_name.clone()))
    };

    html! {
        <li {class}>
            <div class="valuename">
                { value_name.clone().0 }
                <button onclick={on_remove_value}>{"delete value"}</button>
            </div>
            <div class="view">
                <label ondblclick={move |_| edit_toggle.clone().toggle()}>
                    { serde_json::to_string_pretty(&value).unwrap() }
                </label>
            </div>
            <ValueEditView page_name={page_name.clone()} value_name={value_name.clone()} value={value.clone()} on_edit={on_edit} editing={is_editing} />
        </li>
    }
}

#[autoprops]
#[function_component(ValueEditView)]
pub fn value_edit(
    page_name: &PageName,
    value_name: &ValueName,
    value: &Value,
    on_edit: Callback<(PageName, ValueName, Value)>,
    editing: bool,
) -> Html {
    if editing {
        let target_input_value = |e: &Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            input.value()
        };

        let onblur = {
            let page_name = page_name.clone();
            let value_name = value_name.clone();
            let edit = on_edit.clone();

            move |e: FocusEvent| {
                let value = target_input_value(&e);
                // todo handle error case
                if let Ok(value) = serde_json::from_str(&value) {
                    edit.emit((page_name.clone(), value_name.clone(), value))
                }
            }
        };

        let onkeypress = {
            let page_name = page_name.clone();
            let value_name = value_name.clone();
            let edit = on_edit.clone();

            move |e: KeyboardEvent| {
                if e.key() == "Enter" {
                    let value = target_input_value(&e);
                    // todo handle error case
                    if let Ok(value) = serde_json::from_str(&value) {
                        edit.emit((page_name.clone(), value_name.clone(), value))
                    }
                }
            }
        };

        let onmouseover = |e: MouseEvent| {
            e.target_unchecked_into::<HtmlInputElement>()
                .focus()
                .unwrap_or_default();
        };

        html! {
            <input
                class="edit"
                type="text"
                value={ serde_json::to_string_pretty(&value).unwrap() }
                {onmouseover}
                {onblur}
                {onkeypress}
            />
        }
    } else {
        html! { <input type="hidden" /> }
    }
}

#[autoprops]
#[function_component(ValueInput)]
pub fn value_creation(
    page_name: &PageName,
    on_add: Callback<(PageName, ValueName, Value)>,
) -> Html {
    let page_name = page_name.clone();
    let onkeypress = {
        move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: HtmlInputElement = e.target_unchecked_into();
                let value_name = input.value();

                input.set_value("");

                on_add.emit((page_name.clone(), value_name.into(), Value::Null));
            }
        }
    };

    html! {
        <input
            type="text"
            class="new-value-name"
            placeholder=""
            {onkeypress}
        />
    }
}
