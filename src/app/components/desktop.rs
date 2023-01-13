use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct DesktopProps {
    pub children: Children,
    #[prop_or("".to_owned())]
    pub class: String,
    #[prop_or("".to_owned())]
    pub id: String,
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
