use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages;

#[function_component(Footer)]
pub fn footer() -> Html {
    let footnote = match use_location().unwrap().path() {
        "/" => pages::home::footnote(),
        _ => None
    };
    html! {
        <>
            <footer class="mt-5 text-center">
                if let Some(element) = footnote {
                    { element }
                }
                <p>{"© 2023 NXVZBGBFBEN"}</p>
            </footer>
        </>
    }
}
