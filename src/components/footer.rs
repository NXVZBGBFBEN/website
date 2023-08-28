use crate::pages;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    let footnote = match use_location().unwrap().path() {
        "/" => Some(pages::home::footnote()),
        _ => None,
    };
    html! {
        <>
            <footer class="mt-5 text-center">
                <hr/>
                if let Some(element) = footnote {
                    { element }
                }
                <p>{"© 2023 NXVZBGBFBEN"}</p>
            </footer>
        </>
    }
}
