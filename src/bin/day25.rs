const BASE: u64 = 7;
const MOD: u64 = 20201227;

const CARD_PUBLIC: u64 = 335121;
const DOOR_PUBLIC: u64 = 363891;

fn main() {
    let mut pow = 1;
    let mut card_encryption = 1;
    let mut door_encryption = 1;
    let encrytion = loop {
        if pow == CARD_PUBLIC {
            break door_encryption;
        } else if pow == DOOR_PUBLIC {
            break card_encryption;
        } else {
            pow = (pow * BASE) % MOD;
            card_encryption = (card_encryption * CARD_PUBLIC) % MOD;
            door_encryption = (door_encryption * DOOR_PUBLIC) % MOD;
        }
    };
    println!("The encryption key is {}", encrytion);
}
