use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct WindowProps {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub children: Children,
    pub onclose: Callback<MouseEvent, ()>,
    pub onminimize: Callback<MouseEvent, ()>,
    pub onmaximize: Callback<MouseEvent, ()>,
}

pub struct Window;

impl Component for Window {
    type Message = ();
    type Properties = WindowProps;

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let width = ctx.props().width;
        let height = ctx.props().height;
        let title = ctx.props().title.clone();
        let children = ctx.props().children.clone();
        let onclose = ctx.props().onclose.clone();
        let onminimize = ctx.props().onminimize.clone();
        let onmaximize = ctx.props().onmaximize.clone();

        html!{
            <div class="window column" style={format!("width={}; height={};", width, height)}>
                <div class="window-title row">
                    <div class="window-title-text">{title}</div>
                    <div class="window-title-buttons row">
                        <div class="window-title-button window-title-button-close">
                            <button type="button" class="window-title-button-close-button" onclick={onclose}>
                                <svg class="window-title-button-close-icon" xmlns="http://www.w3.org/2000/svg">
                                    <circle stroke="red" fill="red" cx="12" cy="12" r="10"></circle>
                                </svg>
                            </button>
                        </div>
                        <div class="window-title-button window-title-button-minimize">
                            <button type="button" class="window-title-button-minimize-button" onclick={onminimize}>
                                <svg class="window-title-button-minimize-icon" xmlns="http://www.w3.org/2000/svg">
                                    <circle stroke="yellow" fill="yellow" cx="12" cy="12" r="10"></circle>
                                </svg>
                            </button>
                        </div>
                        <div class="window-title-button window-title-button-maximize">
                            <button type="button" class="window-title-button-maximize-button" onclick={onmaximize}>
                                <svg class="window-title-button-maximize-icon" xmlns="http://www.w3.org/2000/svg">
                                    <circle stroke="green" fill="green" cx="12" cy="12" r="10"></circle>
                                </svg>
                            </button>
                        </div>
                    </div>
                </div>
                <div class="window-content container">
                    {children}
                </div>
            </div>
        }
    }
}