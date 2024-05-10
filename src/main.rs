use std::{collections::HashMap, io};

const WORDS: &str = include_str!("..\\wordle_words.txt");

fn main() {
    let mut solutions: Vec<&str> = Vec::new();

    println!("Wordle-Solver:\nCorrect characters: UPPERCASE\nKnown characters: lowercase\nUnknwon characters: number\ne.g. Bread -> B0aeD");

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");
    let input: String = input.trim().to_string();

    println!("Characters that are NOT in the word:\ne.g. 'sfhgjx'");

    let mut wrong: String = String::new();
    io::stdin()
        .read_line(&mut wrong)
        .expect("Failed to read input!");
    let wrong: Vec<char> = wrong.trim().to_string().chars().collect();

    let mut correct: Vec<(char, usize)> = Vec::new();
    let mut known: Vec<(char, usize)> = Vec::new();

    let mut parsed_chars: [usize; 5] = [0, 0, 0, 0, 0];
    for ch in input.chars() {
        if ch.to_ascii_uppercase() == ch && !ch.is_numeric() {
            correct.push((ch.to_ascii_lowercase(), input.find(ch).unwrap()));
        } else if ch.is_alphabetic() {
            let (ch_index, r_ch_index): (usize, usize) =
                (input.find(ch).unwrap(), input.rfind(ch).unwrap());

            if ch_index != r_ch_index && parsed_chars[ch_index] != 1 {
                known.push((ch, ch_index));
                known.push((ch, r_ch_index));
                parsed_chars[ch_index] = 1;
                parsed_chars[r_ch_index] = 1;
            } else if parsed_chars[ch_index] != 1 {
                known.push((ch, ch_index));
                parsed_chars[ch_index] = 1;
            }
        }
    }
    let c_len = correct.len();

    let mut k_len: usize = 0;
    for _ in &known {
        k_len += 1;
    }

    let mut map: HashMap<&str, usize> = HashMap::new();

    // TEST
    // for _ in 0..5 {
    //     todo!();
    // }
    // TEST

    for (ch, i) in &correct {
        'i2: for word in WORDS.split(",") {
            if word.as_bytes()[*i] as char == *ch {
                let c = map.entry(word).or_insert(0);

                if !word.contains(*ch)
                    || ((word.find(*ch) != Some(*i)) && word.rfind(*ch) != Some(*i))
                {
                    map.remove(word).unwrap();
                    continue 'i2;
                }

                // IF THE CHARACTER IS WHERE A KNOWN CHARACTER (WRONG POSITION) IS, REMOVE THE WORD
                for (k_ch, k_i) in &known {
                    if !word.contains(*k_ch)
                        || word.find(*k_ch) == Some(*k_i)
                        || word.rfind(*k_ch) == Some(*k_i)
                    {
                        map.remove(word).unwrap();
                        continue 'i2;
                    }
                }

                // IF ALL CHARACTERS ARE KNOWN
                if k_len + c_len >= 5 {
                    let mut same_ch_check: [bool; 5] = [false, false, false, false, false];
                    for (_ch, i) in &correct {
                        same_ch_check[*i] = true;
                    }
                    for ch_index in 0..word.len() {
                        for (ch, _i) in &known {
                            if word.as_bytes()[ch_index] as char == *ch {
                                same_ch_check[ch_index] = true;
                            }
                        }
                    }
                    if same_ch_check != [true, true, true, true, true] {
                        map.remove(word).unwrap();
                        continue 'i2;
                    }
                }

                // IF THE CHARACTER BELONGS THE THE USER-SPECEFIED WRONG CHARACTERS, REMOVE WORD
                for w_ch in &wrong {
                    let in_correct = {
                        let mut x: bool = false;
                        'i4: for (ch, _i) in &correct {
                            if ch == w_ch {
                                x = true;
                                break 'i4;
                            }
                        }
                        x
                    };

                    if in_correct && word.find(*w_ch) != word.rfind(*w_ch) {
                        map.remove(word).unwrap();
                        continue 'i2;
                    }

                    if word.contains(*w_ch) && !in_correct {
                        map.remove(word).unwrap();
                        continue 'i2;
                    } else if word.contains(*w_ch) && in_correct {
                        *c += 1;
                        continue 'i2;
                    }
                }

                *c += 1;
            }
        }
    }
    if correct.is_empty() {
        for (ch, i) in &known {
            'i2: for word in WORDS.split(",") {
                let c = map.entry(word).or_insert(0);
                if !word.contains(*ch)
                    || (word.find(*ch) != Some(*i) && word.rfind(*ch) == Some(*i))
                    || (word.find(*ch) == Some(*i) && word.rfind(*ch) != Some(*i))
                {
                    map.remove(word).unwrap();
                    continue 'i2;
                }

                let mut dup_ch_count: usize = 0;
                let mut dup_k_ch_count: usize = 0;
                for c_ch in word.chars() {
                    if c_ch == *ch {
                        dup_ch_count += 1;
                    }
                }
                for (ch1, _i) in &known {
                    for (ch2, _i) in &known {
                        if ch1 == ch2 {
                            dup_k_ch_count += 1;
                        }
                    }
                }
                dup_k_ch_count /= 2;

                println!("K dup count: {dup_k_ch_count}");
                if dup_ch_count < dup_k_ch_count {
                    map.remove(word).unwrap();
                    continue 'i2;
                }

                let mut same_ch_check: [bool; 5] = [false, false, false, false, false];
                for ch_index in 0..word.len() {
                    if word.as_bytes()[ch_index] as char == *ch {
                        same_ch_check[ch_index] = true;
                    }
                }
                if !same_ch_check.contains(&true) {
                    map.remove(word).unwrap();
                    continue 'i2;
                }

                for w_ch in &wrong {
                    if word.contains(*w_ch) {
                        map.remove(word).unwrap();
                        continue 'i2;
                    }
                }

                *c += 1;
            }
        }
    }
    if known.is_empty() && correct.is_empty() {
        'o: for word in WORDS.split(",") {
            let c = map.entry(word).or_insert(0);

            for w_ch in &wrong {
                if word.contains(*w_ch) {
                    map.remove(word).unwrap();
                    continue 'o;
                }
            }

            *c += 1;
        }
    }

    let mut max_val: usize = 0;
    for value in map.values() {
        if value > &max_val {
            max_val = *value;
        }
    }

    for (key, value) in map.iter() {
        if value >= &max_val {
            solutions.push(key);
        }
    }

    println!("Solutions:\n{:#?}", solutions);
    println!("Entries: {}", solutions.len());
}

