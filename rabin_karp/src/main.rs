fn hash(data: &str) -> u32 {
    let alph_len = 256;
    let q = 5179;

    let mut res = 0;
    
    for c in data.chars() {
        res = (alph_len * res + c as u32) % q;
    }
    
    res
}

fn substr(data: &str, pattern: &str) -> Vec<usize> {
    let alph_len: i64 = 256;
    let q: i64 = 5179;

    let data_len = data.chars().count();
    let pattern_len = pattern.chars().count();

    let pattern_hash = hash(pattern);

    let mut res = Vec::new();

    let mut curr_hash = hash(&data.chars().take(pattern_len).collect::<String>());
    for i in 0..data_len-pattern_len+1  {
        if curr_hash == pattern_hash {
            if &data[i..i+pattern_len] == pattern {
                res.push(i);
            }
        }

        if i == data_len - pattern_len { break }

        let char_to_add = data.chars().nth(i + pattern_len).unwrap() as i64;
        let char_to_remove = data.chars().nth(i).unwrap() as i64;

        let v = char_to_remove * alph_len.pow(pattern_len as u32 - 1u32);
        let mut new_hash = ((curr_hash as i64 - v) * alph_len + char_to_add) % q;
        if new_hash < 0 { new_hash += q; }

        curr_hash = new_hash as u32;
    }

    res
}

fn main() {
    let res = substr("abcdabcdgsab", "ab");
    println!("{:?}", res);
}
