use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;
use crate::pages::works::home::Works;
use crate::pages::works::ketcindy::KeTCindy;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/works")]
    Works,
    #[at("/works/ketcindy")]
    KeTCindy,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home />},
        Route::Works => html! {<Works />},
        Route::KeTCindy => html! {<KeTCindy />},
        Route::NotFound => html! {<NotFound />},
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <Header />
            <Switch<Route> render={switch} />
            <Footer />
        </>
    }
}
