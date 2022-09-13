use clap::Parser;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::io;

mod goalwords;
mod morewords;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// offset to count
    #[clap(short, long, default_value_t = 0)]
    count: usize,
}

#[allow(dead_code)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[allow(dead_code)]
fn count_chars(_column: usize) {
    let counted = goalwords::GOALWORDS
        .iter()
        //.skip(column)
        //.step_by(5)
        .flat_map(|w| w.chars())
        .filter(|c| c.is_ascii_lowercase())
        .fold(HashMap::with_capacity(26), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

    let mut count_vec = counted.iter().collect::<Vec<(&char, &i32)>>();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));
    count_vec.iter().for_each(|(c, x)| println!("{}:{}", c, x));
}

// Compare two words, and return how good the guess is relative to the goal.
// Output is a string of five letters
// ' ' means the guess is not in the goal word
// 'Y' means the guess letter is in the goal word, but not in the right location.
// 'G' means the guess letter is in the right location in the goal word.
fn compare_words(goal: &str, guess: &str) -> String {
    let mut s: [char; 5] = [' ', ' ', ' ', ' ', ' '];

    let mut goal_chars: Vec<char> = goal.chars().collect();
    let guess_chars: Vec<char> = guess.chars().collect();

    // First pass. Mark the correct letters with a 'G'
    for i in 0..5 {
        if goal_chars[i] == guess_chars[i] {
            s[i] = 'G';
            goal_chars[i] = ' ' // Clear out this character so we don't match it again.
        }
    }

    // Second pass... Mark the guess letters that exist in the goal word but not in the right spot
    // as 'Y'
    for i in 0..5 {
        if s[i] == ' ' {
            let found = goal_chars.iter().enumerate().find_map(|(j, c)| {
                if *c == guess_chars[i] {
                    Some(j)
                } else {
                    None
                }
            });
            if let Some(j) = found {
                s[i] = 'Y';
                goal_chars[j] = ' ';
            }
        }
    }

    s.iter().collect()
}

fn score(word_set: &HashSet<&str>) -> String {
    let word_set_count: f64 = word_set.len() as f64;
    println!("Word set count: {}", word_set_count);

    let mut max_score: f64 = 0.0;
    let mut max_fscore: f64 = 0.0;
    let mut max_word = String::from("");
    let min_of_max: usize = 10000;

    let all_words = goalwords::GOALWORDS
        .iter()
        .chain(morewords::MOREWORDS.iter());
    //let all_words = word_set.iter();
    for possible_guess in all_words {
        // Calculate the clue sets and their size.
        let counted = &word_set
            .iter()
            .fold(HashMap::new(), |mut acc, possible_goal| {
                *acc.entry(compare_words(possible_goal, possible_guess))
                    .or_insert(0) += 1;
                acc
            });

        // Given the clue set, Calculate the Shannon entropy.
        let fscore: f64 = counted
            .iter()
            .map(|(_key, value)| {
                let v_c: f64 = f64::from(*value as i32);
                let f = word_set_count / v_c;
                v_c * f.ln()
            })
            .sum::<f64>()
            / word_set_count;

        // Given a clue set, calculate it's size
        let score: f64 = counted.len() as f64;

        // Given a cluse set, calculate the maximum clue size.
        let _max_clue_size = counted
            .iter()
            .map(|(_key, value)| value)
            .max()
            .ok_or(Some(0))
            .unwrap();

        /*
                if *max_clue_size < min_of_max {
                    min_of_max = *max_clue_size;
                    max_fscore = fscore;
                    max_word = possible_guess.to_string();
                } else if *max_clue_size <= min_of_max && word_set.contains(possible_guess){
                    max_fscore = fscore;
                    min_of_max = *max_clue_size;
                    max_word = possible_guess.to_string();
                } else if *max_clue_size == min_of_max &&
                          fscore > max_fscore {

                    max_fscore = fscore;
                    min_of_max = *max_clue_size;
                    max_word = possible_guess.to_string();
                }

    */
        /*
        if fscore > max_fscore
            || (fscore == max_fscore && min_of_max > *max_clue_size)
            || (fscore == max_fscore
                && min_of_max == *max_clue_size
                && word_set.contains(possible_guess))
        {
            min_of_max = *max_clue_size;
            max_word = possible_guess.to_string();
            max_score = score;
            max_fscore = fscore;
        }
        */

        /*
        if score > max_score ||
           (score == max_score && fscore > max_fscore ) ||
           (score == max_score && fscore == max_fscore && word_set.contains(possible_guess)) {
            max_word = possible_guess.to_string();
            max_score = score;
            max_fscore = fscore;
        }
        */

  /* dd
        if score > max_score ||
           (score == max_score &&  min_of_max > *max_clue_size ) ||
           (score == max_score && min_of_max == *max_clue_size && word_set.contains(possible_guess)) {
            max_word = possible_guess.to_string();
            max_score = score;
            max_fscore = fscore;
            min_of_max = *max_clue_size;
        }

*/
        if fscore == max_fscore && score > max_score {
             max_word = possible_guess.to_string();
             max_score = score;
             max_fscore = fscore;
         } else if fscore == max_fscore && score == max_score && word_set.contains(possible_guess) {
             // prefer possible solutions
             max_score = score;
             max_fscore = fscore;
             max_word = possible_guess.to_string();
         } else if fscore > max_fscore {
             max_score = score;
             max_fscore = fscore;
             max_word = possible_guess.to_string();
         }
    }

    println!(
        "Guess... {}, {} {} {}",
        max_word, max_score, max_fscore, min_of_max
    );
    max_word
}

