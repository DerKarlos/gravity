//use crate::canvas::draw_set_view;
use crate::masses::*;
use crate::ship::*;
use crate::simulation::*;
// for colours
use macroquad::prelude::*;

pub fn set_scene(scene_id: i16) -> (Simulation, Masses, Ship) {
    let mut simulation = Simulation::new(scene_id);
    let mut masses = Masses::new();
    let mut ship = Ship::default();

    // some masses
    let sun_data = MassData::fixstar("sun", YELLOW, km(1.3914e6), mass_sol(1.));
    let sun_dat2 = MassData::orbiter("sun2", GOLD, km(1.3914e6), mass_sol(1.), au(0.5));
    let earth_data = MassData::orbiter("earth", BLUE, km(12756.32), mass_earth(1.), au(1.));
    let big_dat1 = MassData::orbiter("big1", BLUE, km(12756.32), mass_sol(0.01), au(0.1));
    let big_dat2 = MassData::ellipse("big2", SKYBLUE, km(12756.32), mass_sol(0.01), au(0.15), 0.3);

    // more but 0.005 AE radius makes the orbit insable.
    let luna_data = MassData::orbiter("luna", RED, km(3476.), kg(7.349e22), km(370171.));
    let _jupiter_d = MassData::orbiter("jupiter", GREEN, km(142984.0), kg(1.899e27), au(25e3));
    let comet_data = MassData::ellipse("comet", DARKGRAY, km(500.0), kg(1e6), au(1.3), 0.4);
    let ship_data = MassData::orbiter("ship", WHITE, 10.0, 0.0, km(80000.)); // the real 300km are not visible

    match scene_id {
        1 => {
            simulation.set_text("double star");
            let sun = masses.add_at_place(&sun_data);
            let su2 = masses.add_in_orbit(&sun_dat2, sun);
            ship.set_in_orbit(&mut masses, &ship_data.mul_radius(200.), su2);
        }

        2 => {
            simulation.set_text("Sun and two planets");
            let sun = masses.add_at_place(&sun_data);
            masses.add_in_orbit(&big_dat1, sun);
            masses.add_in_orbit(&big_dat2, sun);
            simulation.set_run_mode(false);
        }

        3 => {
            simulation.set_text("Sun, Earth");
            let sun = masses.add_at_place(&sun_data);
            masses.add_in_orbit(&earth_data, sun);
        }

        4 => {
            simulation.set_text("Sun, Earth & Luna +");
            let sun = masses.add_at_place(&sun_data);
            masses.add_in_orbit(&comet_data, sun);
            let earth = masses.add_in_orbit(&earth_data, sun);
            masses.add_in_orbit(&luna_data, earth);
        }

        5 => {
            simulation.set_text("Earth & Luna & Ship");
            simulation.set_seconds_per_orbit(60.);
            let earth = masses.add_at_place(&earth_data);
            masses.add_in_orbit(&luna_data, earth);
            masses.set_in_orbit(&ship_data, earth);
            ship.set_in_orbit(&mut masses, &ship_data, earth);
        }

        6 => {
            simulation.set_text("Moon 8 loop (realy?)");
            let earth = masses.add_at_place(&earth_data);
            let _luna = masses.add_in_orbit(&luna_data.mul_radius(0.1), earth);
            ship.set_in_orbit(&mut masses, &ship_data.mul_radius(0.1), earth);
            ship.set_burn(2154.2760453997807, 10424.84718004583);
            simulation.set_run_mode(false);
            // ship.set_rotation(0., earth, luna);
        }

        // Idee: Flyby führt zu langsammer/schneller/andere Richtung/bis zu 180 grad umkehr
        //
        7 => {
            // https://en.wikipedia.org/wiki/Lagrange_point
            simulation.set_text("Lagrange 4/5");

            //simulation.set_seconds_per_orbit(60.);
            let sun = masses.add_at_place(&sun_data);
            let su2 = masses.add_in_orbit(&earth_data, sun);
            ship.set_in_orbit(&mut masses, &ship_data.mul_radius(20.), su2);

            //let sun = masses.add_at_place(&sun_data);
            //let earth = masses.add_in_orbit(&earth_data, sun);
            //ship.add_in_orbit(&mut masses, &ship_data.mul_radius(100.), earth);

            //ship.set_burn_acceleration(A_BURN * 1.);
            //ship.set_burn(0., 10424.84718004583);

            simulation.set_run_mode(false);
        }

        _ => {
            simulation.set_text("Scene NOT DEFINED");
        }
    };

    // All masses are there, calculate the simulation seconds by the maximal ui-orbit-time
    simulation.set_orbit_seconds(&masses);

    //äää
    simulation.simulated_seconds_per_step = masses.maximal_orbit_seconds()
        / SIMULATION_STEPS_PER_APP_SECOND
        / simulation.ui_time_per_orbit;

    masses.set_radius();

    // initially simulate all the future positinos
    masses.predict_positions(&mut simulation);
    ship.predict_positions(&simulation, &masses);

    (simulation, masses, ship)
}
