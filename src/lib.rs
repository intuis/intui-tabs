use std::marker::PhantomData;

use ratatui::{
    prelude::*,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Tabs as RatatuiTabs, Widget},
};

pub struct Tabs<T: ToString + Default> {
    beginner_mode: bool,
    color: Color,
    _pht: PhantomData<T>,
}

impl<T: ToString + Default> Tabs<T> {
    pub fn new() -> Self {
        Self {
            beginner_mode: true,
            color: Color::default(),
            _pht: PhantomData,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn beginner_mode(mut self, beginner_mode: bool) -> Self {
        self.beginner_mode = beginner_mode;
        self
    }
}

pub struct TabsState<T: ToString + Default + Copy> {
    tabs_list: Vec<T>,
    current_tab: usize,
}

impl<T: ToString + Default + Copy> TabsState<T> {
    pub fn new(possible_tabs: Vec<T>) -> Self {
        Self {
            tabs_list: possible_tabs,
            current_tab: 0,
        }
    }
}

impl<T: ToString + Default + Copy> TabsState<T> {
    pub fn next(&mut self) {
        if self.current_tab == self.tabs_list.len() - 1 {
            return;
        }

        self.current_tab = self.current_tab.saturating_add(1);
    }

    pub fn prev(&mut self) {
        self.current_tab = self.current_tab.saturating_sub(1);
    }

    pub fn set(&mut self, idx: usize) {
        self.current_tab = idx.saturating_sub(1);
    }

    pub fn current(&self) -> T {
        *self
            .tabs_list
            .get(self.current_tab)
            .unwrap_or_else(|| self.tabs_list.last().unwrap())
    }
}

impl<T: ToString + Default + Copy> StatefulWidget for Tabs<T> {
    type State = TabsState<T>;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut TabsState<T>)
    where
        Self: Sized,
    {
        let tabs_list_lines = {
            let mut lines = vec![];
            if self.beginner_mode {
                for (idx, tab) in state.tabs_list.iter().enumerate() {
                    let mut line = Line::default();
                    line.push_span(Span::styled(
                        (idx + 1).to_string(),
                        Style::default().underlined().underline_color(self.color),
                    ));

                    line.push_span(Span::raw(format!(". {}", tab.to_string())));
                    lines.push(line);
                }
            } else {
                for tab in &state.tabs_list {
                    lines.push(Line::raw(tab.to_string()));
                }
            }
            lines
        };

        let tabs_higlight_style = Style::default().fg(self.color).not_underlined();
        let tabs = RatatuiTabs::new(tabs_list_lines)
            .style(Style::default().white())
            .highlight_style(tabs_higlight_style)
            .select(state.current_tab)
            .divider(symbols::DOT);
        tabs.render(area, buf);
    }
}