fn remove(word_set: &mut HashSet<&str>, guess: &str, clue: &str) {
    let guess_chars: Vec<char> = guess.chars().collect();
    let clue_chars: Vec<char> = clue.chars().collect();
    let mut remove_set = HashSet::new();

    for word in word_set.iter() {
        let mut word_chars: Vec<char> = word.chars().collect();
        let mut remove = false;

        for i in 0..5 {
            if clue_chars[i] == 'G' {
                if guess_chars[i] == word_chars[i] {
                    word_chars[i] = ' '; // Don't match this letter again
                } else {
                    // Remove words where the clue is green, but the letters don't match
                    remove = true;
                }
            }

            if remove {
                break;
            }
        }

        if !remove {
            for i in 0..5 {
                if clue_chars[i] == 'Y' {
                    // If the clue is Y then search for that letter.
                    // For Y, valid matches only happen when the match is not in the same position.
                    let found = word_chars.iter().enumerate().find_map(|(j, c)| {
                        if *c == guess_chars[i] {
                            Some(j)
                        } else {
                            None
                        }
                    });

                    if let Some(j) = found {
                        if j != i {
                            word_chars[j] = ' '; // Don't match this letter again.
                        } else {
                            remove = true; // This clue should have been 'G'
                        }
                    } else {
                        remove = true; // Didn't find the matching letter.
                    }
                }

                if remove {
                    break;
                }
            }
        }

        if !remove {
            for i in 0..5 {
                if clue_chars[i] == ' ' {
                    // If the clue is ' ' then that guess letter must not exist in the target.
                    let found = word_chars.iter().enumerate().find_map(|(j, c)| {
                        if *c == guess_chars[i] {
                            Some(j)
                        } else {
                            None
                        }
                    });

                    if let Some(_j) = found {
                        remove = true;
                    }

                    if remove {
                        break;
                    }
                }
            }
        }

        if remove {
            remove_set.insert(word.clone());
        }
    }

    for removeable in &remove_set {
        word_set.remove(removeable);
    }
}

