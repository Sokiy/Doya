use crate::define::EventMsg;
use fltk::button::Button;
use fltk::{app, prelude::*, window::Window};
use fltk::enums::FrameType;
use fltk::frame::Frame;

mod define;
mod feature;
mod layout;

fn main() {
    let app = app::App::default();
    // 定义通道
    let (tx, rx) = app::channel::<define::EventMsg>();
    // 创建窗口
    let mut wind = Window::new(
        define::layout::WINDOW_X,
        define::layout::WINDOW_Y,
        define::layout::WINDOW_WIDTH,
        define::layout::WINDOW_HEIGHT,
        define::layout::APP_NAME,
    );

    // 设置 wind 窗口样式
    layout::set_wind_style(&mut wind);

    let mut frame = Frame::default().with_size(360, 260).center_of(&wind);
    frame.set_frame(FrameType::EngravedBox);
    wind.end();
    wind.show();

    feature::tool::monitor_window_event(&mut wind, tx.clone());

    // 在主程序中监听 wind 的事件触发 send 的消息
    while app.wait() {
        if let Some(msg) = rx.recv() {
            match msg {
                EventMsg::Resize => {
                    println!("resize");
                }
                EventMsg::Parse => {
                    println!("parse");
                }
            }
        }
    }
}
