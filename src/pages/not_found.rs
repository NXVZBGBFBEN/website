use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div class="container mx-auto px-5 flex flex-grow items-center justify-center">
            <p class="text-3xl">{"404 Not Found"}</p>
        </div>
    }
}
