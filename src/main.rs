mod simulation_time;
mod terminal_interface;
mod terrain;

use crossterm::event::{self, Event, KeyCode};
use simulation_time::SimulationTime;
use terminal_interface::TerminalInterface;
use terrain::TerrainGrid;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terrain = TerrainGrid::new(10, 10);
    let mut simulation_time = SimulationTime::new();
    let mut terminal_interface = TerminalInterface::new()?;

    loop {
        update(&mut terrain, &mut simulation_time);
        terminal_interface.render(&terrain, &simulation_time)?;

        if event::poll(std::time::Duration::from_millis(1))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.code == KeyCode::Char('c')
                    && key_event
                        .modifiers
                        .contains(crossterm::event::KeyModifiers::CONTROL)
                {
                    break;
                }
            }
        }
    }

    Ok(())
}

fn update(terrain: &mut TerrainGrid, simulation_time: &mut SimulationTime) {
    simulation_time.advance();
    terrain.update_temperature(simulation_time.hour);
}
