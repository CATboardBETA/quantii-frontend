use std::format;
use yew::function_component;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct SidebarProps {
    pub items: Vec<SidebarItemProps>,
}

#[function_component]
pub fn Sidebar(props: &SidebarProps) -> Html {
    let items = props
        .items
        .iter()
        .map(|item| {
            html_nested! {
                <SidebarItem
                    name={item.name.clone()}
                    icon={item.icon.clone()}
                    callback={item.callback.clone()}
                />
            }
        })
        .collect::<Html>();
    html! {
        <div class="sidebar">
            {items}
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct SidebarItemProps {
    pub name: AttrValue,
    pub icon: AttrValue,
    pub callback: Callback<MouseEvent, ()>,
}

#[function_component]
pub fn SidebarItem(props: &SidebarItemProps) -> Html {
    html! {
        <div class={format!("sidebar-item sidebar-item-{}", props.name)}>
            <button type="button" class="sidebar-item-button" onclick={props.callback.clone()}>
                <img class="sidebar-item-icon" src={props.icon.clone()} />
                <span class="sidebar-item-name">{props.name.clone()}</span>
            </button>
        </div>
    }
}
