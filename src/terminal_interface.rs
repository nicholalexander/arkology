// src/terminal_interface.rs

use crossterm::{
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Stdout};
use tui::{
  backend::CrosstermBackend,
  widgets::{Block, Borders, Paragraph},
  Terminal,
};

use crate::simulation_time::SimulationTime;
use crate::terrain::TerrainGrid;

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




  pub fn render(&mut self, terrain: &TerrainGrid, simulation_time: &SimulationTime) -> Result<(), Box<dyn std::error::Error>> {
      self.terminal.draw(|f| {
          let size = f.size();
          let block = Block::default().title("Simulation").borders(Borders::ALL);
          f.render_widget(block, size);

          // renter time
          let text = format!("Hour: {}", simulation_time.hour);
          let paragraph = Paragraph::new(text);
          f.render_widget(paragraph, size);

          // render average temperature
          let average_temperature: f32 = terrain.tiles.iter().flatten().map(|tile| tile.temperature).sum::<f32>() / (terrain.width * terrain.height) as f32;
          let text = format!("Average temperature: {:.2}°C", average_temperature);
          let paragraph = Paragraph::new(text);
          f.render_widget(paragraph, size);



      })?;
      Ok(())
  }
}

impl Drop for TerminalInterface {
  fn drop(&mut self) {
      disable_raw_mode().unwrap();
      execute!(
          self.terminal.backend_mut(),
          LeaveAlternateScreen
      )
      .unwrap();
  }
}
