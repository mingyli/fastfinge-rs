use cursive::theme::{BaseColor, Color, Effect, Style};

pub const FAST_FINGERS: &str = "fastfinge-rs";

pub const PANEL_WIDTH: usize = 60;
pub const PANEL_ROWS: usize = 2;
pub const PERFORMANCE_WIDTH: usize = 30;
pub const PERFORMANCE_HEIGHT: usize = 5;

pub const INPUT_FILE: &str = "./input/top1000.txt";

pub const ENTRY: &str = "entry";
pub const PERFORMANCE: &str = "performance";
pub const STACK: &str = "stack";
pub const CORE: &str = "core";
pub const DISPLAY: &str = "display";

lazy_static! {
    pub static ref CORRECT_STYLE: Style = Style::from(Effect::Bold);
    pub static ref INCORRECT_STYLE: Style =
        Style::from(Effect::Reverse).combine(Color::Dark(BaseColor::Red));
    pub static ref CURRENT_STYLE: Style = Style::from(Effect::Reverse);
    pub static ref FUTURE_STYLE: Style = Style::from(Effect::Simple);
}
