mod words;

// this testing branch is going to be used to try to get this code a bit less spaghetti'd

// words.rs simply moves the creation of the word list vector into a separate file
// for the sake of code readability.

use clap::{App, Arg};
use fancy_regex::Regex;

#[macro_use(concat_string)]
extern crate concat_string;

fn main() {
    let matches = App::new("churlish")
        .version("0.1")
        .about(
            "a simple regexp wordle guesser

Use uppercase letters for correct letters (green), lowercase letters for incorrectly placed 
letters (yellow), and question marks for wrong letters. 

The example pattern C?a?? would be interpreted as 'c' being the first green letter, and 'a' 
being in the word in a different spot (yellow letter). 

Note: A guess with no correct letters of any kind would be represented as ????? -- this 
will cause most shells to match a filename of length 5 in the directory where churlish
is located. This can be fixed by including single quotes around your pattern, so use
'?????' instead of ?????.

Incorrect letters must be added to the --wrong (-w) argument.
        ",
        )
        .author("by https://github.com/ifben")
        .arg(
            Arg::with_name("first pattern")
                .long("first")
                .takes_value(true)
                .short('1')
                .help("the result of your first wordle guess")
                .required(true),
        )
        .arg(
            Arg::with_name("second pattern")
                .long("second")
                .takes_value(true)
                .short('2')
                .help("the result of your second wordle guess")
                .required(false),
        )
        .arg(
            Arg::with_name("third pattern")
                .long("third")
                .takes_value(true)
                .short('3')
                .help("the result of your third wordle guess")
                .required(false),
        )
        .arg(
            Arg::with_name("fourth pattern")
                .long("fourth")
                .takes_value(true)
                .short('4')
                .help("the result of your fourth wordle guess")
                .required(false),
        )
        .arg(
            Arg::with_name("wrong letters")
                .long("wrong")
                .takes_value(true)
                .short('w')
                .help("all the wrong answers across all guesses")
                .required(false),
        )
        .get_matches();

    let words = words::wordle_list();

    // Now the tedious input checking...

    let first = matches.value_of("first pattern");
    let mut pattern = String::new();

    if let Some(_i) = first {
        if first.unwrap().len() != 5 {
            println!("Pattern length error in first wordle guess.");
            std::process::exit(0);
        } else {
            pattern = first.unwrap().to_string();
        }
    }

    let second = matches.value_of("second pattern");

    if let Some(_i) = second {
        if second.unwrap().len() != 5 {
            println!("Pattern length error in second wordle guess.");
            std::process::exit(0);
        } else {
            pattern = concat_string!(pattern, second.unwrap());
        }
    }

    let third = matches.value_of("third pattern");

    if let Some(_i) = third {
        if third.unwrap().len() != 5 {
            println!("Pattern length error in third wordle guess.");
            std::process::exit(0);
        } else {
            pattern = concat_string!(pattern, third.unwrap());
        }
    }

    let fourth = matches.value_of("fourth pattern");

    if let Some(_i) = fourth {
        if fourth.unwrap().len() != 5 {
            println!("Pattern length error in fourth wordle guess.");
            std::process::exit(0);
        } else {
            pattern = concat_string!(pattern, fourth.unwrap());
        }
    }
    
    let mut wrong = matches.value_of("wrong letters");

    // this is a dumb hack to cover an extreme edge case, but
    // if you were to guess an anagram on your first guess,
    // you would have no wrong letters to input, meaning we have
    // to keep --wrong (-w) as an optional entry. to handle there
    // being a possible None in this option, we just insert "0"
    // as a value, since it'll never match in the word list anyway

    let no_wrongs = String::from("0");

    wrong.get_or_insert_with(|| &no_wrongs);

    // at this point, we have collected all of the inputted wordle patterns into
    // a single string, which we then make a vector of chars from. we don't need
    // right_letters for a while, but we need to build it before sending pattern
    // off to dupe_check()

    let chars: Vec<_> = pattern.chars().collect();
    let right_letters: Vec<_> = pattern.to_lowercase().chars().collect();

    // dupe_check allows us to check for patterns that have duplicate letters
    // such as the pattern ??Ee? for OBESE

    let dupe = dupe_check(pattern);

    let mut guesses: Vec<String> = vec![
        "?".to_string(),
        "?".to_string(),
        "?".to_string(),
        "?".to_string(),
        "?".to_string(),
        "?".to_string(),
    ];

    // this vector collects the possible answers at each letter place in the word.
    // the 6th position is a collector for duplicate letter results

    for i in 0..chars.len() {
        if chars[i].is_uppercase() {
            // this allows us to figure out what position the char is in the wordle
            // pattern, allowing us to evaluate multiple guesses more easily
            let place = (i + 1) % 5;

            match place {
                // a zero for place means the fifth position in the guess (modulo 0)
                // if any of these match, we know that the given upper-case letter
                // must be in this position
                0 => {
                    if guesses[4] == "?" {
                        guesses[4] = chars[i].to_string();
                    }
                }
                1 => {
                    if guesses[0] == "?" {
                        guesses[0] = chars[i].to_string();
                    }
                }
                2 => {
                    if guesses[1] == "?" {
                        guesses[1] = chars[i].to_string();
                    }
                }
                3 => {
                    if guesses[2] == "?" {
                        guesses[2] = chars[i].to_string();
                    }
                }
                4 => {
                    if guesses[3] == "?" {
                        guesses[3] = chars[i].to_string();
                    }
                }
                _ => println!("Error"),
            }
        }
    }
    for j in 0..chars.len() {
        if chars[j].is_lowercase() {
            // lowercase letters are much more confusing, as we need to check that
            // there isn't already an uppercase letter in that place, check
            // for any duplicates, and we need to eliminate positions correctly
            let n_place = (j + 1) % 5;

            match n_place {
                0 => {
                    if guesses[4] == "?" {
                        // if the value is still the empty '?', we can just plug our
                        // letter in here
                        guesses[4] = chars[j].to_string();
                    } else if guesses[4].chars().nth(0).unwrap().is_lowercase() {
                        // if we know the first letter in the vector is lowercase,
                        // we can safely add other wrong position guesses to the vec
                        if guesses[4] == "?" {
                            guesses[4] = chars[j].to_string();
                        } else if guesses[4].chars().nth(guesses[4].len() - 1).unwrap() != chars[j] {
                            // prevent adding duplicate letters to the wrong position letters
                            // in the vector
                            guesses[4] = concat_string!(guesses[4], chars[j].to_string());
                        }
                    } else if guesses[4].chars().nth(0).unwrap().is_uppercase() {
                        // here is where we try to account for words with duplicate letters
                        // like ABBEY -- if we've matched a letter correctly, but the guess
                        // reveals a letter in the wrong position, we add the letter to our collector
                        // for example, a pattern of ?B??? followed by ?e??? would add an 'e'
                        // to the collector even though we know the second position is only 'b'
                        if guesses[5] == "?" {
                            guesses[5] = chars[j].to_string();
                        } else if guesses[5].chars().nth(guesses[5].len() - 1).unwrap() != chars[j] {
                            guesses[5] = concat_string!(guesses[5], chars[j].to_string());
                        }
                    }
                }
                1 => {
                    if guesses[0] == "?" {
                        guesses[0] = chars[j].to_string();
                    } else if guesses[0].chars().nth(0).unwrap().is_lowercase() {
                        if guesses[0] == "?" {
                            guesses[0] = chars[j].to_string();
                        } else if guesses[0].chars().nth(guesses[0].len() - 1).unwrap() != chars[j] {
                            guesses[0] = concat_string!(guesses[0], chars[j].to_string());
                        }
                    } else if guesses[0].chars().nth(0).unwrap().is_uppercase() {
                        if guesses[5] == "?" {
                            guesses[5] = chars[j].to_string();
                        } else if guesses[5].chars().nth(guesses[5].len() - 1).unwrap() != chars[j] {
                            guesses[5] = concat_string!(guesses[5], chars[j].to_string());
                        }
                    }
                }
                2 => {
                    if guesses[1] == "?" {
                        guesses[1] = chars[j].to_string();
                    } else if guesses[1].chars().nth(0).unwrap().is_lowercase() {
                        if guesses[1] == "?" {
                            guesses[1] = chars[j].to_string();
                        } else if guesses[1].chars().nth(guesses[1].len() - 1).unwrap() != chars[j] {
                            guesses[1] = concat_string!(guesses[1], chars[j].to_string());
                        }
                    } else if guesses[1].chars().nth(0).unwrap().is_uppercase() {
                        if guesses[5] == "?" {
                            guesses[5] = chars[j].to_string();
                        } else if guesses[5].chars().nth(guesses[5].len() - 1).unwrap() != chars[j] {
                            guesses[5] = concat_string!(guesses[5], chars[j].to_string());
                        }
                    }
                }
                3 => {
                    if guesses[2] == "?" {
                        guesses[2] = chars[j].to_string();
                    } else if guesses[2].chars().nth(0).unwrap().is_lowercase() {
                        if guesses[2] == "?" {
                            guesses[2] = chars[j].to_string();
                        } else if guesses[2].chars().nth(guesses[2].len() - 1).unwrap() != chars[j] {
                            guesses[2] = concat_string!(guesses[2], chars[j].to_string());
                        }
                    } else if guesses[2].chars().nth(0).unwrap().is_uppercase() {
                        if guesses[5] == "?" {
                            guesses[5] = chars[j].to_string();
                        } else if guesses[5].chars().nth(guesses[5].len() - 1).unwrap() != chars[j] {
                            guesses[5] = concat_string!(guesses[5], chars[j].to_string());
                        }
                    }
                }
                4 => {
                    if guesses[3] == "?" {
                        guesses[3] = chars[j].to_string();
                    } else if guesses[3].chars().nth(0).unwrap().is_lowercase() {
                        if guesses[3] == "?" {
                            guesses[3] = chars[j].to_string();
                        } else if guesses[3].chars().nth(guesses[3].len() - 1).unwrap() != chars[j] {
                            guesses[3] = concat_string!(guesses[3], chars[j].to_string());
                        }
                    } else if guesses[3].chars().nth(0).unwrap().is_uppercase() {
                        if guesses[5] == "?" {
                            guesses[5] = chars[j].to_string();
                        } else if guesses[5].chars().nth(guesses[5].len() - 1).unwrap() != chars[j] {
                            guesses[5] = concat_string!(guesses[5], chars[j].to_string());
                        }
                    }
                }
                _ => println!("Error"),
            }
        }
    }

    // now that we have our vector full of possible guesses, we need to build
    // the regular expression pattern to test against words. this is done kind of
    // messily with string concatenation. basically, we treat incorrectly placed letters
    // as positive lookaheads, we treat correctly placed letters as simply themselves,
    // then we add impossible letters to a [^] for the unmatched letter space


    let mut letter_expression = String::new(); // part of the expression for the 5 characters exactly
    let mut lookaheads_expression = String::new(); // beginning of the regex that collects all lookaheads as an expression
    let mut lookaheads = String::new(); // collector for the positive lookaheads characters

    let mut lookaheads_vec = Vec::new();

    for i in 0..guesses.len() - 1 {
        let val_chars: Vec<_> = guesses[i].chars().collect();
        if val_chars.len() == 1 && val_chars[0].is_uppercase() {
            // if all we have in our guess is 1 uppercase letter,
            // we know it's the right answer
            letter_expression.push_str(&guesses[i].to_lowercase());
        } else if val_chars[0] == '?' {
            // if we have no matching letters, either perfect or misplaced,
            // we know that we only need to input the wrong letters for this guess
            letter_expression.push_str(&concat_string!(String::from("[^"), wrong.unwrap(), String::from("]")));
        } else if val_chars[0].is_lowercase() {
            // if we have any lowercase letters, we can add them all to the lookahead vector
            for p in 0..val_chars.len() {
                lookaheads_vec.push(val_chars[p]);
            }

            lookaheads_vec.sort();
            lookaheads_vec.dedup(); // removes any duplicate lookaheads
            
            letter_expression.push_str(&concat_string!(String::from("[^"), wrong.unwrap(), guesses[i], String::from("]")));
        }
    }

    for p in 0..lookaheads_vec.len() {
        lookaheads.push_str(&concat_string!(String::from("(?=.*"), lookaheads_vec[p].to_string(), String::from(")")));
    }

    lookaheads_expression.push_str(&lookaheads);

    let regex;

    if guesses[5] != "?" {
        // if we have collected values in our extra index, add them to our list of lookaheads and rebuild expression
        let extra_chars: Vec<_> = guesses[5].chars().collect();
        for i in 0..extra_chars.len() {
            lookaheads_vec.push(extra_chars[i]);
        }
        lookaheads_vec.sort();
        lookaheads_vec.dedup();

        // again, remove any duplicate lookaheads

        let mut extra_lookaheads = String::new();
        for p in 0..lookaheads_vec.len() {
            extra_lookaheads = concat_string!(extra_lookaheads, String::from("(?=.*"), lookaheads_vec[p].to_string(), String::from(")"));
        }
        regex = concat_string!(String::from("^"), extra_lookaheads, letter_expression, String::from("$")); // build our final regex string
    } else {
        regex = concat_string!(String::from("^"), lookaheads_expression, letter_expression, String::from("$"));
    }

    let expression = Regex::new(&regex).unwrap();

    let mut j = 1; // just a counter for the results

    for i in 0..words.len() {
        let result = expression.is_match(&words[i]);
        if result.unwrap() != false {
            if dupe[0] != '?' && dupe[2] == '?' {
                // this will tell us that only one duplicate was found
                // so we need to filter for only words with two duplicate letters
                let c = words[i].matches(dupe[0]).count();
                let num = dupe[1] as i32 - 0x30; // terrible type conversion from char to i32 :p
                if c >= num.try_into().unwrap() {
                    println!("{}: {}", j, words[i]);
                    j += 1;
                }
            }
            if dupe[2] != '?' {
                // if dupe[2] has a value, two unique duplicate letters
                // were matched, so we need to filter for both. for example,
                // lal?a will match only ALGAL, since we know we need both
                // letter twice

                let c1 = words[i].matches(dupe[0]).count();
                let c2 = words[i].matches(dupe[2]).count();
                let num1 = dupe[1] as i32 - 0x30;
                let num2 = dupe[3] as i32 - 0x30;

                if c1 >= num1.try_into().unwrap() && c2 >= num2.try_into().unwrap() {
                    println!("{}: {}", j, words[i]);
                    j += 1;
                }
            }

            if dupe[0] == '?' {
                //no dupes, print results as normal!
                println!("{}: {}", j, words[i]);
                j += 1;
            }
        }
    }

    if j == 1 {
        // this is probably a lame/hacky way of handling another edge case,
        // but in case someone accidentally puts a confirmed letter into the
        // wrong letters argument, we check for any matching characters and
        // suggest that it might be an input error. it seems inessential to do this
        // more than once, so once we find a collision we just exit the process
        let wrong_letters: Vec<_> = wrong.unwrap().chars().collect();
        for i in 0..right_letters.len() {         
            for n in 0..wrong_letters.len() {
                if right_letters[i] == wrong_letters[n] {
                    println!("No matches with this pattern. Did you enter a correct letter as wrong? '{}' was found in both your pattern and wrong letters.", wrong_letters[n]);
                    std::process::exit(0);
                }
            }
        }
        println!("No matches with this pattern.");
        
    }
}

