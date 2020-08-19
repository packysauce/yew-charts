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
        let (width, height) = (1024, 768);
        let mut inside = yew::virtual_dom::VTag::new("svg");
        inside.add_attribute(&"width", &width);
        inside.add_attribute(&"height", &height);
        inside.add_attribute(&"viewBox", &format!("0 0 {} {}", width, height));
        inside.add_attribute(&"fill", &"#ffffff");

        {
        let wrapper = utils::VTagWrapper::new(&mut inside, width, height);
        let root = wrapper.into_drawing_area();
        root.fill(&WHITE).unwrap();

        let root_area = root.titled("Image Title", ("sans-serif", 60).into_font()).unwrap();
    
        let (upper, lower) = root_area.split_vertically(512);
    
        let mut cc = ChartBuilder::on(&upper)
            .margin(5)
            .set_all_label_area_size(50)
            .caption("Sine and Cosine", ("sans-serif", 40).into_font())
            .build_ranged(-3.4f32..3.4f32, -1.2f32..1.2f32).unwrap();
    
        cc.configure_mesh()
            .x_labels(20)
            .y_labels(10)
            .disable_mesh()
            .x_label_formatter(&|v| format!("{:.1}", v))
            .y_label_formatter(&|v| format!("{:.1}", v))
            .draw().unwrap();
    
        cc.draw_series(LineSeries::new(
            (0..12).map(|x| ((x - 6) as f32 / 2.0, ((x - 6) as f32 / 2.0).sin())),
            &RED,
        )).unwrap()
        .label("Sine")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    
        cc.draw_series(LineSeries::new(
            (0..6800).map(|x| {
                (
                    (x - 3400) as f32 / 1000.0,
                    ((x - 3400) as f32 / 1000.0).cos(),
                )
            }),
            &BLUE,
        )).unwrap()
        .label("Cosine")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
    
        cc.configure_series_labels().border_style(&BLACK).draw().unwrap();
    
        /*
        // It's possible to use a existing pointing element
         cc.draw_series(PointSeries::<_, _, Circle<_>>::new(
            (0..6).map(|x| ((x - 3) as f32 / 1.0, ((x - 3) as f32 / 1.0).sin())),
            5,
            Into::<ShapeStyle>::into(&RGBColor(255,0,0)).filled(),
        ))?;*/
    
        // Otherwise you can use a function to construct your pointing element yourself
        cc.draw_series(PointSeries::of_element(
            (0..6).map(|x| ((x - 3) as f32 / 1.0, ((x - 3) as f32 / 1.0).sin())),
            5,
            ShapeStyle::from(&RED).filled(),
            &|coord, size, style| {
                EmptyElement::at(coord)
                    + Circle::new((0, 0), size, style)
                    + Text::new(
                        format!("{:?}", coord),
                        (0, 15),
                        ("sans-serif", 15).into_font(),
                    )
            },
        )).unwrap();
    
        let drawing_areas = lower.split_evenly((1, 2));
    
        for (drawing_area, idx) in drawing_areas.iter().zip(1..) {
            let mut cc = ChartBuilder::on(&drawing_area)
                .x_label_area_size(30)
                .y_label_area_size(30)
                .margin_right(20)
                .caption(
                    format!("y = x^{}", 1 + 2 * idx),
                    ("sans-serif", 40).into_font(),
                )
                .build_ranged(-1f32..1f32, -1f32..1f32).unwrap();
            cc.configure_mesh().x_labels(5).y_labels(3).draw().unwrap();
    
            cc.draw_series(LineSeries::new(
                (-100..100).map(|x| {
                    (
                        x as f32 / 100.0,
                        (x as f32 / 100.0).powf(idx as f32 * 2.0 + 1.0),
                    )
                }),
                &BLUE,
            )).unwrap();
        }
        /*
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
            */
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
