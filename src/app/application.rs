// Ratatui
use ratatui::prelude::{Frame,Layout,Direction,Constraint,Alignment,Style,Span,Color};
use ratatui::widgets::Paragraph;
// Internal application adapters
use crate::adapters::sysinfo::sysinfo_datasource::SystemDataSource;
// Internal application components
use crate::app::components::process::ProcessComponent;
use crate::app::components::DrawableComponent;

pub struct App {
    message: String,
    data_source: SystemDataSource,
    process_component: ProcessComponent
}

impl App {
    pub fn default() -> Self {
        let message: String = String::from("Hello, press 'q' or left click to exit.");
        let mut data_source: SystemDataSource = SystemDataSource::default();
        data_source.refresh_all();
        let process_component: ProcessComponent = ProcessComponent::new(&data_source);
        Self {
            message,
            data_source,
            process_component
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) -> anyhow::Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),
                Constraint::Percentage(80)
            ])
            .split(frame.size());
        
        let span = Span::styled(self.message.clone(), Style::new().fg(Color::Green));
        let paragraph = Paragraph::new(span).alignment(Alignment::Center);
        frame.render_widget(paragraph, chunks[0]);
        self.process_component.draw(frame, chunks[1], true);

        Ok(())
    }
}
