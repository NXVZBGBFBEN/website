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
                <ScrollToTop />
                <App />
            </BrowserRouter>
        </>
    }
}

#[function_component(ScrollToTop)]
fn scroll_to_top() -> Html {
    let location = use_location();
    use_effect_with(location, |_| {
        if let Some(window) = web_sys::window() {
            let scroll_to_options = web_sys::ScrollToOptions::new();
            scroll_to_options.set_behavior(web_sys::ScrollBehavior::Instant);
            scroll_to_options.set_top(0.0);
            window.scroll_to_with_scroll_to_options(&scroll_to_options);
        };
    });
    html! {}
}
