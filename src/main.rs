use website::app::App;
use yew::prelude::function_component;
use yew::{html, Html};
use yew_router::BrowserRouter;

#[function_component(Main)]
fn app() -> Html {
    html! {
        <>
            <BrowserRouter>
                <App />
            </BrowserRouter>
        </>
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}
