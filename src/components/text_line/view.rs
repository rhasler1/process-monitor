use crate::adapters::ratatui::theme::{StyleToken, RatatuiTheme};
use crate::components::text_line::model::TextLineModel;
use crate::components::utils::scroll::Scroll;

pub enum LineAlignment {
    Left,
    Right,
    Center
}

pub struct LineStyle {
    pub line:      StyleToken,
    pub focus:     StyleToken,
    pub not_focus: StyleToken
} impl Default for LineStyle {
    fn default() -> Self {
        Self {
            line:      StyleToken::Line,
            focus:     StyleToken::Focus,
            not_focus: StyleToken::NotFocus
        }
    }
}

pub struct TextLineView {
    pub len:        u16,
    pub height:     u16,
    pub alignment:  LineAlignment,
    pub style:      LineStyle,
    pub theme:      RatatuiTheme,
    pub scroll:     Scroll
} impl Default for TextLineView {
    fn default() -> Self {
        Self {
            len:        0,
            height:     0,
            alignment:  LineAlignment::Left,
            style:      LineStyle::default(),
            theme:      RatatuiTheme::default(),
            scroll:     Scroll::default()
        }
    }
}

use ratatui::prelude::{Frame, Rect, Line, Alignment};
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
        // TODO [2/24/26] Scroll should probably be moved into the model
        // scroll is dependant on size of terminal. Currently, the size
        // of the terminal is only communicated via the `draw` stage of
        // the event loop. If I want to move Scroll into state I need to
        // capture 'Resize' events?
        let start = self.scroll.calc_start(self.len.into(), model.cursor().into());
        let view =  &model.buffer()[start..model.len().into()];

        // TODO [2/24/26] Map can be made in adapters/ratatui
        let line = match self.alignment {
            LineAlignment::Left   => Line::from(view).style(self.theme.style(self.style.line)).alignment(Alignment::Left),
            LineAlignment::Right  => Line::from(view).style(self.theme.style(self.style.line)).alignment(Alignment::Right),
            LineAlignment::Center => Line::from(view).style(self.theme.style(self.style.line)).alignment(Alignment::Center),
        };
        frame.render_widget(line, area);
    
        Ok(())
    }
}

