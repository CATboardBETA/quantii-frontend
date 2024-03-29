#![allow(non_snake_case)]

use gloo_console::log;
use rand::prelude::ThreadRng;
use rand::RngCore;
use std::sync::Mutex;
use yew::prelude::*;

mod components;
use components::*;

pub type UsedIds = Vec<u32>;

pub static USED_IDS: Mutex<UsedIds> = Mutex::new(vec![]);

#[function_component(App)]
pub fn app() -> Html {
    let thread_rng: ThreadRng = rand::thread_rng();

    let launch_window = || {};
    let home_callback: Callback<MouseEvent, ()> = Callback::from(move |_| {
        log!("Home");
        launch_window()
    });

    let items = vec![
        SidebarItemProps {
            name: AttrValue::from("home"),
            icon: AttrValue::from("public/home.svg"),
            callback: home_callback,
        },
        SidebarItemProps {
            name: AttrValue::from("settings"),
            icon: AttrValue::from("public/settings.svg"),
            callback: Callback::noop(),
        },
    ];

    let onclose = Callback::noop();
    let onminimize = Callback::noop();
    let onmaximize = Callback::noop();

    html! {
        <div class="row no-padding no-margin">
            <Sidebar items={items} />
            <Desktop class="desktop no-padding no-margin">
                <Window
                    width=5000
                    height=5000
                    title="Test Window"
                    onclose={onclose}
                    onminimize={onminimize}
                    onmaximize={onmaximize}
                    id={random_id(thread_rng)}
                >
                    <div>
                        {"Hello, world!"}
                    </div>
                </Window>
            </Desktop>
        </div>
    }
}

fn random_id(mut rng: ThreadRng) -> String {
    let mut id: u32 = rng.next_u32();
    let mut not_foundid: bool = true;
    while not_foundid {
        if USED_IDS
            .lock()
            .expect("Wha-whoa, spaghet-io!")
            .contains(&id)
        {
            id = rng.next_u32();
        } else {
            USED_IDS.lock().expect("Wha-whoa, spaghet-io!").push(id);
            not_foundid = false;
        }
    }
    id.to_string()
}
