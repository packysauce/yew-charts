mod utils;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use plotters::prelude::*;

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
        let mut inside = yew::virtual_dom::VTag::new("svg");
        inside.add_attribute(&"width", &"800");
        inside.add_attribute(&"height", &"600");
        inside.add_attribute(&"viewBox", &"0 0 800 600");
        inside.add_attribute(&"fill", &"#ffffff");

        {
        let wrapper = utils::VTagWrapper(&mut inside);
        let root = wrapper.into_drawing_area();
        root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(35)
            .y_label_area_size(40)
            .right_y_label_area_size(40)
            .margin(5)
            .caption("Dual Y-Axis Example", ("sans-serif", 50.0).into_font())
            .build_ranged(0f32..10f32, LogRange(0.1f32..1e10f32)).unwrap()
            .set_secondary_coord(0f32..10f32, -1.0f32..1.0f32);

        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .y_desc("Log Scale")
            .y_label_formatter(&|x| format!("{:e}", x))
            .draw().unwrap();

        chart
            .configure_secondary_axes()
            .y_desc("Linear Scale")
            .draw().unwrap();

        chart
            .draw_series(LineSeries::new(
                (0..=100).map(|x| (x as f32 / 10.0, (1.02f32).powf(x as f32 * x as f32 / 10.0))),
                &BLUE,
            )).unwrap()
            .label("y = 1.02^x^2")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

        chart
            .draw_secondary_series(LineSeries::new(
                (0..=100).map(|x| (x as f32 / 10.0, (x as f32 / 5.0).sin())),
                &RED,
            )).unwrap()
            .label("y = sin(2x)")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        chart
            .configure_series_labels()
            .background_style(&RGBColor(128, 128, 128))
            .draw().unwrap();
        }
        let cb = self.link.callback(Msg::HoverPoint);
        /*html! {
            <>
            //<ChartComponent width=800, height=600 data=vec![(12, 54), (100, 40), (120, 50), (180, 70)] on_hover=cb/>
            </>
        }*/
        inside.into()
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
