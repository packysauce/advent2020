use daggy::{Dag, NodeIndex};
use std::{
    collections::{HashMap, HashSet},
    io::{stdin, BufReader},
};
use std::io::BufRead;

type IndexMap = HashMap<String, NodeIndex>;
type BagDag = Dag<String, u32>;

#[derive(Debug)]
struct Contains(u32);

#[derive(Debug)]
struct Bags {
    indices: IndexMap,
    dag: BagDag,
}

fn all_ancestors(idx: NodeIndex, dag: &BagDag) -> HashSet<NodeIndex> {
    use daggy::Walker;
    let mut set = HashSet::new();
    let parent_iter = dag.parents(idx);
    for (_edge, node) in parent_iter.iter(&dag) {
        set.insert(node);
        set.extend(all_ancestors(node, &dag));
    }
    set
}

fn bags_contained_by(idx: NodeIndex, dag: &BagDag) -> u32 {
    use daggy::Walker;
    let children = dag.children(idx);
    let mut count = 0;
    for (e, n) in children.iter(&dag) {
        let number_of_bags = *dag.edge_weight(e).unwrap();
        count += number_of_bags + number_of_bags * bags_contained_by(n, &dag);
    }
    count
}

fn parse_line(line: &str) -> (&str, Vec<(u32, &str)>) {
    lazy_static::lazy_static! {
        static ref HANDLER: regex::Regex = regex::Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();
    };

    let name_len = line
        .split_whitespace()
        .take(2)
        .collect::<Vec<&str>>()
        .join(" ")
        .len();
    let mut out = Vec::new();
    for capture in HANDLER.captures_iter(line) {
        let count = capture.get(1).unwrap().as_str().parse().unwrap();
        let name = capture.get(2).unwrap().as_str();
        out.push((count, name));
    }

    (&line[0..name_len], out)
}

fn build_bagdag(reader: &mut dyn BufRead) -> Bags {
    let mut indices = HashMap::new();
    let mut dag: BagDag = Dag::new();
    let mut lines = reader.lines();

    while let Some(Ok(line)) = lines.next() {
        let (name, children) = parse_line(&line);
        let parent_name = name.to_string();
        let parent_index = *indices
            .entry(parent_name)
            .or_insert_with(|| dag.add_node(name.to_string()));
        for (count, child_name) in children {
            let name = child_name.to_string();
            let child_index = indices
                .entry(name)
                .or_insert_with(|| dag.add_node(child_name.to_string()));
            dag.add_edge(parent_index, *child_index, count).unwrap();
        }
    }

    Bags { indices, dag }
}

fn main() {
    let mut r = BufReader::new(stdin());
    let bags = build_bagdag(&mut r);
    // part 1 - how many shiny gold options are there
    let idx = bags.indices.get("shiny gold").unwrap();
    let ancestors = all_ancestors(*idx, &bags.dag);
    // part 2 - how much does a shiny gold hold?
    let bag_count = bags_contained_by(*idx, &bags.dag);

    use daggy::petgraph::dot::{Config, Dot};
    println!("{:?}", Dot::with_config(&bags.dag, &[]));

    println!("total: {}", ancestors.len());
    println!("shiny gold contains {} bags", bag_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    #[test]
    fn parse_one_line() {
        let line = "striped crimson bags contain 2 shiny gold bags, 4 pale indigo bags, 4 light maroon bags.";
        let (name, contains) = parse_line(line);
        assert_eq!(name, "striped crimson");
        assert_eq!(
            contains,
            vec![
                (2u32, "shiny gold"),
                (4, "pale indigo"),
                (4, "light maroon")
            ]
        );
    }

    #[test]
    fn test_ancestor_lookup() {
        let mut reader = std::io::Cursor::new(DATA);
        let stuff = build_bagdag(&mut reader);
        let idx = *stuff.indices.get("shiny gold").unwrap();
        let total = all_ancestors(idx, &stuff.dag);
        assert_eq!(4, total.len());
    }

    #[test]
    fn test_child_lookup() {
        let mut reader = std::io::Cursor::new(DATA);
        let stuff = build_bagdag(&mut reader);

        let idx = *stuff.indices.get("shiny gold").unwrap();
        let total = bags_contained_by(idx, &stuff.dag);

        assert_eq!(32, total);
    }

    #[test]
    fn test_child_simple_case() {
        let data = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        let mut reader = std::io::Cursor::new(data);
        let stuff = build_bagdag(&mut reader);

        let idx = *stuff.indices.get("shiny gold").unwrap();
        let total = bags_contained_by(idx, &stuff.dag);

        assert_eq!(126, total);
    }

    #[test]
    fn test_build_map_from_strings() {}
}
