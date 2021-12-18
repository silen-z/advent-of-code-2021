const INPUT: &'static str = include_str!("../../input/day02.txt");

enum Command {
    Forward(u32),
    Up(u32),
    Down(u32),
}

fn main() {
    let commands: Vec<Command> = INPUT.lines().map(|l| l.parse().unwrap()).collect();

    let (position, depth) =
        commands
            .iter()
            .fold((0, 0), |(position, depth), command| match command {
                Command::Forward(n) => (position + n, depth),
                Command::Up(n) => (position, depth - n),
                Command::Down(n) => (position, depth + n),
            });

    println!("{}", position * depth);

    let (position, depth, _) =
        commands
            .iter()
            .fold((0, 0, 0), |(position, depth, aim), command| match command {
                Command::Forward(n) => (position + n, depth + aim * n, aim),
                Command::Up(n) => (position, depth, aim - n),
                Command::Down(n) => (position, depth, aim + n),
            });

    println!("{}", position * depth);
}

impl std::str::FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, amount) = s
            .split_once(' ')
            .ok_or_else(|| format!("Invalid command format: {}", s))?;

        let amount: u32 = amount
            .parse()
            .map_err(|_| format!("Amount: {} is not a number", amount))?;

        match dir {
            "forward" => Ok(Command::Forward(amount)),
            "up" => Ok(Command::Up(amount)),
            "down" => Ok(Command::Down(amount)),
            _ => Err(format!("Invalid direction: {}", dir)),
        }
    }
}
