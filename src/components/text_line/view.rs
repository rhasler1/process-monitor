use crate::components::text_line::model::TextLineModel;
use crate::components::utils::scroll::Scroll;
use ratatui::prelude::{Frame, Rect, Line, Alignment};
use ratatui::style::{Style, Color};

pub enum LineAlignment {
    Left,
    Right,
    Center
}

pub struct TextLineView {
    pub len:        u16,
    pub height:     u16,
    pub alignment:  LineAlignment,
    pub scroll:     Scroll
}

impl Default for TextLineView {
    fn default() -> Self {
        Self {
            len:        0,
            height:     0,
            alignment:  LineAlignment::Left,
            scroll:     Scroll::default()
        }
    }
}

impl TextLineView {
    pub fn handle_draw(&mut self,
        frame: &mut Frame,
        area:  Rect,
        _focus: bool,
        model: &TextLineModel) -> anyhow::Result<()>
    {
        if self.len != area.width {
            self.len = area.width;
        }
        if self.height != area.height {
            self.height = area.height;
        }

        let start = self.scroll.calc_start(self.len.into(), model.cursor());
        let view =  &model.buffer()[start..model.len()];
        
        let line = match self.alignment {
            LineAlignment::Left   => Line::from(view).style(Style::default().fg(Color::Blue)).alignment(Alignment::Left),
            LineAlignment::Right  => Line::from(view).style(Style::default().fg(Color::Blue)).alignment(Alignment::Right),
            LineAlignment::Center => Line::from(view).style(Style::default().fg(Color::Blue)).alignment(Alignment::Center),
        };

        frame.render_widget(line, area);
    
        Ok(())
    }
}