fn dupe_check(pattern: String) -> Vec<char> {
    // this is probably a convoluted solution to a problem with our regex.
    // positive lookaheads cannot be iterated, so we can't tell our regex
    // to match 'a' anywhere in the word 2 times. instead, dupe_check returns
    // a vector that tracks duplicated letters, and we can then filter our regex
    // results to match our dupe_check value

    // right now i have this as a hideous set of if tests, but will eventually
    // test to see if nesting some loops would be faster execution wise.

    let mut dupes_vec = vec!['?', '?', '?', '?'];

    // here we can make some nice assumptions about our specific wordle context.
    // there are no 5 letter words that contain more than 2 duplicate letters, so
    // the vector just needs 4 elements, indices 0 and 2 are the duplicate characters

    // another nice assumption we can make: any 3 letter duplicate word like GEESE
    // will also have one of those e's in the correct place, so our filtered regex will
    // match correctly even on a non-word pattern like eE?e? for GEESE, and correctly
    // returns only words with 3 e's.

    let chars: Vec<_> = pattern.to_lowercase().chars().collect();

    // we can convert everything to lowercase chars for easy matching

    let guesses = chars.len() / 5;

    for p in 0..guesses {
        let mut guess_vec = vec!['?', '?', '?', '?', '?'];
        for k in 0..5 {
            let offset = p * 5;
            guess_vec[k] = chars[k + offset];
        }
        // this allows us to rebuild a vector of guesses for each round of guess inputted
        guess_vec.retain(|&x| x != '?');
        // remove wrong guesses, which aren't necessary
        for _i in 0..guesses {
            for i in 0..guess_vec.len() {
                // compare all guessed letters against each other to match any duplicates
                // there are probably faster methods, but we're talking about microseconds here
                let mut j = i + 1; 
                while j < guess_vec.len() {
                    if i != j && guess_vec[i] == guess_vec[j] {
                        if dupes_vec[0] == '?' {
                            dupes_vec[0] = guess_vec[i];
                            dupes_vec[1] = '2';
                        } else if dupes_vec[0] != '?' && dupes_vec[0] != guess_vec[i] && dupes_vec[2] != guess_vec[i] {
                            dupes_vec[2] = guess_vec[i];
                            dupes_vec[3] = '2';
                        } else if dupes_vec[2] != '?' && dupes_vec[2] == guess_vec[i] { 
			                dupes_vec[3] = '3';
			            } else {
                            dupes_vec[1] = '3';
                        }
                    }
                    j += 1;
                }
            }
        }
    }
    return dupes_vec;
}