fn _check_word<'a>(
    wrong: Vec<char>,
    known: Vec<(char, usize)>,
    correct: Vec<(char, usize)>,
    map: &mut HashMap<&'a str, usize>,
    loop_field: Vec<(char, usize)>,
) -> HashMap<&'a str, usize> {
    for (ch, i) in &loop_field {
        'next: for word in WORDS.split(",") {
            let c = map.entry(word).or_insert(0);

            if !word.contains(*ch) || word.find(*ch) == Some(*i) || word.rfind(*ch) == Some(*i) {
                map.remove(word).unwrap();
                continue 'next;
            }

            let mut same_ch_check: [bool; 5] = [false, false, false, false, false];
            for ch_index in 0..word.len() {
                if word.as_bytes()[ch_index] as char == *ch {
                    same_ch_check[ch_index] = true;
                }
            }
            if correct.is_empty() && !same_ch_check.contains(&true)
                || (correct.len() + known.len() >= 5
                    && same_ch_check != [true, true, true, true, true])
            {
                map.remove(word).unwrap();
                continue 'next;
            }

            for w_ch in &wrong {
                if word.contains(*w_ch) {
                    map.remove(word).unwrap();
                    continue 'next;
                }
            }

            *c += 1;
        }
    }
    map.to_owned()
}
