use std::{collections::HashMap, fmt::Display, fs::File, io::Read};

struct Graph {
    nodes: Vec<String>,
    adjacent: Vec<Vec<usize>>,
}

impl Graph {
    fn paths_to_end(&self, current: usize, visited: &mut [bool]) -> usize {
        if current == self.nodes.len() - 1 {
            return 1;
        }

        let mut paths = 0;
        for neighbour in &self.adjacent[current] {
            visited[*neighbour] = true;
            paths += self.paths_to_end(*neighbour, visited);
            visited[*neighbour] = false;
        }

        return paths;
    }

    fn paths_to_end2(
        &self,
        current: usize,
        visited: &mut [bool],
        dac: usize,
        fft: usize,
        cache: &mut [Option<usize>],
    ) -> usize {
        if let Some(paths) =
            cache[(current << 2) + (visited[dac] as usize) * 2 + (visited[fft] as usize)]
        {
            return paths;
        }

        if current == self.nodes.len() - 1 {
            if visited[dac] && visited[fft] {
                return 1;
            } else {
                return 0;
            }
        }

        let mut paths = 0;
        for neighbour in &self.adjacent[current] {
            visited[*neighbour] = true;
            paths += self.paths_to_end2(*neighbour, visited, dac, fft, cache);
            visited[*neighbour] = false;
        }

        cache[(current << 2) + (visited[dac] as usize) * 2 + (visited[fft] as usize)] = Some(paths);
        return paths;
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (node, neighbours) in self.adjacent.iter().enumerate() {
            let neighbours = neighbours
                .iter()
                .map(|neighbour| self.nodes[*neighbour].as_ref())
                .collect::<Vec<_>>();
            writeln!(f, "{}: {}", self.nodes[node], neighbours.join(" "))?;
        }

        Ok(())
    }
}

fn parse(input: &str) -> (Graph, usize, usize, usize, usize) {
    let mut nodes = input
        .lines()
        .map(|line| line.split(": ").next().unwrap().to_string())
        .collect::<Vec<_>>();
    nodes.push(String::from("out"));
    let name_to_index: HashMap<&str, usize> = nodes
        .iter()
        .enumerate()
        .map(|(i, node_name)| (node_name.as_ref(), i))
        .collect();
    let adjacent = input
        .lines()
        .map(|line| {
            line.split(": ")
                .last()
                .unwrap()
                .split(" ")
                .map(|node_name| name_to_index[node_name])
                .collect()
        })
        .collect();

    let you = name_to_index.get("you").cloned().unwrap_or(0);
    let svr = name_to_index.get("svr").cloned().unwrap_or(0);
    let dac = name_to_index.get("dac").cloned().unwrap_or(0);
    let fft = name_to_index.get("fft").cloned().unwrap_or(0);
    (Graph { nodes, adjacent }, you, svr, dac, fft)
}

fn main() {
    let mut input = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    let (graph, you, svr, dac, fft) = parse(&input);

    let mut visited = vec![false; graph.nodes.len()];
    visited[you] = true;
    println!("{}", graph.paths_to_end(you, &mut visited));

    visited = vec![false; graph.nodes.len()];
    visited[svr] = true;
    println!(
        "{}",
        graph.paths_to_end2(
            svr,
            &mut visited,
            dac,
            fft,
            &mut vec![None; graph.nodes.len() * 4]
        )
    );
}
