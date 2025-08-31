use ratatui::layout::Rect;

use super::appscreen::AppScreen;

pub trait Dialog {
    fn get_size(&self) -> Rect;
    fn get_title(&self) -> String;

    fn as_screen(&self) -> &dyn AppScreen;
    fn as_screen_mut(&mut self) -> &mut dyn AppScreen;
}
