use std::{collections::HashMap, io};

const WORDS: &str = include_str!("../wordle_words.txt");

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
    let wrong_chars: Vec<char> = wrong.trim().to_string().chars().collect();
    let mut x: Vec<(char, usize)> = Vec::new();
    for w_ch in wrong_chars {
        x.push((w_ch, 0));
    }
    let wrong: Vec<(char, usize)> = x;

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

    let mut map: HashMap<&str, usize> = HashMap::new();

    if !correct.is_empty() {
        map = check_word(&wrong, &known, &correct, &mut map, &correct);
    } else if correct.is_empty() && !known.is_empty() {
        map = check_word(&wrong, &known, &correct, &mut map, &known);
    } else {
        map = check_word(&wrong, &known, &correct, &mut map, &wrong)
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

fn check_word<'a>(
    wrong: &Vec<(char, usize)>,
    known: &Vec<(char, usize)>,
    correct: &Vec<(char, usize)>,
    map: &mut HashMap<&'a str, usize>,
    loop_field: &Vec<(char, usize)>,
) -> HashMap<&'a str, usize> {
    for (ch, i) in loop_field {
        'next: for word in WORDS.split(",") {
            let c = map.entry(word).or_insert(0);

            // SOME IN CORRECT
            if !correct.is_empty() && word.as_bytes()[*i] as char == *ch {
                // CORRECT
                if !word.contains(*ch)
                    || (word.find(*ch) != Some(*i) && word.rfind(*ch) != Some(*i))
                {
                    map.remove(word).unwrap();
                    continue 'next;
                }

                // KNOWN
                for (k_ch, k_i) in known {
                    if !word.contains(*k_ch)
                        || word.find(*k_ch) == Some(*k_i)
                        || word.rfind(*k_ch) == Some(*k_i)
                    {
                        map.remove(word).unwrap();
                        continue 'next;
                    }
                }

                // ALL KNOWN
                if correct.len() + known.len() >= 5 {
                    let mut check: [bool; 5] = [false, false, false, false, false];
                    for (_, c_i) in correct {
                        check[*c_i] = true;
                    }
                    for ch_index in 0..word.len() {
                        for (k_ch, _) in known {
                            if word.as_bytes()[ch_index] as char == *k_ch {
                                check[ch_index] = true;
                            }
                        }
                    }
                    if check != [true, true, true, true, true] {
                        map.remove(word).unwrap();
                        continue 'next;
                    }
                }

                // WRONG
                for (w_ch, _) in wrong {
                    let in_correct = {
                        let mut x: bool = false;
                        'check: for (ch, _i) in correct {
                            if ch == w_ch {
                                x = true;
                                break 'check;
                            }
                        }
                        x
                    };

                    if (in_correct && word.find(*w_ch) != word.rfind(*w_ch))
                        || (word.contains(*w_ch) && !in_correct)
                    {
                        map.remove(word).unwrap();
                        continue 'next;
                    } else if word.contains(*w_ch) && in_correct {
                        // *c += 1;
                        continue 'next;
                    }
                }
                *c += 1;

            // ONLY KNOWN
            } else if correct.is_empty() && !known.is_empty() {
                if !word.contains(*ch)
                    || (word.find(*ch) != Some(*i) && word.rfind(*ch) == Some(*i))
                    || (word.find(*ch) == Some(*i) && word.rfind(*ch) != Some(*i))
                    || word.find(*ch) == Some(*i)
                {
                    map.remove(word).unwrap();
                    continue 'next;
                }

                let (mut d_ch_c, mut d_k_ch_c): (usize, usize) = (0, 0);
                for word_ch in word.chars() {
                    if word_ch == *ch {
                        d_ch_c += 1;
                    }
                }
                for (k_ch1, _) in known {
                    for (k_ch2, _) in known {
                        if k_ch1 == k_ch2 {
                            d_k_ch_c += 1;
                        }
                    }
                }
                d_k_ch_c /= 2;

                if d_ch_c < d_k_ch_c {
                    map.remove(word).unwrap();
                    continue 'next;
                }

                let mut check: [bool; 5] = [false, false, false, false, false];
                for ch_index in 0..word.len() {
                    if word.as_bytes()[ch_index] as char == *ch {
                        check[ch_index] = true;
                    }
                }
                if !check.contains(&true) {
                    map.remove(word).unwrap();
                    continue 'next;
                }

                for (w_ch, _) in wrong {
                    if word.contains(*w_ch) {
                        map.remove(word).unwrap();
                        continue 'next;
                    }
                }

                *c += 1;
            }
        }
    }
    // NOTHING
    if correct.is_empty() && known.is_empty() {
        'next: for word in WORDS.split(",") {
            let c = map.entry(word).or_insert(0);
            for (w_ch, _) in wrong {
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
