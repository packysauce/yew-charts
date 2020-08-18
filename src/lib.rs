mod utils;
use svg::node::element::tag::Type;
use svg::parser::{Event, Parser};
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services::ConsoleService;
use yew::virtual_dom::{VList, VNode, VTag, VText};

pub struct ChartComponent {
    pub props: Props,
    pub width: isize,
    pub height: isize,
    group: Option<svg::node::element::Group>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Nothing,
}

#[derive(Properties, Clone, Default)]
pub struct Props {}

impl Component for ChartComponent {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let (width, height) = (800, 600);
        let (top, right, bottom, left) = (90, 40, 50, 60);
        let x = charts::ScaleLinear::new()
            .set_domain(vec![0f32, 200f32])
            .set_range(vec![0, width - left - right]);
        let y = charts::ScaleLinear::new()
            .set_domain(vec![0f32, 100f32])
            .set_range(vec![height - top - bottom, 0]);
        let line_data = vec![(12, 54), (100, 40), (120, 50), (180, 70)];
        let line_view = charts::LineSeriesView::new()
            .set_x_scale(&x)
            .set_y_scale(&y)
            .set_marker_type(charts::MarkerType::Circle)
            .set_label_position(charts::PointLabelPosition::N)
            .load_data(&line_data)
            .unwrap();
        let c = charts::Chart::new()
            .set_width(width)
            .set_height(height)
            .set_margins(top, right, bottom, left)
            .add_title("Line Chart".to_string())
            .add_view(&line_view)
            .add_axis_bottom(&x)
            .add_axis_left(&y)
            .add_left_axis_label("Custom Y axis")
            .add_bottom_axis_label("Custom bottom");
        Self {
            props,
            width,
            height,
            group: Some(c.to_svg().unwrap()),
            link,
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        let el = self.group.as_ref().unwrap().get_inner();
        let s = self.group.as_ref().unwrap().to_string();
        let buf = std::io::BufReader::new(s.as_bytes());
        let svg = svg::read(buf).unwrap();
        let mut stack: Vec<VTag> = Vec::new();
        let mut root = VTag::new("svg");
        root.add_attribute("width", &self.width);
        root.add_attribute("height", &self.height);
        stack.push(root);
        for event in svg {
            match event {
                Event::Tag(tag, kind, attrs) => {
                    ConsoleService::log(&format!("Event::Tag({}, {:?}, {:?})", tag, kind, attrs));
                    ConsoleService::debug(&format!("{:?}", stack.last()));
                    match kind {
                        Type::End => {
                            // done adding children to this tag
                            let completed = stack.pop().unwrap();
                            let parent = stack.last_mut().unwrap();
                            parent.add_child(completed.into());
                            continue;
                        }
                        Type::Start => {
                            // have start, build tag and attrs
                            let mut vnode = VTag::new(tag.to_owned());
                            let attributes = attrs
                                .iter()
                                .map(|(k, v)| (k.to_string(), v.to_string()))
                                .collect();
                            vnode.add_attributes(attributes);
                            stack.push(vnode);
                        }
                        Type::Empty => {
                            // "empty" tags have no children
                            let mut vnode = VTag::new(tag.to_owned());
                            let attributes = attrs
                                .iter()
                                .map(|(k, v)| (k.to_string(), v.to_string()))
                                .collect();
                            vnode.add_attributes(attributes);
                            stack
                                .last_mut()
                                .expect("no parent for empty tag")
                                .add_child(vnode.into());
                        }
                    }
                }
                Event::Text(s) => {
                    let node = VText::new(s.to_string());
                    stack
                        .last_mut()
                        .expect("no parent for text item")
                        .add_child(node.into());
                }
                Event::Error(e) => ConsoleService::error(&format!("error parsing svg: {}", e)),
                Event::Comment => ConsoleService::debug("got comment"),
                Event::Declaration => ConsoleService::debug("got decl"),
                Event::Instruction => ConsoleService::debug("go instruction"),
            }
        }
        let node = stack.pop().unwrap().into();
        ConsoleService::debug(&format!("final: {:#?}", node));
        node
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<ChartComponent>();
}
