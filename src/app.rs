use std::time::Duration;

use crossterm::event::{Event, KeyCode, KeyEventKind};

use crate::capture::{InterfaceProvider, PacketSource};
use crate::error::AppError;
use crate::tui::Tui;

pub enum AppMode {
    SelectInterface,
    Capturing,
}

pub struct App<S: PacketSource, I: InterfaceProvider> {
    pub mode: AppMode,
    pub interfaces: Vec<String>,
    pub selected_index: usize,
    pub should_quit: bool,
    pub active_interface: Option<String>,
    source: S,
    _provider: std::marker::PhantomData<I>,
}

impl<S: PacketSource, I: InterfaceProvider> App<S, I> {
    pub fn new(source: S, provider: &I, interface_flag: Option<String>) -> Result<Self, AppError> {
        let interfaces = provider.list_interfaces()?;

        if interfaces.is_empty() {
            return Err(AppError::NoInterfaces);
        }

        if let Some(ref name) = interface_flag {
            if !interfaces.contains(name) {
                return Err(AppError::InterfaceNotFound(name.clone()));
            }
            return Ok(Self {
                mode: AppMode::Capturing,
                interfaces,
                selected_index: 0,
                should_quit: false,
                active_interface: Some(name.clone()),
                source,
                _provider: std::marker::PhantomData,
            });
        }

        Ok(Self {
            mode: AppMode::SelectInterface,
            interfaces,
            selected_index: 0,
            should_quit: false,
            active_interface: None,
            source,
            _provider: std::marker::PhantomData,
        })
    }

    /// Process one iteration of app state: drain the packet source and handle
    /// any pending input events. Separated from `run()` so the state logic can
    /// be exercised in unit tests without a real terminal.
    pub fn tick(&mut self, events: &[Event]) {
        while self.source.next_packet().is_some() {}
        for event in events {
            self.handle_event(event.clone());
        }
    }

    pub fn run(&mut self, tui: &mut Tui) -> Result<(), AppError> {
        loop {
            let mut pending = Vec::new();
            if crossterm::event::poll(Duration::from_millis(16))
                .map_err(crate::error::InterfaceError::from)?
            {
                if let Ok(event) = crossterm::event::read() {
                    pending.push(event);
                }
            }

            self.tick(&pending);

            tui.draw(|frame| crate::tui::ui::render(frame, self))
                .map_err(crate::error::InterfaceError::from)?;

            if self.should_quit {
                break;
            }
        }
        Ok(())
    }

