mod terrain;
use terrain::TerrainGrid;

mod simulation_time; // This line declares the simulation_time module
use simulation_time::SimulationTime; // Correctly scoped import


fn main() {
    let mut terrain = TerrainGrid::new(10, 10); // Example size
    let mut simulation_time = SimulationTime::new();

    // main simulation loop
    loop {
        update(&mut terrain, &mut simulation_time);
    }
}

fn update(terrain: &mut TerrainGrid, simulation_time: &mut SimulationTime) {
    simulation_time.advance();
    terrain.update_temperature(simulation_time.hour);
    println!("Hour: {}, Updating temperatures...", simulation_time.hour);
}

