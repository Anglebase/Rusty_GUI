use std::{f32::consts::PI, os::raw::c_void, ptr::null_mut};

use winapi::{
    shared::windef::{RECT, SIZE},
    um::{wingdi::*, winuser::*},
};

use crate::*;

pub fn clear_device(hdc: *mut c_void, rect: Rect, color: Color) {
    unsafe {
        let rect = RECT {
            left: 0,
            top: 0,
            right: rect.size.width,
            bottom: rect.size.height,
        };
        let color = RGB(color.red, color.green, color.blue);
        let brush = CreateSolidBrush(color);
        FillRect(hdc as _, &rect, brush as _);
        DeleteObject(brush as _);
    }
}

pub fn delete_object(obj: *mut c_void) {
    unsafe {
        DeleteObject(obj as _);
    }
}

pub fn select_object(hdc: *mut c_void, obj: *mut c_void) -> *mut c_void {
    unsafe { SelectObject(hdc as _, obj as _) as *mut c_void }
}

pub fn set_current_text_color(hdc: *mut c_void, color: Color) -> Color {
    let ret = unsafe { SetTextColor(hdc as _, RGB(color.red, color.green, color.blue)) };
    rgb!(GetRValue(ret), GetGValue(ret), GetBValue(ret))
}

pub fn new_pen_object(pen_style: PenStyle) -> *mut c_void {
    let end_cap = match pen_style.cap_style {
        CapStyle::Flat => PS_ENDCAP_FLAT,
        CapStyle::Square => PS_ENDCAP_SQUARE,
        CapStyle::Round => PS_ENDCAP_ROUND,
    };
    let join = match pen_style.join_style {
        JoinStyle::Miter => PS_JOIN_MITER,
        JoinStyle::Bevel => PS_JOIN_BEVEL,
        JoinStyle::Round => PS_JOIN_ROUND,
    };
    let style = match pen_style.line_style {
        LineStyle::Solid => PS_SOLID,
        LineStyle::Dash => PS_DASH,
        LineStyle::Dot => PS_DOT,
        LineStyle::DashDot => PS_DASHDOT,
        LineStyle::DashDotDot => PS_DASHDOTDOT,
        LineStyle::Null => PS_NULL,
    };
    let brush = LOGBRUSH {
        lbStyle: BS_SOLID,
        lbColor: RGB(
            pen_style.color.red,
            pen_style.color.green,
            pen_style.color.blue,
        ),
        lbHatch: 0,
    };
    unsafe {
        ExtCreatePen(
            style | end_cap | join | PS_GEOMETRIC,
            pen_style.width,
            &brush as *const LOGBRUSH as _,
            0,
            null_mut(),
        ) as *mut c_void
    }
}

pub fn new_font_object(style: FontStyle) -> *mut c_void {
    let family = style
        .font
        .to_string()
        .encode_utf16()
        .chain(Some(0))
        .collect::<Vec<u16>>();
    unsafe {
        CreateFontW(
            style.size,
            0,
            0,
            0,
            style.weight as i32,
            style.italic as u32,
            style.underline as u32,
            style.strikeout as u32,
            DEFAULT_CHARSET,
            OUT_DEFAULT_PRECIS,
            CLIP_DEFAULT_PRECIS,
            DEFAULT_QUALITY,
            DEFAULT_PITCH | FF_DONTCARE,
            family.as_ptr(),
        ) as _
    }
}

pub enum BrushParam {
    Solid(Color),
}

pub fn new_brush_object(param: BrushParam) -> *mut c_void {
    match param {
        BrushParam::Solid(color) => {
            let color = RGB(color.red, color.green, color.blue);
            unsafe { CreateSolidBrush(color) as *mut c_void }
        }
    }
}

pub fn draw_line(hdc: *mut c_void, x1: i32, y1: i32, x2: i32, y2: i32) {
    unsafe {
        MoveToEx(hdc as _, x1, y1, null_mut());
        LineTo(hdc as _, x2, y2);
    }
}

pub fn draw_rect(hdc: *mut c_void, rect: Rect) {
    let (x, y, w, h) = rect.into();
    unsafe {
        MoveToEx(hdc as _, x, y, null_mut());
        LineTo(hdc as _, x + w, y);
        LineTo(hdc as _, x + w, y + h);
        LineTo(hdc as _, x, y + h);
        LineTo(hdc as _, x, y);
    }
}

