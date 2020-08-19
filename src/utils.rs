use derive_deref::{Deref, DerefMut};
use plotters::drawing::backend::{BackendCoord, BackendStyle};
use plotters::drawing::{backend::DrawingErrorKind, DrawingBackend};
use plotters::style::text_anchor::{HPos, Pos, VPos};
use plotters::style::{FontTransform, FontStyle, Color};
use thiserror::Error;
use yew::prelude::*;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("They mostly come out at night... mostly")]
    Unknown,
    #[error("Path is empty")]
    NotEnoughPoints,
    #[error("Failed to write png")]
    PngError(#[from] png::EncodingError),
}

fn make_svg_color<C: Color>(color: &C) -> String {
    let (r, g, b) = color.rgb();
    return format!("#{:02X}{:02X}{:02X}", r, g, b);
}

fn make_style<S: BackendStyle>(style: &S) -> String {
    format!(
        "stroke:{}; stroke-width:{}",
        make_svg_color(&style.as_color().to_rgba()),
        style.stroke_width()
    )
}

#[derive(Deref, DerefMut)]
pub struct VTagWrapper<'a>(pub &'a mut yew::virtual_dom::VTag);

impl<'a> DrawingBackend for VTagWrapper<'a> {
    type ErrorType = Error;

    fn get_size(&self) -> (u32, u32) {
        (800, 600)
    }

    fn ensure_prepared(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        Ok(())
    }

    fn present(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        Ok(())
    }

    fn draw_pixel(
        &mut self,
        point: BackendCoord,
        color: &plotters::style::RGBAColor,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        if color.alpha() == 0.0 {
            return Ok(());
        }
        let rgb = color.rgb();
        self.add_child(html! {
            <rect
                x=point.0
                y=point.1
                width=1
                height=1
                stroke="none"
                opacity=color.alpha()
                fill=&format!("#{:02X}{:02X}{:02X}", rgb.0, rgb.1, rgb.2)
            />
        });
        Ok(())
    }

    fn draw_line<S: BackendStyle>(
        &mut self,
        from: BackendCoord,
        to: BackendCoord,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        self.add_child(html! {
            <line
                opacity=style.as_color().alpha()
                stroke=make_svg_color(&style.as_color())
                stroke_width=style.stroke_width()
                x1=from.0
                y1=from.1
                x2=to.0,
                y2=from.1
            />
        });
        //use plotters::drawing::SVGBackend::draw_line
        Ok(())
    }

    fn draw_rect<S: BackendStyle>(
        &mut self,
        upper_left: BackendCoord,
        bottom_right: BackendCoord,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let (x, y) = upper_left;
        let width = bottom_right.0 - x;
        let height = bottom_right.1 - y;
        let (fill, stroke) = if !fill {
            ("none".to_string(), make_svg_color(&style.as_color()))
        } else {
            (make_svg_color(&style.as_color()), "none".to_string())
        };
        self.add_child(html! {
            <rect
                x=x
                y=y
                width=width
                height=height
                opacity=style.as_color().alpha()
                fill=fill
                stroke=stroke
            />
        });
        Ok(())
    }

    fn draw_path<
        S: BackendStyle,
        I: IntoIterator<Item = BackendCoord>,
    >(
        &mut self,
        path: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        if style.as_color().alpha() == 0.0 {
            return Ok(());
        }

        let mut iter = path.into_iter();
        let (startx, starty) = iter
            .next()
            .ok_or(DrawingErrorKind::DrawingError(Error::NotEnoughPoints))?;
        let rest = iter
            .map(|(x, y)| format!("L{},{}", x, y))
            .collect::<Vec<String>>()
            .join(" ");
        let path = format!("M{},{} {}", startx, starty, rest);
        self.add_child(html! {
            <path
                d=path
                fill="none"
                opacity=style.as_color().alpha()
                stroke=make_svg_color(&style.as_color())
                stroke-width=style.stroke_width()
            />
        });
        Ok(())
    }

    fn draw_circle<S: BackendStyle>(
        &mut self,
        center: BackendCoord,
        radius: u32,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let (stroke, fill) = if !fill {
            (make_svg_color(&style.as_color()), "none".to_string())
        } else {
            ("none".to_string(), make_svg_color(&style.as_color()))
        };
        self.add_child(html! {
            <circle
                cx=center.0
                cy=center.1
                r=radius
                stroke=stroke
                fill=fill
                opacity=style.as_color().alpha()
            />
        });
        Ok(())
    }

    fn fill_polygon<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        vert: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let points = vert
            .into_iter()
            .map(|(x, y)| format!("{},{}", x, y))
            .collect::<Vec<String>>()
            .join(" ");

        self.add_child(html! {
            <polygon
                points=points
                opacity=style.as_color().alpha()
                fill=make_svg_color(&style.as_color())
            />
        });
        Ok(())
    }

    fn draw_text(
        &mut self,
        text: &str,
        style: &plotters::style::TextStyle,
        pos: BackendCoord,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let font = &style.font;
        let color = &style.color;
        if color.alpha() == 0.0 {
            return Ok(());
        }

        let (x0, y0) = pos;
        let text_anchor = match style.pos.h_pos {
            HPos::Left => "start",
            HPos::Right => "end",
            HPos::Center => "middle",
        };

        let dy = match style.pos.v_pos {
            VPos::Top => "0.76em",
            VPos::Center => "0.5ex",
            VPos::Bottom => "-0.5ex",
        };

        let font_weight = match font.get_style() {
            FontStyle::Bold => "bold",
            _ => "normal",
        }.to_string();

        let font_style = match font.get_style() {
            FontStyle::Bold => "normal".to_string(),
            other => other.as_str().to_string(),
        };

        let trans = font.get_transform();
        let transform = match trans {
            FontTransform::Rotate90 => {
                format!("rotate(90, {}, {})", x0, y0)
            }
            FontTransform::Rotate180 => {
                format!("rotate(180, {}, {})", x0, y0)
            }
            FontTransform::Rotate270 => {
                format!("rotate(270, {}, {})", x0, y0)
            }
            _ => "".to_string()
        };


        self.add_child(html! {
            <text
                x=pos.0
                y=pos.1
                dy=dy
                text-anchor=text_anchor
                font-family=font.get_name().to_string()
                font-size=font.get_size() / 1.24
                opacity=color.alpha()
                fill=make_svg_color(color)
                font-weight=font_weight
                font-style=font_style
                transform=transform
            >
                {text.to_string()}
            </text>
        });

        Ok(())
    }

    fn blit_bitmap<'b>(
        &mut self,
        pos: BackendCoord,
        (iw, ih): (u32, u32),
        src: &'b [u8],
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let mut buf_inner = Vec::new();
        {
            let buf = std::io::BufWriter::new(&mut buf_inner);
            let mut e = png::Encoder::new(buf, iw, ih);
            e.set_color(png::ColorType::RGB);
            e.set_depth(png::BitDepth::Eight);
            let mut writer = e
                .write_header()
                .map_err(|e| DrawingErrorKind::DrawingError(Error::PngError(e)))?;
            writer.write_image_data(src)
                .map_err(|e| DrawingErrorKind::DrawingError(Error::PngError(e)))?;
        }
        let data = format!("data:image/png;base64,{}", base64::encode(&buf_inner));
        self.add_child(html!{
            <image
                x=pos.0
                y=pos.1
                href=data
            />
        });
        Ok(())
    }
}
