use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <div class="container-xxl">
                <main>
                    <div class="text-center py-5">
                        <img class="d-block mx-auto mb-2" src="/assets/images/USERICON0-480x480.png" width="100" height="100"/>
                        <h2>{"NXVZBGBFBEN"}</h2>
                        <p class="lead">{"Hello, World!"}</p>
                    </div>
                </main>
            </div>
        </>
    }
}
