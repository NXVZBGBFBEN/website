use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <div class="container">
                <main>
                    <div class="text-center py-5 mb-3">
                        <h1>{"NXVZBGBFBEN"}</h1>
                    </div>
                    <section class="row px-5">
                        <h2 class="mb-2 text-center">{"Profile"}</h2>
                        <div class="col-4">
                            <img class="d-block mx-auto my-1" src="/assets/images/USERICON0-480x480.png" width="200" height="200" />
                        </div>
                        <div class="col-8 px-5">
                            <p class="lh-1.5">
                                {"情報工学科所属の高専生です．"}<br/>
                                {"最近は言語処理系，数式処理系，教材作成などに興味があります．"}
                            </p>
                            <h4>{"Prefer:"}</h4>
                            <img height="32" width="32" src="https://cdn.simpleicons.org/gentoo/212529/adb5bd"/>
                            <img class="ms-2" height="32" width="32" src="https://cdn.simpleicons.org/firefoxbrowser/212529/adb5bd" />
                            <img class="ms-2" height="32" width="32" src="https://cdn.simpleicons.org/latex/212529/adb5bd" />
                            <img class="ms-2" height="32" width="32" src="https://cdn.simpleicons.org/rust/212529/adb5bd" />
                            <img class="ms-2" height="32" width="32" src="https://cdn.simpleicons.org/thunderbird/212529/adb5bd" />
                            <img class="ms-2" height="32" width="32" src="https://cdn.simpleicons.org/vim/212529/adb5bd" />
                        </div>
                    </section>
                </main>
            </div>
        </>
    }
}

pub fn footnote() -> Option<Html> {
    Some(html! {
        <>
            <p class="mb-1 small">
                {"The "}
                <a href="https://www.gentoo.org/">{"Gentoo"}</a>
                {" logo image is licensed under the "}
                <a href="https://creativecommons.org/licenses/by-sa/2.5/">{"CC BY-SA 2.5"}</a>
                {" license. cf. "}
                <a href="https://www.gentoo.org/inside-gentoo/foundation/name-logo-guidelines.html">{"Guidelines"}</a>
            </p>
            <p class="mb-1 small">
                {"The "}
                <a href="https://www.rust-lang.org/">{"Rust"}</a>
                {" logo image is licensed under the "}
                <a href="https://creativecommons.org/licenses/by-sa/2.5/">{"CC BY-SA 4.0"}</a>
                {" license. cf. "}
                <a href="https://foundation.rust-lang.org/policies/logo-policy-and-media-guide/">{"Guidelines"}</a>
            </p>
        </>
    })
}
