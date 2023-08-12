use crate::components::header::Header;
use crate::pages::home::Home;
use crate::pages::status::not_found::NotFound;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home />},
        Route::NotFound => html! {<NotFound />},
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <Header />
            <Switch<Route> render={switch} />
        </>
    }
}
