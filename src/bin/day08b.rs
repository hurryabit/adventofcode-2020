use std::collections::BTreeSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

#[derive(Copy, Clone)]
enum Cmd {
    Nop,
    Acc,
    Jmp,
}

fn main() -> Result<(), Error> {
    let reader = BufReader::new(File::open("inputs/day08.txt")?);
    let mut cmds: Vec<_> = reader.lines().map(|line| {
        let line = line.unwrap();
        let words: Vec<_> = line.split_ascii_whitespace().collect();
        let cmd = match words[0] {
            "nop" => Cmd::Nop,
            "acc" => Cmd::Acc,
            "jmp" => Cmd::Jmp,
            other => panic!("unknown command: {}", other),
        };
        let num: i64 = words[1].parse().unwrap();
        (cmd, num)
    }).collect();

    for i in 0..cmds.len() {
        let mut acc: i64 = 0;
        let mut ctr: i64 = 0;
        let mut seen = BTreeSet::new();
        let orig_cmd = cmds[i as usize].0;
        cmds[i as usize].0 = match orig_cmd {
            Cmd::Nop => Cmd::Jmp,
            Cmd::Acc => Cmd::Acc,
            Cmd::Jmp => Cmd::Nop,
        };

        loop {
            if ctr >= cmds.len() as i64 {
                println!("acc = {}", acc);
                return Ok(())
            } else if seen.contains(&ctr) {
                break;
            } else {
                seen.insert(ctr);
            }
            let (cmd, num) = cmds[ctr as usize];
            match cmd {
                Cmd::Nop => ctr += 1,
                Cmd::Acc => {
                    acc += num;
                    ctr += 1;
                }
                Cmd::Jmp => ctr += num,
            }
        }

        cmds[i].0 = orig_cmd;
    }

    println!("could not find non looping version");
    Ok(())
}
