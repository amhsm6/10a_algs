use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::fmt::Debug;

type Graph<V> = HashMap<V, HashMap<V, usize>>;

fn bfs<V>(graph: &Graph<V>, start: V, end: V) -> Vec<V> where
    V: Clone + Copy + Hash + Eq + Debug
{
    let mut parents = HashMap::new();

    let mut distances = HashMap::new();
    distances.insert(start, 0);

    let mut queue = VecDeque::new();
    queue.push_back(start);

    while !queue.is_empty() {
        let v = queue.pop_front().unwrap();
        println!("Processing Vertex {v:?}");

        for (&u, &weight) in &graph[&v] {
            println!(" Found Vertex {u:?} with Weight {weight:?}");

            if distances.contains_key(&u) {
                if distances[&v] + weight < distances[&u] {
                    println!(
                        "  UPD distances, old: {:?}, new: {:?}",
                        distances[&u],
                        distances[&v] + weight
                    );

                    *parents.get_mut(&u).unwrap() = v;
                    *distances.get_mut(&u).unwrap() = distances[&v] + weight;
                } else {
                    continue;
                }
            } else {
                println!("  NEW vertex");

                parents.insert(u, v);
                distances.insert(u, distances[&v] + weight);
            }

            queue.push_back(u);
        }
    }

    let mut path = Vec::new();

    let mut curr = end;
    while parents.contains_key(&curr) {
        path.push(curr);
        curr = parents[&curr];
    }
    path.push(start);

    path.into_iter().rev().collect()
}

fn main() {
    let mut graph = HashMap::new();

    graph.insert(1, HashMap::from([(2, 1), (3, 4)]));
    graph.insert(2, HashMap::from([(3, 2)]));
    graph.insert(3, HashMap::new());

    println!("{:?}", bfs(&graph, 1, 3));
}
