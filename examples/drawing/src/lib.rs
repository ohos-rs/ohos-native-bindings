use napi_derive_ohos::napi;
use ohos_drawing_binding::{
    argb, AlphaFormat, Bitmap, BitmapFormat, Brush, Canvas, ColorFormat, FontCollection, FontStyle,
    FontWeight, Matrix, Path, PathDirection, Pen, Point, Rect, RoundRect, TextStyle,
    TypographyBuilder, TypographyStyle,
};

#[napi]
pub fn smoke() -> String {
    let bitmap = Bitmap::new(
        64,
        48,
        BitmapFormat {
            color: ColorFormat::Rgba8888,
            alpha: AlphaFormat::Premul,
        },
    );
    let bw = bitmap.width();
    let bh = bitmap.height();
    let pixel_len = bitmap.pixels().len();

    let canvas = Canvas::with_bitmap(bitmap);
    canvas.clear(argb(255, 20, 20, 20));
    canvas.save();
    canvas.translate(2.0, 2.0);
    canvas.scale(1.0, 1.0);

    let mut brush = Brush::new();
    brush.set_anti_alias(true);
    brush.set_color(argb(255, 200, 40, 40));
    canvas.attach_brush(&brush);
    let rect = Rect::new(4.0, 4.0, 40.0, 28.0);
    canvas.draw_rect(&rect);
    let round = RoundRect::new(&Rect::new(8.0, 8.0, 28.0, 24.0), 4.0, 4.0);
    canvas.draw_round_rect(&round);
    let center = Point::new(48.0, 16.0);
    canvas.draw_circle(&center, 8.0);
    canvas.detach_brush();

    let mut pen = Pen::new();
    pen.set_anti_alias(true);
    pen.set_color(argb(255, 40, 180, 80));
    pen.set_width(2.0);
    canvas.attach_pen(&pen);
    canvas.draw_line(0.0, 0.0, 63.0, 47.0);
    canvas.draw_oval(&Rect::new(12.0, 30.0, 52.0, 44.0));
    canvas.detach_pen();

    let mut path = Path::new();
    path.move_to(2.0, 40.0);
    path.line_to(20.0, 46.0);
    path.quadratic_curve_to(30.0, 30.0, 40.0, 46.0);
    path.close();
    path.add_circle(50.0, 40.0, 4.0, PathDirection::Cw);
    let cloned = path.clone_path();
    let contains = path.contains(10.0, 42.0);
    let length = path.length(true);
    canvas.attach_brush(&brush);
    canvas.draw_path(&path);
    canvas.draw_path(&cloned);
    canvas.detach_brush();

    let matrix = Matrix::from_affine(1.0, 0.0, 0.0, 1.0, 3.0, 1.0);
    canvas.concat(&matrix);
    canvas.restore();
    let (rx, ry) = round.corner(ohos_drawing_binding::CornerPosition::TopLeft);

    format!(
        "bitmap={bw}x{bh} pixels={pixel_len} canvas={}x{} rect={}x{} round_corner={rx},{ry} path_len={length:.1} contains={contains} brush_aa={} pen_w={}",
        canvas.width(),
        canvas.height(),
        rect.width(),
        rect.height(),
        brush.is_anti_alias(),
        pen.width(),
    )
}

/// Native metrics for the same fixed-height monospace label on two lines.
#[napi(object)]
pub struct TextLineHeightMetrics {
    pub height: f64,
    pub width: f64,
    pub top: f64,
    pub bottom: f64,
    pub second_top: f64,
}

/// Omitting `half_leading` measures the native default for comparison.
#[napi]
pub fn measure_text_line_height(
    line_height: f64,
    half_leading: Option<bool>,
) -> napi_ohos::Result<TextLineHeightMetrics> {
    let mut fonts = FontCollection::shared();
    let mut paragraph_style = TypographyStyle::new();
    let mut style = TextStyle::new();
    style.set_font_size(11.0);
    style.set_font_families(&["monospace"]);
    style.set_font_style(FontStyle {
        weight: FontWeight::W600,
        ..FontStyle::default()
    });
    style
        .set_font_height(line_height / 11.0)
        .map_err(|error| napi_ohos::Error::from_reason(format!("line height: {error:?}")))?;
    if let Some(enabled) = half_leading {
        style.set_half_leading(enabled);
    }
    let mut builder = TypographyBuilder::new(&mut paragraph_style, &mut fonts);
    builder.push_text_style(&mut style);
    builder.add_text("42.18\n42.18");
    builder.pop_text_style();
    let mut typography = builder.build();
    typography.layout(200.0);
    let first = typography.range_rectangles(0, 5);
    let second = typography.range_rectangles(6, 11);
    if typography.line_count() != 2 || first.len() != 1 || second.len() != 1 {
        return Err(napi_ohos::Error::from_reason(format!(
            "expected two text lines, got {} lines with {} and {} ranges",
            typography.line_count(),
            first.len(),
            second.len(),
        )));
    }
    Ok(TextLineHeightMetrics {
        height: typography.height(),
        width: typography.longest_line(),
        top: f64::from(first[0][1]),
        bottom: f64::from(first[0][3]),
        second_top: f64::from(second[0][1]),
    })
}
