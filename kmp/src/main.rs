use std::collections::HashMap;

fn suffix_func(string: &str) -> usize {
    for i in (0..string.len()-1).rev() {
        let suffix = &string[i..];
        if string.matches(suffix).count() > 2 {
            return string.len() - i - 1;
        }   
    }
}

fn substr(text: &str, pattern: &str) -> i32 {
    let pattern_arr = pattern.chars().collect::<Vec<char>>();
    let pattern_len = pattern_arr.len();

    let mut foo = HashMap::new();

    for i in (0..pattern_len-1).rev() {
        if !foo.contains_key(&pattern_arr[i]) {
            foo.insert(pattern_arr[i], pattern_len - i - 1);
        }
    }

    if !foo.contains_key(pattern_arr.last().unwrap()) {
        foo.insert(*pattern_arr.last().unwrap(), pattern_len);
    }

    let mut text_idx = pattern_len - 1;
    let mut pattern_idx = pattern_len - 1;

    bar = suffix_func(pattern);

    while text_idx < text.chars().count() {
        if text.chars().nth(text_idx).unwrap() == pattern_arr[pattern_idx] {
            if pattern_idx == 0 {
                return text_idx as i32;
            }   

            text_idx -= 1;
            pattern_idx -= 1;
        } else {
            if pattern_idx < pattern_len - 1 {
                if bar == 1 {
                    text_idx += foo[&pattern_arr.last().unwrap()];
                } else {
                    text_idx += bar;
                }
            } else {
                text_idx += foo.get(&text.chars().nth(text_idx).unwrap()).unwrap_or(&pattern_len);
                pattern_idx = pattern_len - 1;
            }
        }
    }

    return -1;
}

fn main() {
    println!("{}", substr("123456123", "123"));
}
