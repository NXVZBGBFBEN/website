use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <>
            <footer class="footer footer-center p-10 bg-base-20 text-base-content rounded">
                <nav>
                    <div class="grid grid-flow-col gap-4">
                        <a href="https://github.com/NXVZBGBFBEN"><img height="24" width="24" src="https://cdn.simpleicons.org/github/212529/adb5bd" /></a>
                        <a href="https://x.com/NXVZBGBFBEN"><img height="24" width="24" src="https://cdn.simpleicons.org/x/212529/adb5bd" /></a>
                    </div>
                </nav>
                <aside><p>{"© 2023 NXVZBGBFBEN"}</p></aside>
            </footer>
        </>
    }
}
