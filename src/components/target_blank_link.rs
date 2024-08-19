use yew::prelude::*;

#[function_component(TargetBlankLink)]
pub fn target_blank_link(props: &Props) -> Html {
    html! {
        <>
            <a href={props.href.clone()} target="_blank" rel="noopener noreferrer" class="link link-primary">{props.text.clone()}</a>
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub text: AttrValue,
    pub href: AttrValue,
}
