use std::fs;
use std::str::Chars;

fn code_letter(rotor_choice: [usize; 3], rotor_state: [usize; 3], switchboard: [usize; 27], ring_settings: [usize; 3], letter: usize) -> usize {
    
    let reflector: [usize; 27] = 
        [0, 25, 18, 21, 8, 17, 19, 12, 4, 16, 24, 14, 7, 15, 11, 13, 9, 5, 2, 6, 26, 3, 23, 22, 10, 1, 20];
    let rotors: [[usize; 27]; 8] = [
        [0, 5, 11, 13, 6, 12, 7, 4, 17, 22, 26, 14, 20, 15, 23, 25, 8, 24, 21, 19, 16, 1, 9, 2, 18, 3, 10], 
        [0, 1, 10, 4, 11, 19, 9, 18, 21, 24, 2, 12, 8, 23, 20, 13, 3, 17, 7, 26, 14, 16, 25, 6, 22, 15, 5],
        [0, 2, 4, 6, 8, 10, 12, 3, 16, 18, 20, 24, 22, 26, 14, 25, 5, 9, 23, 7, 1, 11, 13, 21, 19, 17, 15],
        [0, 5, 19, 15, 22, 16, 26, 10, 1, 25, 17, 21, 9, 18, 8, 24, 12, 14, 6, 20, 7, 11, 4, 3, 13, 23, 2],
        [0, 22, 26, 2, 18, 7, 9, 20, 25, 21, 16, 19, 4, 14, 8, 12, 24, 1, 23, 13, 10, 17, 15, 6, 5, 3, 11],
        [0, 10, 16, 7, 22, 15, 21, 13, 6, 25, 17, 2, 5, 14, 8, 26, 18, 4, 11, 1, 19, 24, 12, 9, 3, 20, 23],
        [0, 14, 26, 10, 8, 7, 18, 3, 24, 13, 25, 19, 23, 2, 15, 21, 6, 1, 9, 22, 12, 16, 5, 11, 17, 4, 20],
        [0, 6, 11, 17, 8, 20, 12, 24, 15, 3, 2, 10, 19, 16, 4, 26, 18, 1, 13, 5, 23, 14, 9, 21, 25, 7, 22]
    ];
    let mut encoded_letter: usize = switchboard[letter];
    // apply one way trip
    for i in (0..3).rev() {
        if encoded_letter <= ring_settings[i] {
            encoded_letter = 26 + encoded_letter;
        }
        encoded_letter -= ring_settings[i];
        encoded_letter += rotor_state[i];
        if encoded_letter > 26 {
            encoded_letter -= 26;
        }
        encoded_letter = rotors[rotor_choice[i]][encoded_letter];
        if encoded_letter <= rotor_state[i] {
            encoded_letter = 26 + encoded_letter;
        }
        encoded_letter -= rotor_state[i];
        encoded_letter += ring_settings[i];
        if encoded_letter > 26 {
            encoded_letter -= 26;
        }
    }

    //apply reflector
    encoded_letter = reflector[encoded_letter];

    // go in reverse
    for i in 0..3 {
        if encoded_letter <= ring_settings[i] {
            encoded_letter = 26 + encoded_letter;
        }
        encoded_letter -= ring_settings[i];
        encoded_letter += rotor_state[i];
        if encoded_letter > 26 {
            encoded_letter -= 26;
        }
        // because we are going in reverse, we have to find what letter would have produced the current one
        encoded_letter = rotors[rotor_choice[i]].iter().position(|&r| r == encoded_letter).unwrap();
        if encoded_letter <= rotor_state[i] {
            encoded_letter = 26 + encoded_letter;
        }
        encoded_letter -= rotor_state[i];
        encoded_letter += ring_settings[i];
        if encoded_letter > 26 {
            encoded_letter -= 26;
        }
    }
    return switchboard[encoded_letter];
}

fn step_rotors(rotor_state: &mut [usize; 3], rotor_choice: [usize; 3], knock_letter: [usize; 8]) {

    // rightmost rotor always steps first
    rotor_state[2] += 1;
    if rotor_state[2] > 25 {
        rotor_state[2] = 0;
    }

    // double stepping: if the middle rotor is at its notch or is just before its notch,
    // it will cause both itself and the leftmost rotor to step
    // also account for rotors VI, VII and VIII
    if (rotor_state[1] + 1) % 26 == knock_letter[rotor_choice[1]]  ||
        (knock_letter[rotor_choice[1]] == 13 && rotor_state[1] == 25){
        rotor_state[0] += 1;
        if rotor_state[0] > 25 {
            rotor_state[0] = 0;
        }
        rotor_state[1] += 1;
        if rotor_state[1] > 25 {
            rotor_state[1] = 0;
        }
    } else if rotor_state[2] == knock_letter[rotor_choice[2]] ||
        (knock_letter[rotor_choice[2]] == 13 && rotor_state[2] == 0){
        rotor_state[1] += 1;
        if rotor_state[1] > 25 {
            rotor_state[1] = 0;
        }
    }
}

