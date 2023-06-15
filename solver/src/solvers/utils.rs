// Macro to print progress of BFS.
macro_rules! print_progress {
    ($title:expr, $name:expr, $current:ident, $total:ident) => {
        if $current % 1000 == 0 {
            let icon = match ($current / 1000) % 4 {
                0 => "◜",
                1 => "◝",
                2 => "◞",
                3 => "◟",
                _ => unreachable!(),
            };
            print!("{} {} {}: {} / {}\r", $title, $name, icon, $current, $total);
            std::io::stdout().flush().unwrap();
        }
    };
}

pub(crate) use print_progress;

macro_rules! print_terminated {
    ($title:expr, $name:expr, $current:expr, $total:ident) => {
        println!("{} {} ✅: {} / {}", $title, $name, $current, $total);
    };
}

pub(crate) use print_terminated;
