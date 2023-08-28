use crate::app::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Works)]
pub fn works() -> Html {
    html! {
        <>
            <h1>{"works"}</h1>
            <Link<Route> to={Route::KeTCindy} classes={classes!("nav-link")}>{"Works"}</Link<Route>>
        </>
    }
}
