// Temporary Terminal Interface for Quick Debugging
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Stdout};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, Row, Table},
    Terminal,
};

use crate::simulation_time::SimulationTime;
use crate::terrain::{TerrainGrid, TerrainTile};

pub struct TerminalInterface {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl TerminalInterface {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal })
    }

    pub fn render(
        &mut self,
        terrain: &TerrainGrid,
        simulation_time: &SimulationTime,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(3), // Space for the time display
                    Constraint::Min(0),    // Remaining space for the terrain grid
                ])
                .split(size);

            // Render time
            let time_text = format!("Hour: {}", simulation_time.hour);
            let time_paragraph = Paragraph::new(time_text)
                .block(Block::default().title("Time").borders(Borders::ALL));
            f.render_widget(time_paragraph, chunks[0]);

            // Render terrain grid
            let rows = terrain.tiles.iter().map(|row: &Vec<TerrainTile>| {
                let row_text: Vec<String> = row
                    .iter()
                    .map(|tile| format!("{:.1}", tile.temperature))
                    .collect();
                Row::new(row_text)
            });

            let table = Table::new(rows)
                .block(Block::default().title("Terrain Grid").borders(Borders::ALL))
                .widths(&[Constraint::Percentage(10); 10]); // Assuming a fixed-size grid for simplicity

            f.render_widget(table, chunks[1]);
        })?;
        Ok(())
    }
}

impl Drop for TerminalInterface {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen).unwrap();
    }
}
