// Log
use log::{debug, info, warn};
// Ratatui for rendering and widget creation
use ratatui::prelude::{Frame, Layout, Direction, Constraint};
use ratatui::widgets::StatefulWidget;
// Internal application
use crate::adapters::crossterm::input::Key;
use crate::config::AppConfig;
use crate::domain::process::model::{ProcessSnapShot};
use crate::components::process_table::{ProcessTableComponent, ProcessTableEventState};
use crate::components::process_term::component::Component as ProcTermComponent;
use crate::widgets::ProcessTableWidget;
use crate::components::config_manager::ConfigManagerComponent;

use crate::components::Event;

use anyhow::Result;

pub enum AppEventState {
    Consumed,
    NotConsumed,
    TerminatePid(u32)
}

#[derive(Default)]
pub enum Focus {
    #[default]
    Table,
    Config,
}

pub struct App {
    process_snapshot:   ProcessSnapShot,
    process_table:      ProcessTableComponent,
    // TODO:
    // 1.   Update Refresh Rate: Config Manager
    //      should return new refresh rate to
    //      main to modify app_events interval
    //      Additionally, ProcessTableInterval
    //      needs to be updated.
    //
    // 2.   Save: Config Manager should return new 
    //      RefreshRateConfig & ViewsConfig to
    //      main for config_worker to write.
    config_manager:     ConfigManagerComponent,
    focus:              Focus
}

impl App {
    // If fails to build with argued config, log & build with default
    pub fn new_with_config(config: AppConfig) -> Result<Self> {
        let process_snapshot = ProcessSnapShot::default();

        let (process_table, app_config) =
        match ProcessTableComponent::new_with_config(
            &process_snapshot,
            &config
        ) {
            Ok(process_table_component) => {
                (process_table_component, config.clone())        
            }
            Err(e) => {
                warn!("{e}");
                
                let config = AppConfig::default();

                (ProcessTableComponent::new_with_config(
                    &process_snapshot,
                    &config
                )?,
                config
                )
            }
        };

        let config_manager = ConfigManagerComponent::new(app_config);

        let focus            = Focus::default();

        Ok(Self {
            process_snapshot,
            process_table,
            config_manager,
            focus
        })
    }
    
    pub fn model_update(&mut self, process_snapshot: ProcessSnapShot) -> Result<()> {
        self.process_snapshot = process_snapshot;
        
        self.process_table.new_snapshot(&self.process_snapshot)?;

        Ok(())
    }

    pub fn key_event(&mut self, key: Key) -> Result<AppEventState> {
        match self.focus {
            Focus::Table => {
                match self.process_table.event(key)? {
                    ProcessTableEventState::TerminatePid(pid) => {
                        // Send pid to terminate component & swap focus
                    }
                    ProcessTableEventState::NotConsumed => {
                        // TODO:    Check if focus should be switched
                        //          to configmanager or help
                        //
                        //if matches!(key, Key::Ctrls) {
                        //    self.focus = Focus::Config;
                        //}
                    }
                    ProcessTableEventState::Consumed => {
                        return Ok(AppEventState::Consumed);
                    }
                }
            }
            Focus::Config => {
            }
        }

        Ok(AppEventState::NotConsumed)
    }

    pub fn draw(&mut self, frame: &mut Frame) -> anyhow::Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
            ])
            .split(frame.size());

        // Get table and views
        let (table, views) = self.process_table.table_and_views();

        let widget = ProcessTableWidget::new(table);
        frame.render_stateful_widget(widget, chunks[0], views);

        Ok(())
    }
}
