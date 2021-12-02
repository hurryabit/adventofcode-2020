use std::collections::VecDeque;

type Config = VecDeque<usize>;

fn step(config: &mut Config) {
    let size = config.len();
    let mut head = *config.front().unwrap();
    let removed = config.drain(1..4).collect::<Vec<_>>();
    let insert_at = loop {
        head -= 1;
        if head == 0 {
            head += size;
        }
        if let Some(index) = config.iter().position(|x| *x == head) {
            break index;
        }
    };
    let rest = config.split_off(insert_at + 1);
    config.extend(removed);
    config.extend(rest);
    config.rotate_left(1);
}

fn make_config(mut repr: usize) -> Config {
    let mut result = Config::with_capacity(9);
    for _ in 0..9 {
        result.push_front(repr % 10);
        repr /= 10;
    }
    result
}

fn config_repr(mut config: Config) -> usize {
    let index = config.iter().position(|x| *x == 1).unwrap();
    config.rotate_left(index);
    config.pop_front();
    let mut result = 0;
    for digit in config {
        result = 10 * result + digit;
    }
    result
}

fn main() {
    let mut config = make_config(739862541);
    for _ in 0..100 {
        step(&mut config);
    }
    let result = config_repr(config);
    println!("The labels on the cups are {}", result);
}
