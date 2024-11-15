use std::marker::PhantomData;

use ratatui::{
    prelude::*,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Tabs as RatatuiTabs, Widget},
};

pub struct Tabs<T: ToString + Default> {
    tabs_list: Vec<Line<'static>>,
    color: Color,
    _pht: PhantomData<T>,
}

impl<T: ToString + Default> Tabs<T> {
    pub fn new(tabs: Vec<T>, color: Color, beginner_mode: bool) -> Self {
        let tabs_list = {
            let mut lines = vec![];
            if beginner_mode {
                for (idx, tab) in tabs.iter().enumerate() {
                    let mut line = Line::default();
                    line.push_span(Span::styled(
                        (idx + 1).to_string(),
                        Style::default().underlined().underline_color(color),
                    ));

                    line.push_span(Span::raw(format!(". {}", tab.to_string())));
                    lines.push(line);
                }
            } else {
                for tab in &tabs {
                    lines.push(Line::raw(tab.to_string()));
                }
            }
            lines
        };

        Self {
            tabs_list,
            color,
            _pht: PhantomData,
        }
    }
}

#[derive(Default)]
pub struct TabsState {
    current_tab: usize,
}

impl TabsState {
    pub fn next(&mut self) {
        self.current_tab = self.current_tab.saturating_add(1);
    }

    pub fn prev(&mut self) {
        self.current_tab = self.current_tab.saturating_sub(1);
    }

    pub fn set(&mut self, idx: usize) {
        self.current_tab = idx.saturating_sub(1);
    }
}

impl<T: ToString + Default> StatefulWidget for Tabs<T> {
    type State = TabsState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut TabsState)
    where
        Self: Sized,
    {
        let tabs_higlight_style = Style::default().fg(self.color).not_underlined();
        let tabs = RatatuiTabs::new(self.tabs_list)
            .style(Style::default().white())
            .highlight_style(tabs_higlight_style)
            .select(state.current_tab)
            .divider(symbols::DOT);
        tabs.render(area, buf);
    }
}
