use fltk::{app, enums::Event, image::SharedImage, prelude::*, *};
use std::time::{Duration, Instant};

#[derive(Debug)]
enum Msg {
    Resize,
    Parse,
}

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = create_window();

    // Create a shared image
    let path = "assets/noob.png";
    let mut img = load_image(path);

    // declare send rec channel
    let (send, rec) = app::channel();

    // first time draw the window
    draw_image(&mut wind, &mut img);
    // monitor the window event
    monitor_window_event(&mut wind, send);

    wind.make_resizable(true);
    wind.end();
    wind.show();

    // 在主程序中监听 wind 的事件触发 send 的消息
    while app.wait() {
        if let Some(msg) = rec.recv() {
            match msg {
                Msg::Resize => draw_image(&mut wind, &mut img),
                // Msg::Parse => println!("{}", app::event_text()),
                Msg::Parse => {
                    // 在第一次触发的时候,我们需要判断文件是否存在
                    let path = std::path::PathBuf::from(&app::event_text());
                    println!("{}", path.display());
                    if path.exists() {
                        println!("{}", app::event_text());
                        img = load_image(app::event_text().as_str());
                        draw_image(&mut wind, &mut img);
                    }
                }
            }
        }
    }

    app.run().unwrap();
}

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

///draw image
fn draw_image(wind: &mut window::DoubleWindow, img: &mut image::SharedImage) {
    let mut img = img.clone();

    wind.draw(move |f| {
        let (wind_w, wind_h) = (f.w() as f64, f.h() as f64);
        let (img_w, img_h) = (img.w() as f64, img.h() as f64);
        let (pos_x, pos_y) = calc_pos(&wind_w, &wind_h, &img_w, &img_h);
        img.scale(f.w(), f.h(), true, true);
        img.draw(pos_x, pos_y, img.w(), img.h());

        println!(
            "wind_w:{}, wind_h:{}, img_w:{}, img_h:{},pos_x:{}, pos_y:{}",
            wind_w, wind_h, img_w, img_h, pos_x, pos_y
        );
    });

    wind.redraw();
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
    (pos_x, pos_y)
}

///监听 wind 的各种事件
fn monitor_window_event(wind: &mut window::DoubleWindow, send: app::Sender<Msg>) {
    //获取上次重绘时间,在重绘之后更新重绘时间
    let mut last_resize_time = Instant::now();
    //设置每次重绘的阈值
    let resize_duration = Duration::from_millis(200);
    //是否已经进入
    let mut dnd = false;
    //是否已经释放
    let mut relased = false;

    wind.handle(move |_, event| match event {
        Event::Resize => {
            if last_resize_time.elapsed() > resize_duration {
                last_resize_time = Instant::now();
                send.send(Msg::Resize);
            }
            true
        }
        //文件进入窗体  返回 true 表示已经被处理，不然捕获不到后续的事件
        Event::DndEnter => {
            dnd = true;
            true
        }
        //文件拖拽
        Event::DndDrag => true,
        //文件在窗体中释放
        Event::DndRelease => {
            relased = true;
            true
        }
        //文件粘贴完成, 进行重绘操作
        Event::Paste => {
            if dnd && relased {
                //发送粘贴操作信号
                send.send(Msg::Parse);
                dnd = false;
                relased = false;
            }
            true
        }
        //文件被拖出窗体
        Event::DndLeave => true,
        _ => false,
    });
}
