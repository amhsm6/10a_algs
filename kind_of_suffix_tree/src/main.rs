use std::collections::HashMap;

struct Trie {
    graph: HashMap<i32, HashMap<i32, char>>,
    next_vertex: i32
}

impl Trie {
    fn new() -> Trie {
        Trie { graph: HashMap::new(), next_vertex: 1 }
    }

    // Function to build simple suffix tree
    fn build_suffix_tree(string: &str) -> Trie {
        let mut trie = Trie::new();

        for i in 0..string.chars().count() {
            // Insert suffix, started with index i
            trie.add(&string.chars().skip(i).collect::<String>());
        }

        trie
    }

    // Function to add a string to trie
    fn add(&mut self, string: &str) {
        let mut curr = 0;
        // Iterating over characters in string
        for c in string.chars() {
            if !self.graph.contains_key(&curr) {
                self.graph.insert(curr, HashMap::new());
            }

            let mut found = false;

            // Trying to find next vertex, so that the path contains char c
            for next in &self.graph[&curr] {
                if *next.1 == c {
                    curr = *next.0;
                    found = true;
                    break;
                }
            }

            // If not found, should add new vertex with char c on path to it
            if !found {
                self.graph.get_mut(&curr).unwrap().insert(self.next_vertex, c);
                curr = self.next_vertex;
                self.next_vertex += 1;
            }
        }
    }

    // Function to find string in trie
    fn find(&self, string: &str) -> bool {
        let chars = string.chars().collect::<Vec<char>>();
        let len = chars.len();

        let mut curr = 0;
        // char_idx stores current char index in pattern
        let mut char_idx = 0;
        while char_idx < len {
            if !self.graph.contains_key(&curr) { return false; }

            let mut found = false;

            // Trying to find next vertex with char, which is in pattern with index char_idx
            for next in &self.graph[&curr] {
                if *next.1 == chars[char_idx] {
                    curr = *next.0;
                    char_idx += 1;
                    found = true;
                    break;
                }
            }

            // If found and it was the last char to find, it means, that we found our pattern
            if found && char_idx == len { return true; }
            // If at some point we didn't find necessary char, our pattern is not in suffix tree
            if !found { return false; }
        }

        false
    }
}

fn main() {
    let foo = Trie::build_suffix_tree("banana");

    println!("{}", foo.find("nana"));
}
