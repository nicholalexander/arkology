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
use crate::world::flowers::*;
use crate::world::terrain::{TerrainGrid, TerrainTile};

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
        flowers: &Vec<Box<dyn Flower>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.terminal.draw(|f| {
            let size = f.size();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(3),      // Space for the time display
                    Constraint::Percentage(50), // Half the remaining space for the terrain grid
                    Constraint::Percentage(50), // Half for the flower table
                ])
                .split(size);

            // Render time
            let time_text = format!(
                "Day: {}, Hour: {}",
                simulation_time.day, simulation_time.hour
            );
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

            // Assuming you're inside a rendering function or method
            let flower_rows = flowers.iter().map(|flower| {
                let nectar = flower.nectar_count();
                let name = flower.name();
                let (x, y) = flower.get_position();
                // For simplicity, assuming a direct conversion to String, adjust formatting as needed
                let flower_text = format!("{}: {} | x: {} y: {}", name, nectar, x, y); // Include the flower's name
                Row::new(vec![flower_text])
            });

            let flower_table = Table::new(flower_rows)
                .block(
                    Block::default()
                        .title("Flower Nectar Counts")
                        .borders(Borders::ALL),
                )
                // Adjust the width or other constraints as needed for your layout
                .widths(&[Constraint::Percentage(100)]);

            // Determine where to place this in your layout, adjusting as needed
            f.render_widget(flower_table, chunks[2]); // This should now correctly refer to the flower table's layout section
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
