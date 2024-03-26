mod simulation_time;
mod terminal_interface;
mod world;

use crate::world::terrain::TerrainGrid; // If you need to access TerrainGrid directly
use crossterm::event::{self, Event, KeyCode};
use simulation_time::SimulationTime;
use terminal_interface::TerminalInterface;
use world::flowers::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terrain = TerrainGrid::new(10, 10);
    let mut simulation_time = SimulationTime::new();
    let mut terminal_interface = TerminalInterface::new()?;

    let mut flowers: Vec<Box<dyn Flower>> = Vec::new();

    // Add 5 Goldenrods
    for _ in 0..5 {
        flowers.push(Box::new(Goldenrod::new(0, 0, 0)));
    }

    // Add 5 Chrysanthemums
    for _ in 0..5 {
        flowers.push(Box::new(Chrysanthemum::new(0, 0, 1)));
    }

    loop {
        update(&mut terrain, &mut simulation_time, &mut flowers);
        terminal_interface.render(&terrain, &simulation_time, &flowers)?;

        if event::poll(std::time::Duration::from_millis(1000))? {
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

fn update(
    terrain: &mut TerrainGrid,
    simulation_time: &mut SimulationTime,
    flowers: &mut Vec<Box<dyn Flower>>,
) {
    simulation_time.advance();
    terrain.update_temperature(simulation_time.hour);

    for flower in flowers.iter_mut() {
        flower.update_nectar();
    }
}
