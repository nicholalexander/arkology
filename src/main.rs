mod simulation_time;
mod terminal_interface;
mod world;

use crate::world::terrain::TerrainGrid; // If you need to access TerrainGrid directly
use crossterm::event::{self, Event, KeyCode};
use simulation_time::SimulationTime;
use terminal_interface::TerminalInterface;
use world::bees::*;
use world::flowers::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terrain = TerrainGrid::new(10, 10);
    let mut simulation_time = SimulationTime::new();
    let mut terminal_interface = TerminalInterface::new()?;
    let mut flowers = Flowers::build();
    let mut bees = Bees::build();

    loop {
        update(&mut terrain, &mut simulation_time, &mut flowers, &mut bees);
        terminal_interface.render(&terrain, &simulation_time, &flowers, &mut bees)?;

        if event::poll(std::time::Duration::from_millis(10))? {
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
    bees: &mut Vec<Bee>,
) {
    simulation_time.advance();
    terrain.update_temperature(simulation_time);
    update_flowers_nectar(flowers, terrain);

    world::bees::Bees::move_all_bees(bees, flowers);
    world::bees::Bees::sweep_dead_bees(bees);
}

fn update_flowers_nectar(flowers: &mut Vec<Box<dyn Flower>>, terrain_grid: &TerrainGrid) {
    for flower in flowers.iter_mut() {
        let (x, y) = flower.get_position();
        if let Some(tile) = terrain_grid.get_tile(x, y) {
            flower.update_nectar(tile.temperature);
        }
    }
}
