use crate::canvas::draw_set_view_z;
use crate::masses::*;
use crate::ship::*;
use crate::simulation::*;
// for colours
use macroquad::prelude::*;

pub const SCENE: i16 = 7;

pub fn set_scene(scene_id: i16) -> (Simulation, Masses, Ship) {
    let mut simulation = Simulation::new(scene_id);
    let mut masses = Masses::new();
    let mut ship = Ship::default();

    // some masses
    let sol_data = MassData::fixstar("sun", YELLOW, km(1.3914e6), mass_sol(1.0));
    let sun_data = MassData::orbiter("su2", GOLD, km(1.3914e6), mass_sol(0.7), au(0.5));
    let earth_data = MassData::orbiter("earth", BLUE, km(12756.32), mass_earth(1.), au(1.));
    let big_dat1 = MassData::orbiter("big1", BLUE, km(12756.32), mass_sol(0.01), au(0.1));
    let big_dat2 = MassData::ellipse("big2", SKYBLUE, km(12756.32), mass_sol(0.01), au(0.15), 0.3);

    // Were are the values from?
    let _jupiter_d = MassData::orbiter("jupiter", GREEN, km(142984.0), kg(1.899e27), au(25e3));
    let comet_data = MassData::ellipse("comet", DARKGRAY, km(500.0), kg(1e6), au(1.3), 0.4);
    let luna_data = MassData::orbiter("luna", RED, km(3476.), kg(7.349e22), km(370171.));
    let ship_data = MassData::orbiter("ship", WHITE, 10.0, 0.0, km(300.)); // 80000. the real 300km are not visible

    // todo: make scenes to "new"-methodes (of simulations?)
    match scene_id {
        1 => {
            simulation.set_text("double star");
            let sun = masses.add_at_place(&sol_data);
            let _s2 = masses.add_in_orbit(&sun_data, sun);
        }

        2 => {
            simulation.set_text("Sun and two planets");
            let sun = masses.add_at_place(&sol_data);
            masses.add_in_orbit(&big_dat1, sun);
            masses.add_in_orbit(&big_dat2, sun);
            simulation.set_run_mode(false);
        }

        3 => {
            simulation.set_text("Sun, Earth");
            let sun = masses.add_at_place(&sol_data);
            masses.add_in_orbit(&earth_data, sun);
        }

        4 => {
            simulation.set_text("Sun, Earth & Luna +");
            let sun0 = masses.add_at_place(&sol_data);
            masses.add_in_orbit(&comet_data, sun0);
            let earth2 = masses.add_in_orbit(&earth_data, sun0);
            let _luna3 = masses.add_in_orbit(&luna_data, earth2);
            masses.set_follow(earth2);
            // ship.set_rotation(0., earth2, _luna3);
        }

        5 => {
            simulation.set_text("Earth & Luna & Ship");
            simulation.set_seconds_per_orbit(60.);
            let earth0 = masses.add_at_place(&earth_data);
            let _luna1 = masses.add_in_orbit(&luna_data, earth0);
            ship.set_in_orbit(&mut masses, &ship_data.mul_radius(100.), earth0); // The real 300km are not visible
        }

        6 => {
            simulation.set_text("Moon 8 loop (realy?)");
            let earth0 = masses.add_at_place(&earth_data);
            let luna1 = masses.add_in_orbit(&luna_data.mul_radius(0.1), earth0); // todo: real values ???
            ship.set_in_orbit(&mut masses, &ship_data.mul_radius(20.), earth0); // 26.6 The real 300km are not visible
            //ship.set_burn(3444., 13432.);
            //ship.set_burn(0., 13432.);
            ship.set_burn(3548., 13432.);
            //ship.set_burn_acceleration(0.3);

            masses.set_rotation(earth0, luna1);
            //simulation.set_follow(luna1);
            simulation.set_run_mode(false);
            masses.set_predict_show_and_count(300., 710.);
        }

        7 => {
            simulation.set_text("Artemis II?");
            // With real masses, the ship needs detailed/more time steps to have a sable orbit!
            // Increasing the ship orbit or decreasing the Luna orbit makes it sable but is not the intendet solutin
            // Slowing the simulation by 10000*10 works, but shows almost no predicted trajectory
            // Set the prediction to 10*1000 is ok, more breaks the FPS load
            // Subdivite a simulation step by 1000 for the ship helps, but makes the FPS load to high
            // So wee need a compromise: subdivide by 100, multiply the orbit tiem by 10 and set the prediction to 10*1000
            // The ship orbit still is odd: gets bigger and smaler ??? todo
            //
            // o1000*10+s10+viel"T" ist brauchbar(?)
            simulation.set_seconds_per_orbit(1000. * 10.); // 10'000*10 =ok !! 1'000*10 Schiff-orbit eiert
            ship.set_interpolate(10); // if orbit=100*10: 100= almost ok. OK with interpolation
            let earth0 = masses.add_at_place(&earth_data);
            let _luna1 = masses.add_in_orbit(&luna_data.mul_radius(1.), earth0); // 0.005=OK, mehr eigert
            ship.set_in_orbit(&mut masses, &ship_data.mul_radius(1.), earth0); // 500=ok 100=eiert
            //ship.set_burn(3548., 13432.);
            masses.set_follow(earth0);
            //ship.set_rotation(0., earth0, luna1);
            //masses.set_predict_show_and_count(300., 0.); // 300., 211.
            simulation.set_run_mode(true);
        }

        // Idee: Flyby führt zu langsammer/schneller/andere Richtung/bis zu 180 grad umkehr
        //
        9 => {
            // https://en.wikipedia.org/wiki/Lagrange_point
            simulation.set_text("Lagrange 4/5");

            let sun0 = masses.add_at_place(&sol_data);
            let earth1 = masses.add_in_orbit_lagrange(&earth_data, sun0, 4);
            //t luna_data = MassData::orbiter("luna", RED, km(3476.), kg(7.349e22), km(370171.));
            let test_data = MassData::orbiter("test", WHITE, km(1.), kg(0.), km(5. * 370171.));
            //t test_data = MassData::orbiter("test", WHITE, km(1.), kg(0.), -km(1. * 1480111.));
            let _test2 = masses.add_in_orbit(&test_data, earth1);
            // ship !!!!
            //ship.set_burn_acceleration(A_BURN * 1.);
            //ship.set_burn(0., 2222.);
            //ship.set_burn(0.08034150634335382, 4647.);

            draw_set_view_z(20.);
            masses.set_follow(earth1);
            masses.set_rotation(earth1, sun0);
            simulation.set_run_mode(false);
        }

        _ => {
            simulation.set_text("Scene NOT DEFINED");
        }
    };

    // All masses are there, calculate the simulation seconds per iteration by the maximal ui-orbit-time
    simulation.calc_orbit_seconds(&masses);

    masses.set_radius_to_draw();

    // initially simulate all the future positinos
    masses.predict_positions(&mut simulation);
    ship.predict_positions(&simulation, &masses);

    (simulation, masses, ship)
}
