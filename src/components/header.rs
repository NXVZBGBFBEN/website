use crate::app::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    let route = use_route::<crate::app::Route>().unwrap_or_default();
    html! {
        <>
            <div class="navbar bg-base-100">
                <div class="navbar-start">
                    <div class="dropdown">
                        <div tabindex="0" role="button" class="btn btn-ghost lg:hidden">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h8m-8 6h16" /></svg>
                        </div>
                        <ul tabindex="0" class="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52">
                            <li><Link<Route> to={Route::Home}>{"Home"}</Link<Route>></li>
                            <li><Link<Route> to={Route::Works}>{"Works"}</Link<Route>></li>
                        </ul>
                    </div>
                    <Link<Route> to={Route::Home} classes={classes!("btn", "btn-ghost", "text-2xl")}>{"NXVZBGBFBEN"}</Link<Route>>
                </div>
                <div class="navbar-end hidden lg:flex">
                    <ul class="menu menu-horizontal px-1">
                        <li><Link<Route> classes={classes!("text-lg", if route == Route::Home {"active"} else {""})} to={Route::Home}>{"Home"}</Link<Route>></li>
                        <li><Link<Route> classes={classes!("text-lg", if route == Route::Works {"active"} else {""})} to={Route::Works}>{"Works"}</Link<Route>></li>
                    </ul>
                </div>
            </div>
        </>
    }
}
