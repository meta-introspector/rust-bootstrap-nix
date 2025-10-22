mod config_generator;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <stage_num> <target_triple>", args[0]);
        std::process::exit(1);
    }

    let stage_num = &args[1];
    let target_triple = &args[2];

    config_generator::generate_config_toml(stage_num, target_triple);
}
