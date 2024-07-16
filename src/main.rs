use std::io::stdin;

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
    let mut switchboard: [usize; 27] = [0; 27];
    for i in 0..27 {
        switchboard[i] = i as usize;
    }
    let mut rotor_state: [usize; 3] = [2, 3, 4];
    let ring_settings: [usize; 3] = [9, 1, 3];
    let rotor_choice: [usize; 3] = [0, 1, 7];
    let knock_letter: [usize; 8] = [17, 5, 22, 10, 0, 13, 13, 13];

    // read input
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    let letters: Vec<_> = input.chars()
                    .filter(|c| c.is_alphabetic())
                    .map(|c| c.to_uppercase().next().unwrap())
                    .map(|c| c as usize - 'A' as usize + 1)
                    .collect();
    if letters.len() == 0 {
        println!("No letters found!")
    } else {
        for letter in letters {

            // first the rotors do a step
            step_rotors(&mut rotor_state, rotor_choice, knock_letter);
            let new_letter: u8 = code_letter(rotor_choice, rotor_state, switchboard, ring_settings, letter).try_into().unwrap();
            print!("{}", (new_letter + 64) as char);
        }
        println!("\nAll good chief!")
    }
}
