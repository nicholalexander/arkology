mod simulation_time;
mod terminal_interface;
mod world;

use crossterm::event::{self, Event, KeyCode};
use simulation_time::SimulationTime;
use terminal_interface::TerminalInterface;
use crate::world::terrain::TerrainGrid; // If you need to access TerrainGrid directly
use world::flowers::*;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terrain = TerrainGrid::new(10, 10);
    let mut simulation_time = SimulationTime::new();
    let mut terminal_interface = TerminalInterface::new()?;
    let mut goldenrod: Goldenrod = Goldenrod::new(0);

    loop {
        update(&mut terrain, &mut simulation_time, &mut goldenrod);
        terminal_interface.render(&terrain, &simulation_time, &goldenrod)?;

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

fn update(terrain: &mut TerrainGrid, simulation_time: &mut SimulationTime, goldenrod: &mut Goldenrod) {
    simulation_time.advance();
    terrain.update_temperature(simulation_time.hour);
    goldenrod.update_nectar();
}
