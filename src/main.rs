use std::io::Write;

fn main() {
    let mut stdout = std::io::stdout();
    let first_part = &"\
⠀⠀⠀⠀⠀⠀⠀⠀⣠⣶⣿⣿⣿⣷⣤⡀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⢀⣾⡿⠋⠀⠿⠇⠉⠻⣿⣄⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⢠⣿⠏⠀⠀⠀⠀⠀⠀⠀⠙⣿⣆⠀⠀⠀⠀⠀
⠀⠀⠀⠀⢠⣿⡏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⣿⣆⠀⠀⠀⠀
⠀⠀⠀⠀⢸⣿⡄⠀⠀⠀⢀⣤⣀⠀⠀⠀⠀⣿⡿⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠻⣿⣶⣶⣾⡿⠟⢿⣷⣶⣶⣿⡟⠁⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⣿⡏⠉⠁⠀⠀⠀⠀⠉⠉⣿⡇⠀⠀⠀⠀⠀
";
    let third_part = &"\
⠀⠀⠀⠀⠀⠀⣿⡇⠀⠀⣸⣿⠀⠀⠀⠀⣿⡇⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⣿⡇⢀⣴⣿⠇⠀⠀⠀⠀⣿⡇⠀⠀⠀⠀⠀
⠀⠀⠀⢀⣠⣴⣿⣷⣿⠟⠁⠀⠀⠀⠀⠀⣿⣧⣄⡀⠀⠀⠀
⠀⢀⣴⡿⠛⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠙⢿⣷⣄⠀
⢠⣿⠏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⣿⣆
⣿⡟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢹⣿
⣿⣇⠀⠀⠀⠀⠀⠀⢸⣿⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿
⢹⣿⡄⠀⠀⠀⠀⠀⠀⢿⣷⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⡿
⠀⠻⣿⣦⣀⠀⠀⠀⠀⠈⣿⣷⣄⡀⠀⠀⠀⠀⣀⣤⣾⡟⠁
⠀⠀⠈⠛⠿⣿⣷⣶⣾⡿⠿⠛⠻⢿⣿⣶⣾⣿⠿⠛⠉⠀⠀
";

    stdout.write_all(first_part.as_bytes()).unwrap();

    const DEFAULT_SIZE: usize = 0;
    let size = std::env::args()
        .nth(1)
        .map(|n| n.parse::<usize>().unwrap_or(DEFAULT_SIZE))
        .unwrap_or(DEFAULT_SIZE);
    let repeated_thing = &"⠀⠀⠀⠀⠀⠀⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⣿⡇
";
    for _ in 0..size {
        stdout.write_all(repeated_thing.as_bytes()).unwrap();
    }

    stdout.write_all(third_part.as_bytes()).unwrap();
    stdout.flush().unwrap();
}
