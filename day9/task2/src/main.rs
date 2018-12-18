const PLAYERS: usize = 462;
const MARBLES: i64 = 71938 * 100;

#[derive(Clone)]
struct Node {
    back: usize,
    next: usize,
    data: i64,
}

impl Node {
    fn new(data: i64) -> Self {
        Node { back: 0, next: 0, data }
    }
}

fn insert(list: &mut Vec<Node>, position: usize, data: i64) -> usize {
    let new_position = list.len();
    list.push(Node::new(data));

    let next = list[position].next;
    list[position].next = new_position;
    list[next].back = new_position;

    list[new_position].next = next;
    list[new_position].back = position;

    new_position
}

fn remove(list: &mut Vec<Node>, position: usize) -> i64 {
    // Not removing it from the list so the indexing does not get messed up
    let old_node = list[position].clone();
    list[old_node.back].next = old_node.next;
    list[old_node.next].back = old_node.back;
    old_node.data
}

fn main() {
    let mut linked_list = Vec::new();
    let mut players = vec![0; PLAYERS];

    linked_list.push(Node::new(0));

    let mut current = 0;
    let mut mod23 = 1;
    let mut player = 0;

    for marble in 1..MARBLES + 1 {
        match mod23 {
            23 => {
                for _ in 0..6 {
                    current = linked_list[current].back;
                }
                let old = linked_list[current].back;
                players[player] += remove(&mut linked_list, old) + marble;

                mod23 = 1;
            }
            _ => {
                let position = linked_list[current].next;
                current = insert(&mut linked_list, position, marble);
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
