use yew::prelude::*;

#[function_component(TargetBrankLink)]
pub fn target_blank_link(props: &Props) -> Html {
    html! {
        <>
            <a href={props.href.clone()} target="_blank" rel="noopener noreferrer">{props.text.clone()}</a>
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub text: AttrValue,
    pub href: AttrValue,
}
