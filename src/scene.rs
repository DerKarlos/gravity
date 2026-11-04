use crate::canvas::draw_set_view_z;
use crate::mass::*;
use crate::ship::*;
use crate::simulation::*;
use crate::vec_space::*;

// for colours
use macroquad::prelude::*;

pub const SCENE: i16 = 9;

pub fn set_scene(scene_id: i16) -> Simulation {
    let mut simulation = Simulation::new(scene_id);
    let mut ship = Ship::default();

    // some masses
    let earth: MassProperties = MassProperties::new("earth", mass_earth(1.), km(12756.32), BLUE);
    pub const SOL: MassProperties = MassProperties::new("sun", mass_sol(1.0), km(1.3914e6), YELLOW);
    pub const SUN: MassProperties = MassProperties::new("sun", mass_sol(0.7), km(1.3914e6), GOLD);
    let big1: MassProperties = MassProperties::new("big1", mass_sol(0.01), km(12756.32), BLUE);
    let big2: MassProperties = MassProperties::new("big2", mass_sol(0.01), km(12756.32), SKYBLUE);
    let comet: MassProperties = MassProperties::new("comet", kg(1e6), km(500.0), DARKGRAY);
    let luna: MassProperties = MassProperties::new("luna", kg(7.349e22), km(3476.), RED);

    let _ol_data = MassData::fixstar("sun", YELLOW, km(1.3914e6), mass_sol(1.0));
    let _un_data = MassData::orbiter("su2", GOLD, km(1.3914e6), mass_sol(0.7), au(0.5));
    let _arth_data = MassData::orbiter("earth", BLUE, km(12756.32), mass_earth(1.), au(1.));
    let _ig_dat1 = MassData::orbiter("big1", BLUE, km(12756.32), mass_sol(0.01), au(0.1));
    let _ig_dat2 = MassData::ellipse("big2", SKYBLUE, km(12756.32), mass_sol(0.01), au(0.15), 0.3);
    // Were are the values from?
    let _jupiter_d = MassData::orbiter("jupiter", GREEN, km(142984.0), kg(1.899e27), au(25e3));
    let _omet_data = MassData::ellipse("comet", DARKGRAY, km(500.0), kg(1e6), au(1.3), 0.4);
    let _una_data = MassData::orbiter("luna", RED, km(3476.), kg(7.349e22), km(370171.));
    let ship_data = MassData::orbiter("ship", WHITE, 10.0, 0.0, km(300.)); // 80000. the real 300km are not visible

    // some orbits
    let left_vector: MassVector = MassVector::new(VecSpace::new(-au(0.2), 0.), VecSpace::ZERO);
    pub const EARTH_ORBIT: MassOrbit = MassOrbit::new(au(1.), 0.);
    pub const SOL_ORBIT: MassOrbit = MassOrbit::new(au(0.5), 0.);
    pub const BIG1_ORBIT: MassOrbit = MassOrbit::new(au(0.10), 0.0);
    pub const BIG2_ORBIT: MassOrbit = MassOrbit::new(au(0.15), 0.3);
    pub const COMET_ORBIT: MassOrbit = MassOrbit::new(au(1.3), 0.4);
    pub const LUNA_ORBIT: MassOrbit = MassOrbit::new(km(370171.), 0.0);
    pub const LUN2_ORBIT: MassOrbit = MassOrbit::new(km(37017.1), 0.0);

    // todo: make scenes to "new"-methodes (of simulations?)
    match scene_id {
        1 => {
            simulation.set_text("double star");
            let sol = simulation.masses.add_by_vector(SOL, left_vector);
            let _s2 = simulation.masses.add_by_orbit(SUN, SOL_ORBIT, sol);
        }

        2 => {
            simulation.set_text("Sun and two planets");
            let sol = simulation.masses.add_by_center(SOL);
            simulation.masses.add_by_orbit(big1, BIG1_ORBIT, sol);
            simulation.masses.add_by_orbit(big2, BIG2_ORBIT, sol);
            simulation.set_run_mode(false);
        }

        3 => {
            simulation.set_text("Sun, Earth");
            let sol = simulation.masses.add_by_vector(SOL, MassVector::ZERO);
            let ___ = simulation.masses.add_by_orbit(earth, EARTH_ORBIT, sol);
        }

        4 => {
            simulation.set_text("Sun, Earth & Luna +");
            let sol = simulation.masses.add_by_vector(SOL, MassVector::ZERO);
            let ___ = simulation.masses.add_by_orbit(comet, COMET_ORBIT, sol);
            let ert = simulation.masses.add_by_orbit(earth, EARTH_ORBIT, sol);
            let ___ = simulation.masses.add_by_orbit(luna, LUNA_ORBIT, ert);

            simulation.masses.set_follow(ert);
            // ship.set_rotation(0., earth2, _luna3);
        }

        5 => {
            simulation.set_text("Earth & Luna & Ship");
            simulation.set_seconds_per_orbit(60.);

            let ert = simulation.masses.add_by_center(earth);
            let ___ = simulation.masses.add_by_orbit(luna, LUNA_ORBIT, ert);
            // ???
            ship.set_in_orbit(&mut simulation, &ship_data.mul_radius(100.), ert); // The real 300km are not visible
        }

        6 => {
            simulation.set_text("Moon 8 loop (realy?)");
            let ert = simulation.masses.add_by_center(earth);
            let lna = simulation.masses.add_by_orbit(luna, LUN2_ORBIT, ert); // todo: real values ???

            ship.set_in_orbit(&mut simulation, &ship_data.mul_radius(20.), ert); // 26.6 The real 300km are not visible
            //ship.set_burn(3444., 13432.);
            //ship.set_burn(0., 13432.);
            ship.set_burn(3548., 13432.);
            //ship.set_burn_acceleration(0.3);

            simulation.masses.set_rotation(ert, lna);
            //simulation.masses.set_follow(lna);

            simulation.set_run_mode(false);
            simulation.masses.set_predict_show_and_count(300., 710.);
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

            let ert = simulation.masses.add_by_center(earth);
            let ___ = simulation.masses.add_by_orbit(luna, LUNA_ORBIT, ert);

            //let earth0 = simulation.masses.add_at_place(&earth_data);
            //let _luna1 = simulation
            //    .masses
            //    .add_in_orbit(&luna_data.mul_radius(1.), earth0); // 0.005=OK, mehr eigert
            ship.set_in_orbit(&mut simulation, &ship_data.mul_radius(1.), ert); // 500=ok 100=eiert
            //ship.set_burn(3548., 13432.);
            simulation.masses.set_follow(ert);
            //ship.set_rotation(0., earth0, luna1);
            //masses.set_predict_show_and_count(300., 0.); // 300., 211.
            simulation.set_run_mode(true);
        }

        // Idee: Flyby führt zu langsammer/schneller/andere Richtung/bis zu 180 grad umkehr
        //
        9 => {
            // https://en.wikipedia.org/wiki/Lagrange_point
            simulation.set_text("Lagrange 4/5");

            //t sun0 = simulation.masses.add_at_place(&sol_data);
            let sol = simulation.masses.add_by_center(SOL);
            let ert = simulation.masses.add_by_orbit(earth, EARTH_ORBIT, sol); // lagrange 4 !!! ???
            //t earth1 = simulation
            //  .masses
            //  .add_in_orbit_lagrange(&earth_data, sun0, 4);
            let ___ = simulation.masses.add_by_orbit(luna, LUNA_ORBIT, ert);

            //t luna_data = MassData::orbiter("luna", RED, km(3476.), kg(7.349e22), km(370171.));

            //t test_data = MassData::orbiter("test", WHITE, km(1.), kg(0.), km(5. * 370171.));
            let test = MassProperties::new("test", kg(0.), km(1.), WHITE);
            let test_orbit = MassOrbit::new(370171., 0.);

            //t test_data = MassData::orbiter("test", WHITE, km(1.), kg(0.), -km(1. * 1480111.));

            //let _test2 = simulation.masses.add_in_orbit(&test_data, earth1);
            let ___ = simulation.masses.add_by_orbit(test, test_orbit, ert);

            // ship !!!!
            //ship.set_burn_acceleration(A_BURN * 1.);
            //ship.set_burn(0., 2222.);
            //ship.set_burn(0.08034150634335382, 4647.);

            draw_set_view_z(20.);
            simulation.masses.set_follow(ert);
            simulation.masses.set_rotation(ert, sol);
            simulation.set_run_mode(false);
        }

        _ => {
            simulation.set_text("Scene NOT DEFINED");
        }
    };

    // All masses are there, calculate the simulation seconds per iteration by the maximal ui-orbit-time
    simulation.calc_orbit_seconds();

    simulation.masses.set_radius_to_draw();

    // All masses are there, calculate the simulation seconds by the maximal orbit time (as ui-time!)
    simulation.calc_orbit_seconds();
    // initially simulate all the future positinos
    simulation
        .masses
        .predict_positions(simulation.simulated_seconds_per_step());
    ship.predict_positions(
        &simulation.masses,
        simulation.simulated_seconds(),
        simulation.simulated_seconds_per_step(),
    );

    simulation.ships.push(ship);

    simulation
}
