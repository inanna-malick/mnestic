// use _ValueProps::on_edit_template;
use serde_json::Value;
use web_sys::{HtmlInputElement, MouseEvent};
use yew::events::{Event, FocusEvent, KeyboardEvent};
use yew::prelude::*;

use crate::hooks::use_bool_toggle::use_bool_toggle;
use crate::state::{PageName, ValueName};

#[derive(PartialEq, Properties, Clone)]
pub struct ValueProps {
    pub page_name: PageName,
    pub name: ValueName,
    pub value: serde_json::Value,
    pub on_edit_value: Callback<(PageName, ValueName, Value)>,
    pub on_remove_value: Callback<(PageName, ValueName)>,
}

#[function_component(ValueView)]
pub fn page(props: &ValueProps) -> Html {
    let page_name = &props.page_name;
    let value_name = &props.name;
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
        let on_edit = props.on_edit_value.clone();
        let edit_toggle = edit_toggle.clone();
        move |value| {
            edit_toggle.clone().toggle();
            on_edit.emit(value)
        }
    };

    let on_remove_value = {
        let on_remove_value = props.on_remove_value.clone();
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
                    { serde_json::to_string_pretty(&props.value).unwrap() }
                </label>
            </div>
            <ValueEditView page_name={page_name.clone()} value_name={value_name.clone()} value={props.value.clone()} on_edit={on_edit} editing={is_editing} />
        </li>
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct ValueEditProps {
    pub page_name: PageName,
    pub value_name: ValueName,
    pub value: Value,
    pub on_edit: Callback<(PageName, ValueName, Value)>,
    pub editing: bool,
}

#[function_component(ValueEditView)]
pub fn page_edit(props: &ValueEditProps) -> Html {
    if props.editing {
        let page_name = &props.page_name;
        let value_name = &props.value_name;

        let target_input_value = |e: &Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            input.value()
        };

        let onblur = {
            let page_name = props.page_name.clone();
            let value_name = props.value_name.clone();
            let edit = props.on_edit.clone();

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
            let edit = props.on_edit.clone();

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
                value={ serde_json::to_string_pretty(&props.value).unwrap() }
                {onmouseover}
                {onblur}
                {onkeypress}
            />
        }
    } else {
        html! { <input type="hidden" /> }
    }
}
