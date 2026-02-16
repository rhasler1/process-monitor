pub mod process;

pub trait DrawableComponent {
    fn draw(
        &mut self,
        f: &mut ratatui::prelude::Frame, 
        area: ratatui::prelude::Rect, 
        focused: bool) -> anyhow::Result<()>;
}

pub trait Component {
    fn key_event(
        &mut self, 
        key: crate::adapters::crossterm::input::KeyInput,
    ) -> anyhow::Result<crate::app::EventState>;

    fn mouse_event(
        &mut self, 
        mouse: crate::adapters::crossterm::input::MouseInput,
    ) -> anyhow::Result<crate::app::EventState>;
}
