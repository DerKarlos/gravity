use crate::canvas::*;
use crate::masses::*;
use crate::ship::*;
use crate::simulation::*;
use crate::*;

use macroquad::prelude::*;

pub fn set_scene(scene: i16) -> (Simulation, Masses, Ship, Canvas) {
    let mut simulation = Simulation::new(scene);
    let mut masses = Masses::new();
    let mut ship = Ship::default();
    let mut canvas = Canvas::new(&conf());

    // some masses
    let sun_data = MassData::fixstar("sun", YELLOW, km(1.3914e6), mass_sol(1.));
    let sun_dat2 = MassData::orbiter("sun2", GOLD, km(1.3914e6), mass_sol(1.), au(0.5));
    let earth_data = MassData::orbiter("earth", BLUE, km(12756.32), mass_earth(1.), au(1.));
    let big_dat1 = MassData::orbiter("earth", BLUE, km(12756.32), mass_sol(0.01), au(0.1));
    let big_dat2 = MassData::ellipse("earth", BLUE, km(12756.32), mass_sol(0.01), au(0.15), 0.3);

    // more but 0.005 AE radius makes the orbit insable.
    let luna_data = MassData::orbiter("luna", RED, km(3476.), kg(7.349e22), km(370171.));
    let _jupiter_d = MassData::orbiter("jupiter", GREEN, km(142984.0), kg(1.899e27), au(25e3));
    let comet_data = MassData::ellipse("comet", WHITE, km(500.0), kg(1e6), au(1.3), 0.4);
    let ship_data = MassData::orbiter("ship", WHITE, 10.0, 0.0, km(80000.)); // the real 300km are not visible

    match scene {
        1 => {
            simulation.set_text("Sun, Earth");
            let sun = masses.add_at_place(&sun_data);
            masses.add_in_orbit(&earth_data, sun);
        }

        2 => {
            simulation.set_text("double star");
            let sun = masses.add_at_place(&sun_data);
            masses.add_in_orbit(&sun_dat2, sun);
        }

        3 => {
            simulation.set_text("Earth & Luna & Ship");
            simulation.set_seconds_per_orbit(60.);
            let earth = masses.add_at_place(&earth_data);
            masses.add_in_orbit(&luna_data, earth);
            //???ship.set_in_orbit(&mut masses, &ship_data, earth);
        }

        4 => {
            simulation.set_text("Sun, Earth & Luna +");
            let sun = masses.add_at_place(&sun_data);
            let earth = masses.add_in_orbit(&earth_data, sun);
            masses.add_in_orbit(&luna_data, earth);
            masses.add_in_orbit(&comet_data, sun);
        }

        5 => {
            simulation.set_text("Sun, Earth+Earth");
            let sun = masses.add_at_place(&sun_data);
            masses.add_in_orbit(&big_dat1, sun);
            masses.add_in_orbit(&big_dat2, sun);
            simulation.run_mode = false;
        }

        _ => {
            simulation.set_text("Test");
            let earth = masses.add_at_place(&earth_data);
            masses.add_in_orbit(&luna_data.mul_radius(0.1), earth);
            ship.set_in_orbit(&mut masses, &ship_data.mul_radius(0.1), earth);
            ship.set_burn(0.289, 1.1781022706580768); // Luna-Orbit
            ship.set_burn(0.339, 1.48); // Not an 8 curse yet
            simulation.run_mode = false;
        }
    };

    // All masses are there, calculate the simulation time by the maximal orbit time
    simulation.set_orbit_time(&masses);
    masses.set_radius(&mut canvas);

    // initially simulate all the future positinos
    masses.predict_positions(&mut simulation);
    ship.predict_positions(&mut simulation, &mut masses);

    (simulation, masses, ship, canvas)
}
