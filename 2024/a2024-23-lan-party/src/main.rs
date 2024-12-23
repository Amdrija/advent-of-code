use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

fn parse(content: &str) -> Vec<(&str, &str)> {
    content
        .lines()
        .map(|line| {
            let mut splitted = line.split("-");
            (splitted.next().unwrap(), splitted.next().unwrap())
        })
        .collect()
}

fn make_graph<'a>(edges: &'a [(&str, &str)]) -> HashMap<&'a str, HashSet<&'a str>> {
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (from, to) in edges {
        graph.entry(from).or_default().insert(to);
        graph.entry(to).or_default().insert(from);
    }

    graph
}

fn find_triplets<'a>(
    graph: &'a HashMap<&str, HashSet<&str>>,
    edges: &'a [(&str, &str)],
) -> Vec<(&'a str, &'a str, &'a str)> {
    let mut result = vec![];
    for (from, to) in edges {
        for third in graph[*from].intersection(&graph[*to]) {
            result.push((*from, *to, *third));
        }
    }

    result
}

fn find_maximum_clique<'a>(
    graph: &'a HashMap<&str, HashSet<&str>>,
    edges: &'a [(&str, &str)],
) -> Vec<&'a str> {
    let mut cliques = edges
        .iter()
        .map(|(from, to)| vec![*from, *to])
        .collect::<HashSet<_>>();
    while cliques.len() > 1 {
        let mut bigger_cliques = HashSet::new();

        for clique in &cliques {
            let mut intersection = graph[clique[0]].clone();
            for node in &clique[1..] {
                intersection = intersection
                    .intersection(&graph[*node])
                    .map(|s| *s)
                    .collect::<HashSet<_>>();
            }

            for new_node in intersection {
                let mut new_clique = clique.clone();
                new_clique.push(new_node);
                new_clique.sort();
                bigger_cliques.insert(new_clique);
            }
        }
        cliques = bigger_cliques;
    }

    cliques.into_iter().next().unwrap()
}

fn main() {
    let mut content = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();
    let edges = parse(&content);
    let graph = make_graph(&edges);
    let triplets = find_triplets(&graph, &edges);
    let t_triplets = triplets
        .iter()
        .filter(|(a, b, c)| a.starts_with("t") || b.starts_with("t") || c.starts_with("t"));
    println!("{}", t_triplets.count() / 3);
    println!("{}", find_maximum_clique(&graph, &edges).join(","));
}
