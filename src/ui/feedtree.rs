use ratatui::widgets::{Block, Borders, List, ListItem, Widget};

pub struct FeedTree {}

impl Widget for FeedTree {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let items = vec![
            ListItem::new("> Category 1"),
            ListItem::new("\t > Feed 1"),
            ListItem::new("\t > Feed 2"),
            ListItem::new("> Category 2"),
            ListItem::new("> Category 3"),
        ];

        let list = List::new(items)
            .block(Block::default().title("Feeds").borders(Borders::ALL));
        list.render(area, buf);
    }
}
