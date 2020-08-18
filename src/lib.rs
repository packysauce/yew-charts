mod utils;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub mod chart;
pub use chart::ChartComponent;

pub struct App {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Properties, Clone, Default)]
pub struct Props {
    pub selected: (i32, i32),
}

pub enum Msg {
    HoverPoint((isize,isize)),
}

impl Component for App {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        let cb = self.link.callback(Msg::HoverPoint);
        html! {
            <>
            <ChartComponent width=800, height=600 data=vec![(12, 54), (100, 40), (120, 50), (180, 70)] on_hover=cb/>
            </>
        }
    }
    
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<App>();
}
