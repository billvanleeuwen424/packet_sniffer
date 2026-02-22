use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Text;
use ratatui::widgets::{Block, List, ListItem, ListState, Paragraph};
use ratatui::Frame;

use crate::app::{App, AppMode};
use crate::capture::{InterfaceProvider, PacketSource};

pub fn render<S: PacketSource, I: InterfaceProvider>(frame: &mut Frame, app: &App<S, I>) {
    match app.mode {
        AppMode::SelectInterface => render_select_interface(frame, app),
        AppMode::Capturing => render_capturing(frame, app),
    }
}

fn render_select_interface<S: PacketSource, I: InterfaceProvider>(
    frame: &mut Frame,
    app: &App<S, I>,
) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(area);

    // Build numbered list items
    let items: Vec<ListItem> = app
        .interfaces
        .iter()
        .enumerate()
        .map(|(i, name)| ListItem::new(Text::raw(format!("{}: {}", i + 1, name))))
        .collect();

    let list = List::new(items)
        .block(Block::bordered().title("Select Interface"))
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    let mut state = ListState::default();
    state.select(Some(app.selected_index));

    frame.render_stateful_widget(list, chunks[0], &mut state);

    let status = Paragraph::new("\u{2191}\u{2193} to select, Enter to confirm, q to quit")
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(status, chunks[1]);
}

fn render_capturing<S: PacketSource, I: InterfaceProvider>(frame: &mut Frame, app: &App<S, I>) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(area);

    let list = List::new(Vec::<ListItem>::new()).block(Block::bordered().title("Packets"));
    frame.render_widget(list, chunks[0]);

    let iface_name = app.active_interface.as_deref().unwrap_or("unknown");
    let status_text = format!("interface: {}   \u{25cf} capturing", iface_name);
    let status = Paragraph::new(status_text).style(Style::default().fg(Color::Green));
    frame.render_widget(status, chunks[1]);
}
