use crate::app::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <>
            <nav class="navbar navbar-expand-lg bg-body-tertiary">
                <div class="container-fluid">
                    <Link<Route> to={Route::Home} classes={classes!("navbar-brand")}>{"NXVZBGBFBEN"}</Link<Route>>
                    <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
                        <span class="navbar-toggler-icon"></span>
                    </button>
                    <div class="collapse navbar-collapse" id="navbarSupportedContent">
                        <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                            <li class="nav-item">
                                <Link<Route> to={Route::Home} classes={classes!("nav-link")}>{"Home"}</Link<Route>>
                            </li>
                            <li class="nav-item">
                                <Link<Route> to={Route::NotFound} classes={classes!("nav-link", "disabled")}>{"Works"}</Link<Route>>
                            </li>
                            <li class="nav-item">
                                <Link<Route> to={Route::NotFound} classes={classes!("nav-link", "disabled")}>{"Hobbies"}</Link<Route>>
                            </li>
                            <li class="nav-item dropdown">
                                <button class="nav-link dropdown-toggle" type="button" data-bs-toggle="dropdown" aria-expanded="false">
                                    {"Links"}
                                </button>
                                <ul class="dropdown-menu">
                                    <li><a class="dropdown-item" href="https://github.com/NXVZBGBFBEN">{"GitHub"}</a></li>
                                    <li><a class="dropdown-item" href="https://twitter.com/NXVZBGBFBEN">{"X (Twitter)"}</a></li>
                                </ul>
                            </li>
                        </ul>
                        <form class="d-flex" role="search">
                             <input class="form-control me-2" type="search" placeholder="Search" aria-label="Search"/>
                             <button class="btn btn-outline-success disabled" type="submit">{"Search"}</button>
                        </form>
                    </div>
                </div>
            </nav>
        </>
    }
}
