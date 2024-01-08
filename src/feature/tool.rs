use crate::define;
use fltk::{app, enums::Event, image::SharedImage, prelude::*, *};
use std::time::{Duration, Instant};

///监听 wind 的各种事件
pub fn monitor_window_event(wind: &mut window::DoubleWindow, send: app::Sender<define::EventMsg>) {
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
                send.send(define::EventMsg::Resize);
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
                send.send(define::EventMsg::Parse);
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
