use clap::{Arg, ArgMatches, Command};
use matahatan_lib::{run_simulation, Config};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut command = Command::new("Matahatan")
        .version("0.1")
        .author("Adfinis AG")
        .about("Virtual Maze Solving Challenge")
        .subcommand(
            Command::new("simulate")
                .about("Test/train your maze-solver")
                .arg(
                    Arg::new("lua")
                        .short('l')
                        .long("lua")
                        .value_name("FILE")
                        .help("Run the lua-script FILE (enables lua-mode)")
                        .num_args(1),
                )
                .arg(
                    Arg::new("fps")
                        .short('f')
                        .long("fps")
                        .default_value("25")
                        .value_name("FPS")
                        .help("FPS of the simulation not the GUI (0 as fast as possible)")
                        .num_args(1),
                )
                .arg(
                    Arg::new("no-gui")
                        .short('x')
                        .long("no-gui")
                        .help("Do not run GUI (unattended training) sets FPS to 0")
                        .num_args(0),
                )
                .arg(
                    Arg::new("stdio")
                        .short('o')
                        .long("stdio")
                        .help("Run the simulation in stdio-mode")
                        .num_args(0),
                )
                .arg(
                    Arg::new("stick")
                        .short('s')
                        .long("stick")
                        .help("Run the simulation with stick (gamepad/joystick)")
                        .num_args(0),
                ),
        )
        .subcommand(
            Command::new("server")
                .about("Run the maze-solver server (the actual challenges will be missing)"),
        );
    let matches = command.clone().get_matches();

    match matches.subcommand() {
        Some(("simulate", m)) => simulate(m),
        Some(("server", _m)) => (),
        _ => command.print_long_help()?,
    }
    Ok(())
}

fn simulate(m: &ArgMatches) {
    let gui = !m.get_flag("no-gui");
    let stick = m.get_flag("stick");
    let framerate: f32 = match m.get_one::<String>("fps") {
        Some(fps_str) => fps_str.parse().unwrap_or(25.0_f32),
        None => 25.0_f32,
    };
    let config = Config {
        gui,
        stick,
        framerate,
    };
    run_simulation(&config);
}
