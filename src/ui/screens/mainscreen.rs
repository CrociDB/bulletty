use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{
    Block, BorderType, Borders, Clear, List, ListItem, ListState, Padding, Paragraph, Wrap,
};
use tracing::error;

use crate::app::AppWorkStatus;
use crate::core::ui::appscreen::{AppScreen, AppScreenEvent};
use crate::core::{
    feed::{
        feedentry::FeedEntry,
        feedlist::{FeedList, FeedListState},
    },
    library::Library,
};
use crate::ui::screens::{helpdialog::HelpDialog, readerscreen::ReaderScreen, urldialog::UrlDialog};

#[derive(PartialEq, Clone, Copy)]
enum MainInputState {
    Menu,
    Content,
}

pub struct FeedEntryState {
    selected: usize,
    list: Vec<FeedEntry>,
    state: ListState,
}

impl FeedEntryState {
    fn new() -> FeedEntryState {
        FeedEntryState {
            selected: 0,
            list: vec![],
            state: ListState::default(),
        }
    }

    fn set_list(&mut self, list: Vec<FeedEntry>) {
        self.list = list;
        self.selected = 0;
        self.state.select(Some(0));
    }

    fn select_next(&mut self) {
        if !self.list.is_empty() {
            self.selected = std::cmp::min(self.selected + 1, self.list.len() - 1);
            self.state.select(Some(self.selected));
        }
    }

    fn select_previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
            self.state.select(Some(self.selected));
        }
    }

    fn select_first(&mut self) {
        if !self.list.is_empty() {
            self.selected = 0;
            self.state.select(Some(self.selected));
        }
    }

    fn select_last(&mut self) {
        if !self.list.is_empty() {
            self.selected = self.list.len() - 1;
            self.state.select(Some(self.selected));
        }
    }

    fn get_selected(&self) -> Option<FeedEntry> {
        if self.selected < self.list.len() {
            Some(self.list[self.selected].clone())
        } else {
            None
        }
    }

    fn get_selected_index(&self) -> usize {
        self.selected
    }

    fn get_list(&self) -> &Vec<FeedEntry> {
        &self.list
    }

    fn set_current_read(&mut self) {
        if self.selected < self.list.len() {
            self.list[self.selected].seen = true;
        }
    }

    fn is_empty(&self) -> bool {
        self.list.is_empty()
    }
}

pub struct MainScreen {
    library: Library,
    feedliststate: FeedListState,
    feedentrystate: FeedEntryState,
    inputstate: MainInputState,
}

impl MainScreen {
    pub fn new(library: Library) -> MainScreen {
        let mut feedliststate = FeedListState::new();
        feedliststate.set_list(library.data.get_feeds());

        MainScreen {
            library,
            feedliststate,
            feedentrystate: FeedEntryState::new(),
            inputstate: MainInputState::Menu,
        }
    }

    fn set_all_read(&mut self) {
        if let Some(selected_feed) = self.feedliststate.get_selected() {
            self.library.data.set_all_read(&selected_feed);
            self.update_content();
        }
    }

    fn update_content(&mut self) {
        if let Some(selected_feed) = self.feedliststate.get_selected() {
            let entries = self.library.data.get_entries(&selected_feed);
            self.feedentrystate.set_list(entries);
        }
    }

    fn open_external_url(&self, url: &str) -> Result<AppScreenEvent> {
        match open::that(url) {
            Ok(_) => Ok(AppScreenEvent::None),
            Err(_) => {
                error!("Couldn't invoke system browser");
                Ok(AppScreenEvent::OpenDialog(Box::new(UrlDialog::new(
                    url.to_string(),
                ))))
            }
        }
    }
}

impl AppScreen for MainScreen {
    fn start(&mut self) {
        self.update_content();
    }