pub fn draw_round_rect(hdc: *mut c_void, rect: Rect, rx: i32, ry: i32) {
    let (x, y, w, h) = rect.into();
    let (left, top, right, bottom) = (x, y, x + w, y + h);
    unsafe {
        MoveToEx(hdc as _, left + rx, top, null_mut());
        ArcTo(
            hdc as _,
            left,
            top,
            left + 2 * rx,
            top + 2 * ry,
            left + rx,
            top,
            left,
            top + rx,
        );
        LineTo(hdc as _, left, bottom - ry);
        ArcTo(
            hdc as _,
            left,
            bottom - 2 * ry,
            left + 2 * rx,
            bottom,
            left,
            bottom - ry,
            left + rx,
            bottom,
        );
        LineTo(hdc as _, right - rx, bottom);
        ArcTo(
            hdc as _,
            right - 2 * rx,
            bottom,
            right,
            bottom - 2 * ry,
            right - rx,
            bottom,
            right,
            bottom - ry,
        );
        LineTo(hdc as _, right, top + ry);
        ArcTo(
            hdc as _,
            right - 2 * rx,
            top,
            right,
            top + 2 * ry,
            right,
            top + ry,
            right - rx,
            top,
        );
        LineTo(hdc as _, left + rx, top);
    }
}

pub fn draw_polyline(hdc: *mut c_void, points: &[Point]) {
    if points.len() < 2 {
        panic!("At least two points are required to draw a polyline");
    }
    unsafe { MoveToEx(hdc as _, points[0].x, points[0].y, null_mut()) };
    for Point { x, y } in points.iter() {
        unsafe { LineTo(hdc as _, *x, *y) };
    }
}

pub fn draw_polygon(hdc: *mut c_void, points: &[Point]) {
    if points.len() < 2 {
        panic!("At least two points are required to draw a polygon");
    }
    unsafe { MoveToEx(hdc as _, points[0].x, points[0].y, null_mut()) };
    for Point { x, y } in points.iter() {
        unsafe { LineTo(hdc as _, *x, *y) };
    }
    unsafe { LineTo(hdc as _, points[0].x, points[0].y) };
}

pub fn draw_arc(hdc: *mut c_void, rect: Rect, start: f32, sweep: f32) {
    let (x1, y1, w, h) = rect.into();
    let (x2, y2) = (x1 + w, y1 + h);
    let (x, y) = (x1 + w / 2, y1 + h / 2);
    let (x3, y3) = (
        x + (100.0 * f32::cos(start)) as i32,
        y + (100.0 * f32::sin(start)) as i32,
    );
    let (x4, y4) = (
        x + (100.0 * f32::cos(start + sweep)) as i32,
        y + (100.0 * f32::sin(start + sweep)) as i32,
    );
    unsafe {
        Arc(hdc as _, x1, y1, x2, y2, x3, y3, x4, y4);
    }
}

pub fn draw_pie(hdc: *mut c_void, rect: Rect, start: f32, sweep: f32) {
    let (left, top, w, h) = rect.into();
    let (right, bottom) = (left + w, top + h);
    let (x, y) = (left + w / 2, top + h / 2);
    let (xr1, yr1) = (
        x + (100.0 * f32::cos(start)) as i32,
        y + (100.0 * f32::sin(start)) as i32,
    );
    let (xr2, yr2) = (
        x + (100.0 * f32::cos(start + sweep)) as i32,
        y + (100.0 * f32::sin(start + sweep)) as i32,
    );
    unsafe {
        MoveToEx(hdc as _, x, y, null_mut());
        ArcTo(hdc as _, left, top, right, bottom, xr1, yr1, xr2, yr2);
        LineTo(hdc as _, x, y);
    }
}

pub fn draw_ellipse(hdc: *mut c_void, rect: Rect) {
    draw_arc(hdc as _, rect, 0.0, 2.0 * PI);
}

pub fn draw_circle(hdc: *mut c_void, pos: Point, radius: i32) {
    let rect = rect!(pos.x - radius, pos.y - radius, 2 * radius, 2 * radius);
    draw_ellipse(hdc as _, rect);
}

pub fn draw_fill_rect(hdc: *mut c_void, rect: Rect) {
    let (x, y, w, h) = rect.into();
    let (left, top, right, bottom) = (x, y, x + w, y + h);
    unsafe {
        Rectangle(hdc as _, left, top, right, bottom);
    }
}

pub fn draw_fill_round_rect(hdc: *mut c_void, rect: Rect, rx: i32, ry: i32) {
    let (x, y, w, h) = rect.into();
    let (left, top, right, bottom) = (x, y, x + w, y + h);
    unsafe {
        RoundRect(hdc as _, left, top, right, bottom, rx * 2, ry * 2);
    }
}

pub fn draw_fill_polygon(hdc: *mut c_void, points: &[Point]) {
    if points.len() < 2 {
        panic!("At least two points are required to draw a filled polygon");
    }
    unsafe {
        Polygon(
            hdc as _,
            points.as_ptr() as _,
            points.len().try_into().unwrap(),
        );
    }
}

