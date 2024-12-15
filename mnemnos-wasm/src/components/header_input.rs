use web_sys::HtmlInputElement;
use yew::events::KeyboardEvent;
use yew::prelude::*;
use yew_autoprops::autoprops;

use mnemnos_types::{Page, PageName};

#[autoprops]
#[function_component(HeaderInput)]
pub fn header_input(on_add: Callback<(PageName, Page)>) -> Html {
    let onkeypress = {
        move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: HtmlInputElement = e.target_unchecked_into();
                let page_name = input.value();

                input.set_value("");

                on_add.emit((
                    page_name.into(),
                    Page {
                        markdown: "some markdown here".to_string(),
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