    fn render(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
        let block = Block::default()
            .style(Style::default().bg(Color::from_u32(0x262626)))
            .padding(Padding::new(3, 3, 3, 3));

        frame.render_widget(block, area);

        let layout = Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)])
            .margin(2)
            .split(area);

        // Left panel (feeds)
        let feeds = self.library.data.get_feeds();
        let feeditems: Vec<ListItem> = feeds
            .iter()
            .map(|feed| {
                let unread_count = self.library.data.get_unread_count(feed);
                let display_text = if unread_count > 0 {
                    format!("{} ({})", feed.title, unread_count)
                } else {
                    feed.title.clone()
                };
                ListItem::new(display_text)
            })
            .collect();

        let feedlist = List::new(feeditems)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(if self.inputstate == MainInputState::Menu {
                        Style::default().fg(Color::LightRed)
                    } else {
                        Style::default().fg(Color::from_u32(0x777777))
                    })
                    .title(" Feeds "),
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .bg(Color::from_u32(0x444444))
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        frame.render_stateful_widget(feedlist, layout[0], &mut self.feedliststate.state);

        // Right panel (entries)
        let entryitems: Vec<ListItem> = self
            .feedentrystate
            .list
            .iter()
            .map(|entry| {
                let prefix = if entry.seen { " " } else { "\u{2022} " };
                let display_text = format!("{}{}", prefix, entry.title);
                ListItem::new(display_text)
            })
            .collect();

        let entrylist = List::new(entryitems)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(if self.inputstate == MainInputState::Content {
                        Style::default().fg(Color::LightRed)
                    } else {
                        Style::default().fg(Color::from_u32(0x777777))
                    })
                    .title(" Entries "),
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .bg(Color::from_u32(0x444444))
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        frame.render_stateful_widget(entrylist, layout[1], &mut self.feedentrystate.state);
    }

    fn handle_event(&mut self, event: Event) -> Result<AppScreenEvent> {
        if let Event::Key(key) = event {
            if key.kind != KeyEventKind::Press {
                return Ok(AppScreenEvent::None);
            }

            match self.inputstate {
                MainInputState::Menu => match (key.modifiers, key.code) {
                    (_, KeyCode::Char('q'))
                    | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                        Ok(AppScreenEvent::ExitApp)
                    }
                    (_, KeyCode::Down | KeyCode::Char('j')) => {
                        self.feedliststate.select_next();
                        self.update_content();
                        Ok(AppScreenEvent::None)
                    }
                    (_, KeyCode::Up | KeyCode::Char('k')) => {
                        self.feedliststate.select_previous();
                        self.update_content();
                        Ok(AppScreenEvent::None)
                    }
                    (_, KeyCode::Home | KeyCode::Char('g')) => {
                        self.feedliststate.select_first();
                        self.update_content();
                        Ok(AppScreenEvent::None)
                    }
                    (_, KeyCode::End | KeyCode::Char('G')) => {
                        self.feedliststate.select_last();
                        self.update_content();
                        Ok(AppScreenEvent::None)
                    }
                    (_, KeyCode::Enter | KeyCode::Right | KeyCode::Char('l')) => {
                        self.inputstate = MainInputState::Content;
                        Ok(AppScreenEvent::None)
                    }
                    (_, KeyCode::Char('R')) => {
                        self.set_all_read();
                        Ok(AppScreenEvent::None)
                    }
                    (_, KeyCode::Char('?')) => Ok(AppScreenEvent::OpenDialog(Box::new(
                        HelpDialog::new(self.get_full_instructions()),
                    ))),
                    _ => Ok(AppScreenEvent::None),
                },
                MainInputState::Content => match (key.modifiers, key.code) {
                    (_, KeyCode::Char('q'))
                    | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                        Ok(AppScreenEvent::ExitApp)
                    }
                    (_, KeyCode::Down | KeyCode::Char('j')) => {
                        self.feedentrystate.select_next();
                        Ok(AppScreenEvent::None)
                    }
                    (_, KeyCode::Up | KeyCode::Char('k')) => {
                        self.feedentrystate.select_previous();
                        Ok(AppScreenEvent::None)
                    }
                    (_, KeyCode::Home | KeyCode::Char('g')) => {
                        self.feedentrystate.select_first();
                        Ok(AppScreenEvent::None)
                    }
                    (_, KeyCode::End | KeyCode::Char('G')) => {
                        self.feedentrystate.select_last();
                        Ok(AppScreenEvent::None)
                    }
                    (_, KeyCode::Esc) => {
                        self.inputstate = MainInputState::Menu;
                        Ok(AppScreenEvent::None)
                    }
                    (_, KeyCode::Right | KeyCode::Char('h')) => {
                        self.inputstate = MainInputState::Menu;
                        Ok(AppScreenEvent::None)
                    }
                    (_, KeyCode::Enter) => {
                        if let Some(entry) = self.feedentrystate.get_selected() {
                            self.library.data.set_entry_seen(&entry);
                            self.feedentrystate.set_current_read();

                            // Pass the full list of entries and current index to ReaderScreen
                            let entries = self.feedentrystate.get_list().clone();
                            let current_index = self.feedentrystate.get_selected_index();

                            Ok(AppScreenEvent::ChangeState(Box::new(
                                ReaderScreen::new_with_entries(entries, current_index),
                            )))
                        } else {
                            Ok(AppScreenEvent::None)
                        }
                    }
                    (_, KeyCode::Char('r')) => {
                        if let Some(entry) = self.feedentrystate.get_selected() {
                            self.library.data.toggle_entry_seen(&entry);
                        }
                        Ok(AppScreenEvent::None)
                    }
                    (_, KeyCode::Char('R')) => {
                        self.set_all_read();
                        Ok(AppScreenEvent::None)
                    }
                    (_, KeyCode::Char('o')) => {
                        if let Some(entry) = self.feedentrystate.get_selected() {
                            self.library.data.set_entry_seen(&entry);
                            self.open_external_url(&entry.url)
                        } else {
                            Ok(AppScreenEvent::None)
                        }
                    }
                    (_, KeyCode::Char('?')) => Ok(AppScreenEvent::OpenDialog(Box::new(
                        HelpDialog::new(self.get_full_instructions()),
                    ))),
                    _ => Ok(AppScreenEvent::None),
                },
            }
        } else {
            Ok(AppScreenEvent::None)
        }
    }

    fn get_title(&self) -> String {
        String::from("Main")
    }

    fn get_instructions(&self) -> String {
        if self.inputstate == MainInputState::Menu {
            String::from("?: Help | j/k/↓/↑: move | Enter: select | Esc: quit")
        } else {
            String::from("?: Help | j/k/↓/↑: move | o: open | Enter: read | Esc: back")
        }
    }

    fn get_work_status(&self) -> AppWorkStatus {
        self.library.get_update_status()
    }

    fn get_full_instructions(&self) -> String {
        String::from(
            r#"Main Screen Instructions:

Navigation:
j/k or ↓/↑   - Move up/down in current panel
h/l or ←/→   - Switch between panels
g/G          - Go to first/last item
Enter        - Select item or enter reader mode
Esc          - Go back or quit

Feed Management:
r            - Toggle read status of selected entry
R            - Mark all entries in feed as read
o            - Open selected entry URL in browser

Reader Mode:
When you press Enter on an entry, you'll enter reader mode
where you can navigate between entries using 'n' and 'p'
without leaving the reader.

?            - Show this help
q/Ctrl+C     - Quit application
"#,
        )
    }
}
