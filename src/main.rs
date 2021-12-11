mod r_cube_mod;
mod sim_mod;
mod cube_side_mod;

use crate::sim_mod::Sim as Sim;

fn main() {
    let mut sim_sim = Sim::new();
    sim_sim.randomize_cube_mut(100);
    println!("Cube: {}",sim_sim.show_cube_status());
}