    pub fn handle_event(&mut self, event: Event) {
        let Event::Key(key) = event else { return };
        // Only handle key press events, not release/repeat
        if key.kind != KeyEventKind::Press {
            return;
        }

        match self.mode {
            AppMode::SelectInterface => match key.code {
                KeyCode::Up => {
                    self.selected_index = self.selected_index.saturating_sub(1);
                }
                KeyCode::Down => {
                    let max = self.interfaces.len().saturating_sub(1);
                    self.selected_index = (self.selected_index + 1).min(max);
                }
                KeyCode::Enter => {
                    if let Some(name) = self.interfaces.get(self.selected_index) {
                        self.active_interface = Some(name.clone());
                        self.mode = AppMode::Capturing;
                    }
                }
                KeyCode::Char('q') => {
                    self.should_quit = true;
                }
                _ => {}
            },
            AppMode::Capturing => {
                if let KeyCode::Char('q') = key.code {
                    self.should_quit = true;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capture::interface::test_helpers::MockInterfaceProvider;
    use crate::capture::packet_source::test_helpers::MockPacketSource;
    use crate::error::AppError;
    use crossterm::event::{KeyEvent, KeyModifiers};

    fn make_app_select(ifaces: Vec<String>) -> App<MockPacketSource, MockInterfaceProvider> {
        let provider = MockInterfaceProvider::new(ifaces);
        let source = MockPacketSource::empty();
        App::new(source, &provider, None).expect("make_app_select failed")
    }

    fn make_app_capturing(iface: &str) -> App<MockPacketSource, MockInterfaceProvider> {
        let provider = MockInterfaceProvider::new(vec![iface.to_string()]);
        let source = MockPacketSource::empty();
        App::new(source, &provider, Some(iface.to_string())).expect("make_app_capturing failed")
    }

    fn key(code: KeyCode) -> Event {
        Event::Key(KeyEvent::new(code, KeyModifiers::NONE))
    }

    #[test]
    fn q_in_select_interface_sets_should_quit() {
        let mut app = make_app_select(vec!["eth0".to_string()]);
        app.handle_event(key(KeyCode::Char('q')));
        assert!(app.should_quit);
    }

    #[test]
    fn q_in_capturing_sets_should_quit() {
        let mut app = make_app_capturing("eth0");
        app.handle_event(key(KeyCode::Char('q')));
        assert!(app.should_quit);
    }

    #[test]
    fn enter_transitions_to_capturing() {
        let mut app = make_app_select(vec!["eth0".to_string(), "lo".to_string()]);
        app.handle_event(key(KeyCode::Enter));
        assert!(matches!(app.mode, AppMode::Capturing));
        assert_eq!(app.active_interface, Some("eth0".to_string()));
    }

    #[test]
    fn down_arrow_increments_selected_index() {
        let mut app = make_app_select(vec!["eth0".to_string(), "lo".to_string()]);
        assert_eq!(app.selected_index, 0);
        app.handle_event(key(KeyCode::Down));
        assert_eq!(app.selected_index, 1);
    }

    #[test]
    fn up_arrow_at_zero_does_not_underflow() {
        let mut app = make_app_select(vec!["eth0".to_string()]);
        app.handle_event(key(KeyCode::Up));
        assert_eq!(app.selected_index, 0);
    }

    #[test]
    fn down_arrow_at_end_does_not_overflow() {
        let mut app = make_app_select(vec!["eth0".to_string(), "lo".to_string()]);
        app.handle_event(key(KeyCode::Down));
        app.handle_event(key(KeyCode::Down));
        assert_eq!(app.selected_index, 1);
    }

    #[test]
    fn interface_flag_starts_in_capturing_mode() {
        let app = make_app_capturing("lo");
        assert!(matches!(app.mode, AppMode::Capturing));
        assert_eq!(app.active_interface, Some("lo".to_string()));
    }

    #[test]
    fn no_interface_flag_starts_in_select_interface() {
        let app = make_app_select(vec!["eth0".to_string()]);
        assert!(matches!(app.mode, AppMode::SelectInterface));
        assert!(app.active_interface.is_none());
    }

    #[test]
    fn up_arrow_decrements_selected_index() {
        let mut app = make_app_select(vec!["eth0".to_string(), "lo".to_string()]);
        app.handle_event(key(KeyCode::Down)); // move to index 1
        app.handle_event(key(KeyCode::Up)); // move back to index 0
        assert_eq!(app.selected_index, 0);
    }

    #[test]
    fn non_quit_keys_in_capturing_are_inert() {
        let mut app = make_app_capturing("eth0");
        app.handle_event(key(KeyCode::Down));
        app.handle_event(key(KeyCode::Enter));
        assert!(!app.should_quit);
        assert!(matches!(app.mode, AppMode::Capturing));
    }

    #[test]
    fn tick_processes_multiple_events_in_order() {
        // Down moves to index 1, then Enter transitions to Capturing with "lo"
        let mut app = make_app_select(vec!["eth0".to_string(), "lo".to_string()]);
        app.tick(&[key(KeyCode::Down), key(KeyCode::Enter)]);
        assert!(matches!(app.mode, AppMode::Capturing));
        assert_eq!(app.active_interface, Some("lo".to_string()));
    }

    // Satisfies: R-01-09 — unknown interface name returns InterfaceNotFound
    #[test]
    fn unknown_interface_returns_not_found() {
        let provider = MockInterfaceProvider::new(vec!["eth0".to_string()]);
        let source = MockPacketSource::empty();
        let result = App::new(source, &provider, Some("fake0".to_string()));
        assert!(matches!(result, Err(AppError::InterfaceNotFound(ref n)) if n == "fake0"));
    }

    #[test]
    fn provider_failure_returns_interface_error() {
        let provider = MockInterfaceProvider::failing();
        let source = MockPacketSource::empty();
        let result = App::new(source, &provider, None);
        assert!(matches!(result, Err(AppError::Interface(_))));
    }

    // Satisfies: R-01-11 — no interfaces available returns NoInterfaces error
    #[test]
    fn empty_interface_list_returns_no_interfaces() {
        let provider = MockInterfaceProvider::new(vec![]);
        let source = MockPacketSource::empty();
        let result = App::new(source, &provider, None);
        assert!(matches!(result, Err(AppError::NoInterfaces)));
    }

    #[test]
    fn snapshot_select_interface() {
        use ratatui::backend::TestBackend;
        use ratatui::Terminal;

        let app = make_app_select(vec![
            "eth0".to_string(),
            "lo".to_string(),
            "wlan0".to_string(),
        ]);
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| crate::tui::ui::render(frame, &app))
            .unwrap();
        insta::assert_debug_snapshot!(terminal.backend().buffer().clone());
    }

    #[test]
    fn snapshot_capturing_status_bar() {
        use ratatui::backend::TestBackend;
        use ratatui::Terminal;

        let app = make_app_capturing("eth0");
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| crate::tui::ui::render(frame, &app))
            .unwrap();
        insta::assert_debug_snapshot!(terminal.backend().buffer().clone());
    }
}
