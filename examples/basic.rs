use std::{
    fmt::Display,
    io::{self, stdout, Stdout},
    time::Duration,
};

use intui_tabs::{Tabs, TabsState};
use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    prelude::CrosstermBackend,
    widgets::Widget,
    Terminal,
};
use ratatui::{prelude::*, style::Color};

fn main() {
    let mut terminal = init_terminal().unwrap();
    App::new().run(&mut terminal).unwrap();
    restore_terminal().unwrap();
}

fn init_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)
}

#[derive(Copy, Clone)]
enum CurrentTab {
    Main,
    Settings,
    About,
}

impl Display for CurrentTab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CurrentTab::Main => write!(f, "Main"),
            CurrentTab::Settings => write!(f, "Settings"),
            CurrentTab::About => write!(f, "About"),
        }
    }
}

impl Default for CurrentTab {
    fn default() -> Self {
        Self::Main
    }
}

struct App {
    tabs_state: TabsState<CurrentTab>,
}

impl App {
    fn new() -> Self {
        Self {
            tabs_state: TabsState::new(vec![
                CurrentTab::Main,
                CurrentTab::Settings,
                CurrentTab::About,
            ]),
        }
    }

    fn run(mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
        loop {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;

            if event::poll(Duration::from_millis(10))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('h') => self.tabs_state.prev(),
                        KeyCode::Char('l') => self.tabs_state.next(),
                        KeyCode::Char('1') => self.tabs_state.set(1),
                        KeyCode::Char('2') => self.tabs_state.set(2),
                        KeyCode::Char('3') => self.tabs_state.set(3),
                        _ => (),
                    }
                }
            }
        }

        Ok(())
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Tabs::new()
            .color(Color::Red)
            .beginner_mode(true)
            .center(true)
            .render(area, buf, &mut self.tabs_state);
    }
}