fn main() {

    // rotor settings
    let roman_numerals: [&str; 8] = ["I", "II", "III", "IV", "V", "VI", "VII", "VIII"];
    let mut switchboard: [usize; 27] = [0; 27];
    for i in 0..27 {
        switchboard[i] = i as usize;
    }
    let mut rotor_state: [usize; 3] = [0; 3];
    let mut ring_settings: [usize; 3] = [0; 3];
    let mut rotor_choice: [usize; 3] = [8; 3];
    let knock_letter: [usize; 8] = [17, 5, 22, 10, 0, 13, 13, 13];

    // reading from file
    let contents: String = fs::read_to_string("./src/input.txt")
        .expect("Cannot read file");
    let lines: Vec<&str> = contents.lines().collect();

    // choosing rotor
    let rotor_input: Vec<&str> = lines[0].split_whitespace().collect();
    if rotor_input.len() != 3  {
        println!("This program only supports 3 rotors");
        return;
    }
    for i in 0..3 {
        for j in 0..8 {
            if rotor_input[i].eq(roman_numerals[j]) {
                rotor_choice[i] = j;
                break;
            }
        }
        if rotor_choice[i] == 8 {
            println!("Invalid rotor choice");
            return;
        }
    }

    // setting up rotor state
    let state_input: Vec<&str> = lines[1].split_whitespace().collect();
    if state_input.len() != 3  {
        println!("This program only supports 3 rotors");
        return;
    }
    for i in 0..3 {
        rotor_state[i] = match state_input[i].parse::<usize>() {
            Ok(mut t) => {
                if t > 26 || t < 1 {
                    t = 28;
                }
                t - 1
            },
            Err(_) => {27},
        };
        if rotor_state[i] == 27 {
            rotor_state[i] = match state_input[i].parse::<char>() {
                Ok(t) => {
                    let mut letter: usize = t as usize;
                    if letter > 64 && letter < 91 {
                        letter = letter - 65;
                    } else if letter > 96 && letter < 123 {
                        letter = letter - 97;
                    } else {
                        letter = 27;
                    }
                    letter
                },
                Err(_) => {27},
            }
        }
        if rotor_state[i] == 27 {
            println!("invalid rotor state");
            return;
        }
    }

    // setting up rotor rings
    let ring_input: Vec<&str> = lines[2].split_whitespace().collect();
    if ring_input.len() != 3  {
        println!("This program only supports 3 rotors");
        return;
    }
    for i in 0..3 {
        ring_settings[i] = match ring_input[i].parse::<usize>() {
            Ok(mut t) => {
                if t > 26 || t < 1 {
                    t = 28;
                }
                t - 1
            },
            Err(_) => {27},
        };
        if ring_settings[i] == 27 {
            ring_settings[i] = match ring_input[i].parse::<char>() {
                Ok(t) => {
                    let mut letter: usize = t as usize;
                    if letter > 64 && letter < 91 {
                        letter = letter - 65;
                    } else if letter > 96 && letter < 123 {
                        letter = letter - 97;
                    } else {
                        letter = 27;
                    }
                    letter
                },
                Err(_) => {27},
            }
        }
        if ring_settings[i] == 27 {
            println!("invalid rotor ring settings");
            return;
        }
    }


    // setting up plugboard
    let plugboard_input: Vec<&str> = lines[3].split_whitespace().collect();
    for switch in plugboard_input {
        if switch.len() != 2 {
            println!("Invalid plugboard");
            return;
        }
        let letters: Chars = switch.chars();
        for letter in letters.clone() {
            if !letter.is_alphabetic() {
                println!("Invalid plugboard");
                return;
            }
        }
        let plugs: Vec<char> = letters.collect();
        let mut plug1: usize = plugs[0] as usize - 64;
        let mut plug2: usize = plugs[1] as usize - 64;
        if plug1 > 26 {
            plug1 -= 32;
        }
        if plug2 > 26 {
            plug2 -= 32;
        }
        if switchboard[plug1] != plug1 {
            println!("Invalid plugboard");
            return;
        }
        if switchboard[plug2] != plug2 {
            println!("Invalid plugboard");
            return;
        }
        switchboard[plug1] = plug2;
        switchboard[plug2] = plug1;
    }


    let input = lines[4].trim();
    let letters: Vec<_> = input.chars()
                    .filter(|c| c.is_alphabetic())
                    .map(|c| c.to_uppercase().next().unwrap())
                    .map(|c| c as usize - 'A' as usize + 1)
                    .collect();

    let mut result_string: String = "".to_string();
    // start of processing
    if letters.len() == 0 {
        println!("No letters found!")
    } else {
        for letter in letters {

            // first the rotors do a step
            step_rotors(&mut rotor_state, rotor_choice, knock_letter);
            let new_letter: u8 = code_letter(rotor_choice, rotor_state, switchboard, ring_settings, letter).try_into().unwrap();

            // make output nicer to look at
            if result_string.len() % 100 == 99 {
                result_string.push('\n');
            }
            result_string.push((new_letter + 64) as char);
        }
        fs::write("./src/output.txt", result_string).expect("Unable to write file");
        println!("\nAll good chief!")
    }
}
