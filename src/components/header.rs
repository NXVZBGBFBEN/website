use yew::prelude::*;
use stylist::style;

#[function_component(Header)]
pub fn header() -> Html {
    let header_style = style!(r#"
        * {
        }
    "#).expect("Faild to mount style");
    html! {
        <>
        <div class={header_style}>
            <h1>{"NXVZBGBFBEN"}</h1>
        </div>
        </>
    }
}
