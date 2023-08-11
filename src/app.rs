use crate::components::header::Header;
use crate::pages::home::Home;
use crate::pages::status::not_found::NotFound;
use stylist::css;
use stylist::yew::Global;
use yew::prelude::function_component;
use yew::{html, Html};
use yew_router::prelude::{Routable, Switch};

#[derive(Clone, Routable, PartialEq)]
enum Route {
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
    let global_style = css!(
        r#"
        * {
            background-color: #282C34;
            color: #ABB2BF;
        }
    "#
    );
    html! {
        <>
            <Global css={global_style} />
            <Header />
            <Switch<Route> render={switch} />
        </>
    }
}
