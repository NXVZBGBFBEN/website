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
                            <a class="btn btn-link p-0" data-bs-toggle="tooltip" data-bs-trigger="focus" title="aaa">
                                <img height="32" width="32" src="https://cdn.simpleicons.org/gentoo/212529/adb5bd"/>
                            </a><a href="#" class="d-inline-block" data-bs-toggle="tooltip" title="通常のツールチップ">
  <svg xmlns="http://www.w3.org/2000/svg" width="50" height="50" viewBox="0 0 100 100">
    <rect width="100%" height="100%" fill="#563d7c"/>
    <circle cx="50" cy="50" r="30" fill="#007bff"/>
  </svg>
</a>
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
