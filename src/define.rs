// 窗口布局的常量定义
pub mod layout {
    // 应用名称
    pub const APP_NAME: &str = "Doya";
    // 应用版本
    pub const APP_VERSION: &str = "0.0.1";
    // 窗口宽度
    pub const WINDOW_WIDTH: i32 = 800;
    // 窗口高度
    pub const WINDOW_HEIGHT: i32 = 600;
    // 窗口 X 坐标
    pub const WINDOW_X: i32 = 500;
    // 窗口 Y 坐标
    pub const WINDOW_Y: i32 = 500;
}

// 设置基础背景色常量
pub mod color {
    pub const BASE_BACKGROUND_COLOR: &str = "#d3d4d8";
    pub const FLEX_RIGHT_WRAP_BACKGROUND_COLOR: &str = "#f5f5f5";
}

pub enum EventMsg {
    Resize,
    Parse,
}
