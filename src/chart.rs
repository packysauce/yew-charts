use svg::node::element::tag::Type;
use svg::parser::{Event, Parser};
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services::ConsoleService;
use yew::virtual_dom::{VList, VNode, VTag, VText};
use std::rc::Rc;
pub struct ChartComponent {
    pub props: Props,
    pub width: isize,
    pub height: isize,
    group: Option<svg::node::element::Group>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Hover(yew::MouseEvent),
    Nothing,
}

#[derive(Properties, Clone, Default)]
pub struct Props {
    pub width: isize,
    pub height: isize,
    #[prop_or(0)]
    pub min_x: isize,
    #[prop_or(0)]
    pub min_y: isize,
    #[prop_or_default]
    pub on_hover: Option<Callback<(isize, isize)>>,
    #[prop_or_default]
    pub data: Vec<(isize, isize)>,
}

impl Component for ChartComponent {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let (width, height) = (props.width, props.height);
        let (top, right, bottom, left) = (90, 40, 50, 60);
        let x = charts::ScaleLinear::new()
            .set_domain(vec![0f32, 200f32])
            .set_range(vec![0, width - left - right]);
        let y = charts::ScaleLinear::new()
            .set_domain(vec![0f32, 100f32])
            .set_range(vec![height - top - bottom, 0]);
        //let line_data = vec![(12, 54), (100, 40), (120, 50), (180, 70)];
        let line_view = charts::LineSeriesView::new()
            .set_x_scale(&x)
            .set_y_scale(&y)
            .set_marker_type(charts::MarkerType::Circle)
            .set_label_position(charts::PointLabelPosition::N)
            .load_data(&props.data)
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
        match msg {
            Msg::Hover(e) => {
   
                ConsoleService::info(&format!("{:?}", e.target()));
            },
            Msg::Nothing => {},
        }
        true
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        let s = self.group.as_ref().unwrap().to_string();
        let buf = std::io::BufReader::new(s.as_bytes());
        let svg = svg::read(buf).unwrap();
        let mut stack: Vec<VTag> = Vec::new();
        let mut root = VTag::new("svg");
        root.add_attribute("width", &"100%");
        root.add_attribute("height", &"100%");
        root.add_attribute(
            "viewBox",
            &format!(
                "{} {} {} {}",
                self.props.min_x, self.props.min_y, self.props.width, self.props.height
            ),
        );
        root.add_attribute("preserveAspectRatio", &"none");
        stack.push(root);
        let mut data_idx = 0;
        for event in svg {
            match event {
                Event::Tag(tag, kind, attrs) => {
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
                            let mouseable = attrs.get("class").map(|e| e.eq("scatter-point")).unwrap_or(false);
                            let mut attributes: Vec<(String, String)> = attrs
                                .iter()
                                .map(|(k, v)| (k.to_string(), v.to_string()))
                                .collect();
                            attributes.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));
                            vnode.add_attributes(attributes);
                            if self.props.on_hover.is_some() {
                                let wrapper = yew::html::onmouseover::Wrapper::new(self.link.callback(Msg::Hover));
                                if mouseable {
                                    vnode.add_listener(Rc::new(wrapper));
                                }
                            }
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
        node
    }
}
