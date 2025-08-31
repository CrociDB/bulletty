use ratatui::layout::Rect;

use super::appscreen::AppScreen;

pub trait Dialog {
    /// Get the sizes of dialog
    ///  - x, y: min size, if zero, uses the percentage
    ///  - width, height: percentage size
    fn get_size(&self) -> Rect;

    fn as_screen(&self) -> &dyn AppScreen;
    fn as_screen_mut(&mut self) -> &mut dyn AppScreen;
}
