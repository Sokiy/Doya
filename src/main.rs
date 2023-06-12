use fltk::{app, enums::Event, image::SharedImage, prelude::*, *};
use std::time::{Duration, Instant};
///create a window
fn create_window() -> window::Window {
    const WIDTH: i32 = 500;
    const HEIGHT: i32 = 500;
    const TITLE: &str = "Doya";
    let wind = window::Window::new(0, 0, WIDTH, HEIGHT, TITLE).center_screen();
    wind
}

/// use SharedImage load image from path
fn load_image(path: &str) -> image::SharedImage {
    println!("load image from {}", path);
    SharedImage::load(path).unwrap()
}

/// calc img position
fn calc_pos(wind_w: &f64, wind_h: &f64, img_w: &f64, img_h: &f64) -> (i32, i32) {
    let wind_ratio = wind_w / wind_h;
    let img_ratio = img_w / img_h;

    let mut pos_x = 0;
    let mut pos_y = 0;

    if wind_ratio > img_ratio {
        //计算缩放后的图片宽度
        let compute_img_w = img_w * wind_h / img_h;
        pos_x = ((wind_w - compute_img_w).abs() / 2.0).round() as i32;
    } else {
        //计算图片后的缩放高度
        let compute_img_h = wind_w * img_h / img_w;
        pos_y = ((wind_h - compute_img_h).abs() / 2.0).round() as i32;
    }
    // println!("window_w:{}, window_h:{}, img_w:{}, img_h:{},wind_ratio:{}, img_ratio:{}, pos_x:{}, pos_y:{}", wind_w, wind_h, img_w, img_h, wind_ratio, img_ratio, pos_x, pos_y);

    (pos_x, pos_y)
}

///监听 wind 的各种事件
fn monitor_window_event(wind: &mut window::DoubleWindow, send: app::Sender<bool>) {
    //获取上次重绘时间,在重绘之后更新重绘时间
    let mut last_resize_time = Instant::now();
    //设置每次重绘的阈值
    let resize_duration = Duration::from_millis(100);

    wind.handle(move |_, event| match event {
        Event::Resize => {
            if last_resize_time.elapsed() > resize_duration {
                last_resize_time = Instant::now();
                send.send(true);
            }
            true
        }
        _ => false,
    });
}

//draw image
fn draw_image(wind: &mut window::DoubleWindow, img: &mut image::SharedImage) {
    let (wind_w, wind_h, img_w, img_h) = (
        wind.w() as f64,
        wind.h() as f64,
        img.w() as f64,
        img.h() as f64,
    );
    let (pos_x, pos_y) = calc_pos(&wind_w, &wind_h, &img_w, &img_h);

    let mut img = img.clone();

    wind.draw(move |f| {
        img.scale(f.w(), f.h(), true, true);
        img.draw(pos_x, pos_y, img.w(), img.h());
        println!(
            "wind_w:{}, wind_h:{}, img_w:{}, img_h:{},pos_x:{}, pos_y:{}",
            wind_w, wind_h, img_w, img_h, pos_x, pos_y
        );
    });

    wind.redraw();
}

// fn resize_callback(w: &mut window::Window, _x: i32, _y: i32, _w: i32, _h: i32) {
//     println!("Window resized to ({}, {})", w.width(), w.height());
// }

fn main() {
    let app = app::App::default();
    let mut wind = create_window();
    let path = "assets/noob.png";
    let mut img = load_image(path);

    let (send, rec) = app::channel();

    draw_image(&mut wind, &mut img);

    // wind.resize_callback(Box::new(resize_callback));
    monitor_window_event(&mut wind, send);

    wind.make_resizable(true);
    wind.end();
    wind.show();

    // 在主程序中监听 wind 的事件触发 send 的消息
    while app.wait() {
        if let Some(msg) = rec.recv() {
            println!("{}", msg);
            draw_image(&mut wind, &mut img);
        }
    }

    app.run().unwrap();
}
