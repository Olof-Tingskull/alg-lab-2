use std::collections::{HashMap, HashSet};

struct GraphColoringInput {
    n_colors: usize,
    edges: Vec<(usize, usize)>,
}

struct NumberExtractor<'a> {
    s: &'a str,
    index: usize,
}

impl<'a> NumberExtractor<'a> {
    fn new(s: &'a str) -> Self {
        NumberExtractor { s, index: 0 }
    }
}

impl<'a> Iterator for NumberExtractor<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let bytes = self.s.as_bytes();
        let mut current_num = None;
        let mut start = None;

        while self.index < bytes.len() {
            let c = bytes[self.index] as char;
            if c.is_digit(10) {
                if start.is_none() {
                    start = Some(self.index);
                }
            } else {
                if let Some(start_idx) = start {
                    let num_str = &self.s[start_idx..self.index];
                    if let Ok(num) = num_str.parse::<usize>() {
                        current_num = Some(num);
                    }
                    start = None;
                    self.index += 1;
                    break;
                }
            }
            self.index += 1;
        }

        if start.is_some() && current_num.is_none() {
            // This handles the case where the number is at the end of the string
            let num_str = &self.s[start.unwrap()..self.index];
            if let Ok(num) = num_str.parse::<usize>() {
                current_num = Some(num);
            }
        }

        current_num
    }
}


fn parse_input(input: &str) -> Result<GraphColoringInput, String> {
    let mut extractor = NumberExtractor::new(input);
    let _n_vertices = extractor.next().ok_or("Expected number of vertices missing")?;
    let n_edges = extractor.next().ok_or("Expected number of edges missing")?;
    let n_colors = extractor.next().ok_or("Expected number of colors missing")?;

    let edges: Vec<(usize, usize)> = (0..n_edges)
        .map(|_| {
            let from = extractor.next().ok_or::<&str>("Expected edge start vertex missing")?;
            let to = extractor.next().ok_or::<&str>("Expected edge end vertex missing")?;
            Ok((from, to))
        })
        .collect::<Result<_, &str>>()?;

    Ok(GraphColoringInput { n_colors, edges })
}


const BASE_ACTORS: usize = 3;
const BASE_SCENES: usize = 2;
const BASE_ROLES: usize = 3;

pub fn reduce_to_casting_problem(input: &str) -> Result<String, String> {
    let GraphColoringInput { n_colors, edges } = parse_input(input)?;

    let attached_vertices: HashSet<usize> = edges
        .iter()
        .flat_map(|&(from, to)| vec![from, to])
        .collect();

    let vertex_map: HashMap<_, _> = attached_vertices
        .iter()
        .enumerate()
        .map(|(i, &v)| (v, i + 1))
        .collect();

    let mut output_lines = Vec::<String>::new();
    let mut add_line = |line: String| output_lines.push(line);

    let num_roles = attached_vertices.len() + BASE_ROLES;
    let num_scenes = edges.len() + BASE_SCENES;
    let num_actors = usize::min(n_colors + BASE_ACTORS, num_roles);

    // Amounts
    add_line(format!("{}", num_roles));
    add_line(format!("{}", num_scenes));
    add_line(format!("{}", num_actors));

    // Roles

    // Base Roles
    for i in 1..=BASE_ROLES {
        add_line(format!("1 {}", i));
    }

    // Rest of the roles
    let actors: Vec<String> = (1 + BASE_ACTORS..=num_actors)
        .map(|actor| actor.to_string())
        .collect();
    for _ in attached_vertices {
        add_line(format!("{} {}", actors.len(), actors.join(" ")));
    }

    // Scenes

    // Base Scenes
    add_line("2 1 3".to_string());
    add_line("2 2 3".to_string());

    // Rest of the scenes
    for &(from, to) in &edges {
        add_line(format!(
            "2 {} {}",
            vertex_map[&from] + 3,
            vertex_map[&to] + 3
        ));
    }

    Ok(output_lines.join("\n"))
}
