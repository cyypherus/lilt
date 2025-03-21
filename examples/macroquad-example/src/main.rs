use lilt::*;
use macroquad::prelude::*;
use std::f32::consts::PI;

#[macroquad::main("BasicShapes")]
async fn main() {
    let time = get_time() as f32 * 1000.;
    let animation1: Animated<bool, f32> = Animated::new(false)
        .duration(2000.)
        .easing(Easing::InOutQuint)
        .repeat_forever()
        .auto_reverse()
        .auto_start(true, time);

    let animation2: Animated<bool, f32> = Animated::new(false)
        .duration(2000.)
        .easing(Easing::Linear)
        .repeat_forever()
        .auto_start(true, time);

    loop {
        let time = get_time() as f32 * 1000.;
        clear_background(LIGHTGRAY);

        let rect_left = animation1.animate_bool(100., screen_width() - 200.0, time);
        let rotation = animation2.animate_bool(0., 2., time);

        let line_origin_x = 300.;
        let line_origin_y = 300.;
        let length = 400.;
        draw_line(
            line_origin_x,
            line_origin_y,
            (rotation * PI).cos() * (length - line_origin_x)
                - (rotation * PI).sin() * (length - line_origin_x)
                + line_origin_x,
            (rotation * PI).sin() * (length - line_origin_y)
                + (rotation * PI).cos() * (length - line_origin_y)
                + line_origin_y,
            15.0,
            BLUE,
        );
        draw_rectangle(rect_left, 100.0, 120.0, 60.0, GREEN);

        next_frame().await
    }
}
