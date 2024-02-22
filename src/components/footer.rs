use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    let footnote = match use_location().unwrap().path() {
        "/" => Some(footnote()),
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

fn footnote() -> Html {
    html! {
        <>
            <small class="mb-1">
                {"The "}
                <a href="https://www.gentoo.org/">{"Gentoo"}</a>
                {" logo image is licensed under the "}
                <a href="https://creativecommons.org/licenses/by-sa/2.5/">{"CC BY-SA 2.5"}</a>
                {" license. cf. "}
                <a href="https://www.gentoo.org/inside-gentoo/foundation/name-logo-guidelines.html">{"Guidelines"}</a>
            </small>
            <br/>
            <small class="mb-1">
                {"The "}
                <a href="https://www.rust-lang.org/">{"Rust"}</a>
                {" logo image is licensed under the "}
                <a href="https://creativecommons.org/licenses/by-sa/2.5/">{"CC BY-SA 4.0"}</a>
                {" license. cf. "}
                <a href="https://foundation.rust-lang.org/policies/logo-policy-and-media-guide/">{"Guidelines"}</a>
            </small>
        </>
    }
}
