use gloo_utils::document;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

use yew::prelude::*;

pub enum WindowMsg {
    Close,
    Minimize,
    Maximize,
    Resize { x: i32, y: i32 },
    Move { x: i32, y: i32 },
}

#[derive(Clone, PartialEq, Properties)]
pub struct WindowProps {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub children: Children,
    pub onclose: Callback<MouseEvent, ()>,
    pub onminimize: Callback<MouseEvent, ()>,
    pub onmaximize: Callback<MouseEvent, ()>,
    #[prop_or("".to_owned())]
    pub class: String,
    #[prop_or("".to_owned())]
    pub id: String,
}

pub struct Window {
    pub full_ref: NodeRef,
    pub titlebar_ref: NodeRef,
}

impl Component for Window {
    type Message = WindowMsg;
    type Properties = WindowProps;

    fn create(_: &Context<Self>) -> Self {
        Self {
            full_ref: NodeRef::default(),
            titlebar_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let width = ctx.props().width;
        let height = ctx.props().height;
        let title = ctx.props().title.clone();
        let children = ctx.props().children.clone();
        let onclose = ctx.props().onclose.clone();
        let onminimize = ctx.props().onminimize.clone();
        let onmaximize = ctx.props().onmaximize.clone();
        let class = ctx.props().class.clone();
        let id = ctx.props().id.clone();
        let header_id = ctx.props().id.clone() + "-header";

        html! {
            <div ref={self.full_ref.clone()} class="window column" style={format!("width={width}px; height={height}px;")} class={class} id={id}>
                <div ref={self.titlebar_ref.clone()} class={format!("window-title row flex-center {}", header_id)}>
                    <div class="window-title-text">{title}</div>
                    <div class="window-title-button window-title-button-close">
                        <button type="button" onclick={onclose} class="window-title-button-close-icon dot no-padding no-margin"></button>
                    </div>
                    <div class="window-title-button window-title-button-minimize">
                        <button type="button" onclick={onminimize} class="window-title-button-minimize-icon dot no-padding no-margin"></button>
                    </div>
                    <div class="window-title-button window-title-button-maximize">
                        <button type="button" onclick={onmaximize} class="window-title-button-maximize-icon dot no-padding no-margin"></button>
                    </div>
                </div>
                <div class="window-content container">
                    {children}
                </div>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let document = document();
            let full_ref = self.full_ref.cast::<Element>().unwrap();
            let titlebar_ref = self.titlebar_ref.cast::<Element>().unwrap();
            let header_id = ctx.props().id.clone() + "-header";

            let full_ref2 = full_ref.clone();
            let resize = move |x: i32, y: i32| {
                let width = full_ref2.client_width() as i32 + x;
                let height = full_ref2.client_height() as i32 + y;
                full_ref2
                    .set_attribute(
                        "style",
                        &format!("width: {}px; height: {}px;", width, height),
                    )
                    .unwrap();
            };

            let full_ref3 = full_ref.clone();
            let move_window = move |x: i32, y: i32| {
                let full_ref_htmlelement = full_ref3.clone().dyn_into::<HtmlElement>().unwrap();
                let left = full_ref_htmlelement.offset_left() as i32 + x;
                let top = full_ref_htmlelement.offset_top() as i32 + y;
                full_ref3
                    .set_attribute("style", &format!("top: {}px; left: {}px;", top, left))
                    .unwrap();
            };

            let mousemove: Box<dyn FnMut(&Event)> = Box::new(move |event: &web_sys::Event| {
                let event = event.dyn_ref::<web_sys::MouseEvent>().unwrap();
                let full_ref_htmlelement = full_ref.clone().dyn_into::<HtmlElement>().unwrap();
                let x = event.client_x();
                let y = event.client_y();
                let target = event.target().unwrap();
                let target = target.dyn_into::<Element>().unwrap();
                let target_class = target.class_name();
                let target_id = target.id();

                let window_far_left = full_ref_htmlelement.offset_left() as i32;
                let window_far_right = window_far_left + full_ref.client_width() as i32;
                let window_far_top = full_ref_htmlelement.offset_top() as i32;
                let window_far_bottom = window_far_top + full_ref.client_height() as i32;

                if target_class.contains("window-title-button") {
                    // They clicked on a button, so we don't need to do anything here
                } else if x >= window_far_right - 10
                    || window_far_right >= x - 10
                    || y >= window_far_bottom - 10
                    || window_far_bottom >= y - 10
                    || x <= window_far_left + 10
                    || window_far_left >= x + 10
                    || y <= window_far_top + 10
                    || window_far_top >= y + 10
                {
                    // They clicked on the edge of the window, so we need to resize it
                    resize(event.movement_x(), event.movement_y());
                } else if target_id.contains(&header_id) {
                    // They clicked on the header, so we need to move the window
                    move_window(event.movement_x(), event.movement_y());
                } else {
                    // They clicked on the inside of the window, so we don't need to do anything
                }
            });

            gloo_events::EventListener::new(&document, "mousemove", mousemove).forget();
        } else {
            // It's not the first render, so all this has already been done
        }
    }
}
