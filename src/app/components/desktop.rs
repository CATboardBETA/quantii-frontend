use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct DesktopProps {
    pub children: Children,
}

#[function_component(Desktop)]
pub fn desktop(props: &DesktopProps) -> Html {
    let children = props.children.clone();
    html! {
        <div class="desktop">
            {children}
        </div>
    }
}