pub fn draw_fill_pie(hdc: *mut c_void, rect: Rect, start: f32, sweep: f32) {
    let (left, top, w, h) = rect.into();
    let (right, bottom) = (left + w, top + h);
    let (x, y) = (left + w / 2, top + h / 2);
    let (xr1, yr1) = (
        x + (100.0 * f32::cos(start)) as i32,
        y + (100.0 * f32::sin(start)) as i32,
    );
    let (xr2, yr2) = (
        x + (100.0 * f32::cos(start + sweep)) as i32,
        y + (100.0 * f32::sin(start + sweep)) as i32,
    );
    unsafe {
        Pie(hdc as _, left, top, right, bottom, xr1, yr1, xr2, yr2);
    }
}

pub fn draw_fill_ellipse(hdc: *mut c_void, rect: Rect) {
    let (x, y, w, h) = rect.into();
    let (left, top, right, bottom) = (x, y, x + w, y + h);
    unsafe {
        Ellipse(hdc as _, left, top, right, bottom);
    }
}

pub fn draw_fill_circle(hdc: *mut c_void, pos: Point, radius: i32) {
    let rect = rect!(pos.x - radius, pos.y - radius, 2 * radius, 2 * radius);
    draw_fill_ellipse(hdc as _, rect);
}

pub fn draw_xy_text(hdc: *mut c_void, pos: Point, text: &str, align: TextAlign) {
    let text = text
        .to_string()
        .encode_utf16()
        .chain(Some(0))
        .collect::<Vec<u16>>();
    let (x, y) = (pos.x, pos.y);
    let align = match align {
        TextAlign::LeftTop => TA_LEFT | TA_TOP,
        TextAlign::LeftBottom => TA_LEFT | TA_BOTTOM,
        TextAlign::CenterTop => TA_CENTER | TA_TOP,
        TextAlign::CenterBottom => TA_CENTER | TA_BOTTOM,
        TextAlign::RightTop => TA_RIGHT | TA_TOP,
        TextAlign::RightBottom => TA_RIGHT | TA_BOTTOM,
        _ => {
            panic!("Unsupported text align {:?}", align);
        }
    };
    unsafe {
        SetTextAlign(hdc as _, align);
        TextOutW(
            hdc as _,
            x,
            y,
            text.as_ptr() as _,
            (text.len() - 1).try_into().unwrap(),
        );
    }
}

pub fn draw_rect_text(hdc: *mut c_void, rect: Rect, text: &str, align: TextAlign) {
    let text = text
        .to_string()
        .encode_utf16()
        .chain(Some(0))
        .collect::<Vec<u16>>();
    let (x, y, w, h) = rect.into();
    let mut rect = RECT {
        left: x,
        top: y,
        right: x + w,
        bottom: y + h,
    };
    let align = match align {
        TextAlign::LeftTop => DT_LEFT | DT_TOP,
        TextAlign::LeftMiddle => DT_LEFT | DT_VCENTER,
        TextAlign::LeftBottom => DT_LEFT | DT_BOTTOM,
        TextAlign::CenterTop => DT_CENTER | DT_TOP,
        TextAlign::Center => DT_CENTER | DT_VCENTER,
        TextAlign::CenterBottom => DT_CENTER | DT_BOTTOM,
        TextAlign::RightTop => DT_RIGHT | DT_TOP,
        TextAlign::RightMiddle => DT_RIGHT | DT_VCENTER,
        TextAlign::RightBottom => DT_RIGHT | DT_BOTTOM,
    };
    unsafe {
        DrawTextW(
            hdc as _,
            text.as_ptr() as _,
            (text.len() - 1).try_into().unwrap(),
            &mut rect,
            align | DT_SINGLELINE,
        );
    }
}

pub fn begin_path(hdc: *mut c_void) {
    unsafe {
        BeginPath(hdc as _);
    }
}

pub fn end_path(hdc: *mut c_void) {
    unsafe {
        EndPath(hdc as _);
    }
}

pub fn move_to(hdc: *mut c_void, x: i32, y: i32) {
    unsafe {
        MoveToEx(hdc as _, x, y, null_mut());
    }
}

pub fn line_to(hdc: *mut c_void, x: i32, y: i32) {
    unsafe {
        LineTo(hdc as _, x, y);
    }
}

pub fn draw_path_fill(hdc: *mut c_void) {
    unsafe {
        FillPath(hdc as _);
    }
}

pub fn draw_path_frame(hdc: *mut c_void) {
    unsafe {
        StrokePath(hdc as _);
    }
}

