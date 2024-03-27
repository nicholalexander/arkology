// Temporary Terminal Interface for Quick Debugging
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::collections::HashMap;
use std::io::{self, Stdout};

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Gauge, Paragraph, Row, Table},
    Terminal,
};

use crate::simulation_time::SimulationTime;
use crate::world::bees::*;
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
        bees: &Vec<Bee>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.terminal.draw(|f| {
            let size = f.size();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(3),      // Space for the time display
                    Constraint::Percentage(50), // Adjusted to leave space for the gauge
                    Constraint::Percentage(30), // Adjusted for the flower table
                    Constraint::Length(3),      // New space for the total nectar gauge
                ])
                .split(size);

            // Render time
            let time_text = format!(
                "Day: {}, Hour: {}, Month: {}, Year: {}",
                simulation_time.day,
                simulation_time.hour,
                simulation_time.month,
                simulation_time.year
            );
            let time_paragraph = Paragraph::new(time_text)
                .block(Block::default().title("Time").borders(Borders::ALL));
            f.render_widget(time_paragraph, chunks[0]);

            // Render terrain grid
            // Create a HashMap from flower positions to references to the flowers
            let flower_positions: HashMap<(usize, usize), &Box<dyn Flower>> = flowers
                .iter()
                .map(|flower| ((flower.get_position(), flower)))
                .collect();

            let bee_positions: HashMap<(usize, usize), &Bee> =
                bees.iter().map(|bee| ((bee.get_position(), bee))).collect();

            // Adjusted rendering code with manual tracking of x and y coordinates
            let rows = terrain.tiles.iter().enumerate().map(|(y, row)| {
                let cells = row.iter().enumerate().map(|(x, tile)| {
                    // Use the flower_positions HashMap to check if there's a flower at this tile's position
                    let tile_and_flower_content = if let Some(flower) =
                        flower_positions.get(&(x, y))
                    {
                        // Get the nectar count from the flower
                        let nectar = flower.nectar_count();
                        let emoji = flower.flower_emoji();

                        // Convert nectar count to a percentage of the gauge
                        let gauge_percentage = nectar as f32 / 100.0;

                        // Create a simple textual gauge. Adjust the size as necessary.
                        let gauge_size = 10; // Total size of the gauge
                        let filled_length = (gauge_percentage * gauge_size as f32).round() as usize;
                        let empty_length = gauge_size - filled_length;
                        let gauge_visual = format!(
                            "[{}{}]",
                            "#".repeat(filled_length), // Filled part of the gauge
                            "-".repeat(empty_length)   // Empty part of the gauge
                        );

                        // Append the gauge to the temperature text
                        format!("{:.1}{}\n{}", tile.temperature, emoji, gauge_visual)
                    } else {
                        // If no flower, just display the temperature
                        format!("{:.1}", tile.temperature)
                    };

                    let bee_content = if let Some(bee) = bee_positions.get(&(x, y)) {
                        format!("{}", bee.emoji())
                    } else {
                        format!(" ")
                    };

                    let combined_content = format!("{} {}", tile_and_flower_content, bee_content);

                    Cell::from(combined_content)
                });
                // Here we set the height of each row to 2
                Row::new(cells).height(2).bottom_margin(0)
            });

            let table = Table::new(rows)
                .block(Block::default().title("Terrain Grid").borders(Borders::ALL))
                .widths(&[Constraint::Percentage(10); 10]);

            // Assuming `f.render_widget` is part of your UI rendering logic
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

            // Calculate the total nectar from all flowers
            let total_nectar: u32 = flowers.iter().map(|flower| flower.nectar_count()).sum();
            let max_nectar_possible: u32 = flowers.len() as u32 * 100; // Assuming max 100 nectar per flower, adjust as needed

            // Calculate the percentage of the total nectar to the maximum possible nectar
            let total_nectar_percentage = if max_nectar_possible > 0 {
                (total_nectar as f64 / max_nectar_possible as f64 * 100.0).round() as u16
            } else {
                0 // To avoid division by zero
            };

            // Create a gauge for total nectar
            let total_nectar_gauge = Gauge::default()
                .block(Block::default().title("Total Nectar").borders(Borders::ALL))
                .gauge_style(Style::default().fg(Color::Yellow)) // Customize as needed
                .percent(total_nectar_percentage);

            // Render the gauge in the designated area
            f.render_widget(total_nectar_gauge, chunks[3]); // Assuming `chunks[2]` is the intended area
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
