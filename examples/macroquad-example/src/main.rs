use std::f32::consts::PI;

use lilt::*;
use macroquad::prelude::*;


#[macroquad::main("BasicShapes")]
async fn main() {
    let time = get_time() as f32 * 1000.;
    let animation1: Animated<bool, f32> = Animated::new(false)
                        .duration(2000.)
                        .easing(Easing::EaseInOutQuint)
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

        let axeX = 300.;
        let axeY = 300.;
        let length = 400.;
        draw_line(
            axeX,
            axeY,
            (rotation * PI).cos() * (length - axeX) - (rotation * PI).sin() * (length - axeX) + axeX,
            (rotation * PI).sin() * (length - axeY) + (rotation * PI).cos() * (length - axeY) + axeY,
            15.0,
            BLUE
        );
        draw_rectangle(
            rect_left,
            100.0,
            120.0,
            60.0,
            GREEN
        );
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text("HELLO", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