pub fn draw_path_frame_and_fill(hdc: *mut c_void) {
    unsafe {
        StrokeAndFillPath(hdc as _);
    }
}

pub fn draw_rad_arc(hdc: *mut c_void, x: i32, y: i32, radius: i32, start: f32, sweep: f32) {
    unsafe {
        AngleArc(
            hdc as _,
            x,
            y,
            radius as _,
            start / PI * 180.0,
            sweep / PI * 180.0,
        );
    }
}

pub fn arc_to(hdc: *mut c_void, rect: Rect, start: f32, sweep: f32) {
    let (left, top, w, h) = rect.into();
    let (right, bottom) = (left + w, top + h);
    let (x, y) = (left + w / 2, top + h / 2);
    let (xr1, yr1) = (
        x + (100.0 * f32::cos(start)) as i32,
        y + (100.0 * f32::sin(start)) as i32,
    );
    let (xr2, yr2) = (
        x + (100.0 * f32::cos(start + sweep)) as i32,
        y + (100.0 * f32::sin(start + sweep)) as i32,
    );
    unsafe {
        ArcTo(hdc as _, left, top, right, bottom, xr1, yr1, xr2, yr2);
    }
}

pub fn draw_fill_chord(hdc: *mut c_void, rect: Rect, start: f32, sweep: f32) {
    let (left, top, w, h) = rect.into();
    let (right, bottom) = (left + w, top + h);
    let (x, y) = (left + w / 2, top + h / 2);
    let (xr1, yr1) = (
        x + (100.0 * f32::cos(start)) as i32,
        y + (100.0 * f32::sin(start)) as i32,
    );
    let (xr2, yr2) = (
        x + (100.0 * f32::cos(start + sweep)) as i32,
        y + (100.0 * f32::sin(start + sweep)) as i32,
    );
    unsafe {
        Chord(hdc as _, left, top, right, bottom, xr1, yr1, xr2, yr2);
    }
}

pub fn draw_chord(hdc: *mut c_void, rect: Rect, start: f32, sweep: f32) {
    let (left, top, w, h) = rect.into();
    let (right, bottom) = (left + w, top + h);
    let (x, y) = (left + w / 2, top + h / 2);
    let (xr1, yr1) = (
        x + (100.0 * f32::cos(start)) as i32,
        y + (100.0 * f32::sin(start)) as i32,
    );
    let (xr2, yr2) = (
        x + (100.0 * f32::cos(start + sweep)) as i32,
        y + (100.0 * f32::sin(start + sweep)) as i32,
    );
    unsafe {
        MoveToEx(hdc as _, xr1, yr1, null_mut());
        ArcTo(hdc as _, left, top, right, bottom, xr1, yr1, xr2, yr2);
        LineTo(hdc as _, xr1, yr1);
    }
}

pub fn draw_bezier_curve(hdc: *mut c_void, points: &[Point]) {
    if points.len() < 4 || points.len() % 3 != 1 {
        panic!("3n+1(n>=1) points are required to draw a bezier curve!");
    }
    unsafe {
        PolyBezier(
            hdc as _,
            points.as_ptr() as _,
            points.len().try_into().unwrap(),
        );
    }
}

pub fn bezier_curve_to(hdc: *mut c_void, points: &[Point]) {
    if points.len() < 3 || points.len() % 3 != 0 {
        panic!("3n points are required to draw a bezier curve!");
    }
    unsafe {
        PolyBezierTo(
            hdc as _,
            points.as_ptr() as _,
            points.len().try_into().unwrap(),
        );
    }
}

pub fn polyline_to(hdc: *mut c_void, points: &[Point]) {
    unsafe {
        PolylineTo(
            hdc as _,
            points.as_ptr() as _,
            points.len().try_into().unwrap(),
        );
    }
}

pub fn set_fill_mode(hdc: *mut c_void, mode: PathFillMode) {
    unsafe {
        SetPolyFillMode(
            hdc as _,
            match mode {
                PathFillMode::Alternate => ALTERNATE,
                PathFillMode::Winding => WINDING,
            },
        );
    }
}

pub fn calc_text_size(hdc: *mut c_void, text: &str) -> Size {
    let text = text
        .to_string()
        .encode_utf16()
        .chain(Some(0))
        .collect::<Vec<u16>>();
    let mut size = SIZE { cx: 0, cy: 0 };
    unsafe {
        GetTextExtentPoint32W(
            hdc as _,
            text.as_ptr(),
            text.len().try_into().unwrap(),
            &mut size as *mut _,
        );
    }
    Size {
        width: size.cx,
        height: size.cy,
    }
}
