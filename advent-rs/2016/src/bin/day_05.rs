// day 5: how about a nice game of chess?

fn find_door_code_part1(door_id: &str) -> String {
    let mut door_code: String = String::new();
    let mut index = 0;

    let door_id: String = String::from(door_id);
    let mut door_id_with_index: String;
    let mut hash: String;

    while door_code.len() < 8 {
        door_id_with_index = door_id.clone() + &index.to_string();
        hash = format!("{:x}", md5::compute(&door_id_with_index));

        if hash.starts_with("00000") {
            door_code.push(hash.chars().nth(5).unwrap());
        }

        index += 1;
    }

    door_code
}

fn find_door_code_part2(door_id: &str) -> String {
    let mut door_code: [char; 8] = ['_', '_', '_', '_', '_', '_', '_', '_'];
    let mut index = 0;

    let door_id: String = String::from(door_id);
    let mut door_id_with_index: String;
    let mut hash: String;

    while door_code.contains(&'_') {
        door_id_with_index = door_id.clone() + &index.to_string();
        hash = format!("{:x}", md5::compute(&door_id_with_index));

        if hash.starts_with("00000") {
            if let Some(index_char) = hash.chars().nth(5).unwrap().to_digit(10) {
                let index_char = index_char as usize;
                if index_char < door_code.len() && door_code[index_char] == '_' {
                    door_code[index_char as usize] = hash.chars().nth(6).unwrap();
                }
            }
        }

        index += 1;
    }

    door_code.into_iter().collect::<String>()
}

fn main() {
    // d4cd2ee1
    println!("{}", find_door_code_part1("ugkcyxxp"));

    // f2c730e5
    println!("{}", find_door_code_part2("ugkcyxxp"));
}
