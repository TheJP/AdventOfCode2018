const PLAYERS: usize = 462;
const MARBLES: i32 = 71938;

fn main() {
    let mut players = vec![0; PLAYERS];
    let mut marbles = vec![0];

    let mut current: i32 = 0;
    let mut mod23 = 1;
    let mut player = 0;
    for i in 1..MARBLES + 1 {
        match mod23 {
            23 => {
                current -= 7;
                if current < 0 { current += marbles.len() as i32; }
                players[player] += i + marbles.remove(current as usize);
                if current >= marbles.len() as i32 { current = 0; }
                mod23 = 1;
            }
            _ => {
                current += 2;
                if current > marbles.len() as i32 { current -= marbles.len() as i32; }
                marbles.insert(current as usize, i);
                mod23 += 1;
            }
        }

        player = match player + 1 {
            PLAYERS => { 0 }
            _ => { player + 1 }
        }
    }

    println!("{}", players.iter().max().unwrap());
}
