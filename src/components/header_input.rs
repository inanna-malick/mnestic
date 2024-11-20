use std::collections::HashMap;

use serde_json::Value;
use web_sys::HtmlInputElement;
use yew::events::KeyboardEvent;
use yew::prelude::*;
use yew_autoprops::autoprops;

use crate::state::{Page, PageName};

#[autoprops]
#[function_component(HeaderInput)]
pub fn header_input(on_add: Callback<(PageName, Page)>) -> Html {
    let onkeypress = {
        move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: HtmlInputElement = e.target_unchecked_into();
                let page_name = input.value();

                input.set_value("");

                let mut initial_values = HashMap::new();
                initial_values.insert("name".into(), Value::String("Inanna".to_string()));

                on_add.emit((
                    page_name.into(),
                    Page {
                        template: "Hello {{ name }}".to_string(),
                        values: initial_values,
                    },
                ));
            }
        }
    };

    html! {
        <input
            type="text"
            class="new-page-name"
            placeholder=""
            {onkeypress}
        />
    }
}
