use website::app::App;
use yew::prelude::*;
use yew_router::prelude::*;

fn main() {
    yew::Renderer::<Main>::new().render();
}

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