fn play_wordle() {
    let mut word_set = goalwords::GOALWORDS
        .iter()
        .fold(HashSet::<&str>::new(), |mut acc, word| {
            acc.insert(word);
            acc
        });

    while !word_set.is_empty() {
        let recommend = score(&word_set);

        let mut clue = String::new();

        println!("Enter clue...");
        io::stdin()
            .read_line(&mut clue)
            .expect("Failed to read line");

        remove(&mut word_set, &recommend, &clue);

        let count = word_set.len();
        println!("{} possible words", count);

        if count < 300 {
            for word in &word_set {
                print!("{} ", word);
            }
            println!(" ");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    //let args = Args::parse();

    //if args.count>0 && args.count<6 {
    //    count_chars( args.count )
    //}
    //println!("total up column {0}", args.count );

    //count_chars(args.count);
    //    let counted = goalwords::GOALWORDS
    //                    .iter()
    //                    .fold( HashMap::new(),
    //                          |mut acc, goal| {*acc.entry(compare_words(goal,"trace")).or_insert(0) += 1; acc } );

    //    let max = counted.iter().map(|(key,value)| value).max();

    //    for (key,value) in counted.iter() {

    //        println!("{} {}", key, value);
    //    }

    //println!("{}, {}, {}", guess, counted.len(),max.unwrap());

    if args[1] == String::from("map") {
        println!("{}, {}, {}, {}", "Words", "sets", "complex", "max");
        for guess in goalwords::GOALWORDS {
            let counted = goalwords::GOALWORDS
                .iter()
                .fold(HashMap::new(), |mut acc, goal| {
                    *acc.entry(compare_words(goal, guess)).or_insert(0) += 1;
                    acc
                });

            let max = counted.iter().map(|(_key, value)| value).max();
            let set_count = counted.iter().fold(
                0,
                |acc, (_key, value)| if *value > 1 { acc + 1 } else { acc },
            );

            println!(
                "{}, {}, {}, {}",
                guess,
                counted.len(),
                set_count,
                max.unwrap()
            );
        }
    }

    if args[1] == String::from("log") {
        println!("{}, {}", "Words", "score");
        let all_words = goalwords::GOALWORDS
            .iter()
            .chain(morewords::MOREWORDS.iter());
        let _all_words_count: f64 = all_words.count() as f64;

        let all_words = goalwords::GOALWORDS
            .iter()
            .chain(morewords::MOREWORDS.iter());

        let goal_words = HashSet::from(["crack", "crazy", "cramp"]);
        let goal_words_count: f64 = goal_words.iter().count() as f64;
        for guess in all_words {
            let counted = goal_words.iter().fold(HashMap::new(), |mut acc, goal| {
                *acc.entry(compare_words(goal, guess)).or_insert(0) += 1;
                acc
            });

            // Calculate the Shannon entropy of this guess word.
            let score: f64 = counted
                .iter()
                .map(|(_key, value)| {
                    let v_c: f64 = f64::from(*value);
                    let f = v_c / goal_words_count;
                    v_c * f.ln()
                })
                .sum::<f64>()
                / goal_words_count;

            println!("{}, {}", guess, -score);
        }
    }

    if args[1] == String::from("play") {
        play_wordle();
    }

    if args[1] == String::from("deep") {
        let mut all = HashMap::new();
        let all_words = goalwords::GOALWORDS
            .iter()
            .chain(morewords::MOREWORDS.iter());
        for guess1 in all_words {
            println!("{}", guess1);
            let all_words2 = goalwords::GOALWORDS
                .iter()
                .chain(morewords::MOREWORDS.iter());
            for guess2 in all_words2 {
                let mut counted = HashMap::new();
                for goal in goalwords::GOALWORDS {
                    let filter1 = compare_words(goal, guess1);
                    let filter2 = compare_words(goal, guess2);
                    let mut filter = String::new();
                    filter.push_str(&filter1);
                    filter.push_str(&filter2);
                    //println!("{}", filter);
                    *counted.entry(filter).or_insert(0) += 1;
                }

                let mut wordpair = String::new();
                wordpair.push_str(&guess1);
                wordpair.push_str(&guess2);
                if counted.len() > 1000 {
                    println!("{} {}", wordpair, counted.len());
                    all.insert(wordpair, counted.len());
                }
            }
        }

        let mut count_vec = all.iter().collect::<Vec<(&String, &usize)>>();
        count_vec.sort_by(|(_, i1), (_, i2)| i1.cmp(i2));
        count_vec.iter().for_each(|(s, i)| println!("{}:{}", s, i));
    }

    if args[1] == String::from("scan") {
        let first_word = String::from(&args[2]);

        let mut counted = HashMap::new();

        for goal in goalwords::GOALWORDS {
            let filter = compare_words(goal, &first_word);
            *counted.entry(filter).or_insert(0) += 1;
        }

        let mut count_vec = counted.iter().collect::<Vec<(&String, &usize)>>();
        count_vec.sort_by(|(_, i1), (_, i2)| i1.cmp(i2));
        count_vec.iter().for_each(|(s, i)| println!("{}:{}", s, i));
    }
}

#[test]
fn it_works() {
    let s = compare_words("stern", "sueat");
    assert_eq!(s, String::from("G G Y"));

    let s = compare_words("stern", "clamp");
    assert_eq!(s, String::from("     "));

    let s = compare_words("stern", "stern");
    assert_eq!(s, String::from("GGGGG"));

    let s = compare_words("stern", "clamp");
    assert_eq!(s, String::from("     "));

    let s = compare_words("abcde", "edcba");
    assert_eq!(s, String::from("YYGYY"));
}

#[test]
fn remote_test() {
    let mut a = HashSet::from(["abcde", "abcdf", "abcdg"]);

    for word in a.iter() {
        println!("in {}", word.clone());
    }

    remove(&mut a, "iiiig", "    G");

    for word in a.iter() {
        println!("after {}", word.clone());
    }

    assert_eq!(a.contains("abcdg"), true);
    assert_eq!(a.contains("abcdf"), false);
}

#[test]
fn y_test() {
    let mut a = HashSet::from(["abcde", "fbcdf", "abcdg"]);

    remove(&mut a, "iifii", "  Y  ");

    assert_eq!(a.contains("fbcdf"), true);
    assert_eq!(a.contains("abcdg"), false);
}

#[test]
fn gy_test() {
    let mut a = HashSet::from(["abcae", "fbcdf", "abadg", "aiiba"]);

    remove(&mut a, "aiiia", "G   Y");

    assert_eq!(a.contains("fbcdf"), false);
    assert_eq!(a.contains("abcae"), true);
    assert_eq!(a.contains("abadg"), true);
    assert_eq!(a.contains("aiiba"), false);
}

#[test]
fn ggg_test() {
    let mut a = HashSet::from(["crack", "cramp"]);

    remove(&mut a, "crack", "GGG  ");

    assert_eq!(a.contains("crack"), false);
    assert_eq!(a.contains("cramp"), true);
}
