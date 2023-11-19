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
                    <section class="row">
                        <h2 class="mb-2">{"Profile"}</h2>
                        <hr/>
                        <div class="col-md-4">
                            <img class="img-thumbnail d-block mx-auto" src="/assets/images/USERICON0-480x480.png" width="50%"/>
                        </div>
                        <div class="col-md-8">
                            <p class="lh-1.5">
                                {"情報工学科所属の高専生です．"}<br/>
                                {"最近はサイバーセキュリティ，言語・数式処理系，教材作成などに興味があります．"}
                            </p>
                            <p class="lh-1.5">
                                {"所有資格・受賞歴等は"}
                                <a href="https://www.linkedin.com/in/akimoto-sora/" target="_blank" rel="noopener noreferrer">{"LinkedInアカウント"}</a>
                                {"に掲載しています．"}
                            </p>
                            <table class="table table-borderless">
                                <tbody>
                                    <tr>
                                        <th class="align-middle">{"Favorite software"}</th>
                                        <td>
                                            <div class="d-md-flex gap-2">
                                                <img height="32" width="32" title="Gentoo Linux" src="https://cdn.simpleicons.org/gentoo/212529/adb5bd"/>
                                                <img height="32" width="32" title="Firefox browser" src="https://cdn.simpleicons.org/firefoxbrowser/212529/adb5bd" />
                                                <img height="32" width="32" title="LaTeX" src="https://cdn.simpleicons.org/latex/212529/adb5bd" />
                                                <img height="32" width="32" title="Rust" src="https://cdn.simpleicons.org/rust/212529/adb5bd" />
                                                <img height="32" width="32" title="Thunderbird" src="https://cdn.simpleicons.org/thunderbird/212529/adb5bd" />
                                                <img height="32" width="32" title="Vim" src="https://cdn.simpleicons.org/vim/212529/adb5bd" />
                                            </div>
                                        </td>
                                    </tr>
                                </tbody>
                            </table>
                        </div>
                    </section>
                </main>
            </div>
        </>
    }
}

pub fn footnote() -> Html {
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
