use std::{cell::RefCell, cmp::Ordering, fmt::Display, rc::Rc};

use graph::*;

#[test]
fn single_node() {
    let spec = vec![(1, vec![])];
    let graph = make_graph(&spec);
    let parsed_graph = parse_graph(graph);

    assert_specs_eq(parsed_graph, spec);
}

#[test]
fn two_nodes() {
    let spec = vec![(1, vec![2]), (2, vec![1])];
    let graph = make_graph(&spec);
    let parsed_graph = parse_graph(graph);

    assert_specs_eq(parsed_graph, spec);
}

#[test]
fn three_nodes() {
    let spec = vec![(1, vec![2, 3]), (2, vec![1]), (3, vec![1])];
    let graph = make_graph(&spec);
    let parsed_graph = parse_graph(graph);

    assert_specs_eq(parsed_graph, spec);
}

#[test]
fn three_nodes_with_cycle() {
    let spec = vec![(1, vec![2, 3]), (2, vec![1, 3]), (3, vec![1, 2])];
    let graph = make_graph(&spec);
    let parsed_graph = parse_graph(graph);

    assert_specs_eq(parsed_graph, spec);
}

#[test]
fn many_nodes_around_center() {
    let spec = vec![
        (1, vec![2, 3, 4, 5, 6, 7]),
        (2, vec![1]),
        (3, vec![1]),
        (4, vec![1]),
        (5, vec![1]),
        (6, vec![1]),
        (7, vec![1]),
    ];
    let graph = make_graph(&spec);
    let parsed_graph = parse_graph(graph);

    assert_specs_eq(parsed_graph, spec);
}

#[test]
fn long_rope() {
    let spec = vec![
        (1, vec![2]),
        (2, vec![1, 3]),
        (3, vec![2, 4]),
        (4, vec![3, 5]),
        (5, vec![4, 6]),
        (6, vec![5, 7]),
        (7, vec![6]),
    ];
    let graph = make_graph(&spec);
    let parsed_graph = parse_graph(graph);

    assert_specs_eq(parsed_graph, spec);
}

#[test]
fn big_circle() {
    let spec = vec![
        (1, vec![7, 2]),
        (2, vec![1, 3]),
        (3, vec![2, 4]),
        (4, vec![3, 5]),
        (5, vec![4, 6]),
        (6, vec![5, 7]),
        (7, vec![6, 1]),
    ];
    let graph = make_graph(&spec);
    let parsed_graph = parse_graph(graph);

    assert_specs_eq(parsed_graph, spec);
}

#[test]
fn many_connected() {
    let spec = vec![
        (1, vec![2, 3, 4, 5, 6, 7]),
        (2, vec![1, 3, 4, 5, 6, 7]),
        (3, vec![1, 2, 4, 5, 6, 7]),
        (4, vec![1, 2, 3, 5, 6, 7]),
        (5, vec![1, 2, 3, 4, 6, 7]),
        (6, vec![1, 2, 3, 4, 5, 7]),
        (7, vec![1, 2, 3, 4, 5, 6]),
    ];
    let graph = make_graph(&spec);
    let parsed_graph = parse_graph(graph);

    assert_specs_eq(parsed_graph, spec);
}

#[test]
fn update_value() {
    let spec = vec![
        (1, vec![2, 3, 4, 5, 6, 7]),
        (2, vec![1, 3, 4, 5, 6, 7]),
        (3, vec![1, 2, 4, 5, 6, 7]),
        (4, vec![1, 2, 3, 5, 6, 7]),
        (5, vec![1, 2, 3, 4, 6, 7]),
        (6, vec![1, 2, 3, 4, 5, 7]),
        (7, vec![1, 2, 3, 4, 5, 6]),
    ];
    let graph = make_graph(&spec);

    // multiply all values by 10 in a fully connected graph
    //
    for n in graph.neighbors().chain([graph.clone()]) {
        n.update_value(|x| x * 10)
    }

    let expected = vec![
        (10, vec![20, 30, 40, 50, 60, 70]),
        (20, vec![10, 30, 40, 50, 60, 70]),
        (30, vec![10, 20, 40, 50, 60, 70]),
        (40, vec![10, 20, 30, 50, 60, 70]),
        (50, vec![10, 20, 30, 40, 60, 70]),
        (60, vec![10, 20, 30, 40, 50, 70]),
        (70, vec![10, 20, 30, 40, 50, 60]),
    ];

    let parsed_graph = parse_graph(graph);

    assert_specs_eq(parsed_graph, expected);
}

#[test]
fn remove_neighbor() {
    let spec = vec![
        (1, vec![2, 3, 4, 5, 6, 7]),
        (2, vec![1, 3, 4, 5, 6, 7]),
        (3, vec![1, 2, 4, 5, 6, 7]),
        (4, vec![1, 2, 3, 5, 6, 7]),
        (5, vec![1, 2, 3, 4, 6, 7]),
        (6, vec![1, 2, 3, 4, 5, 7]),
        (7, vec![1, 2, 3, 4, 5, 6]),
    ];
    let graph = make_graph(&spec);

    // partition graph into odd and even nodes
    //
    let nodes: Vec<_> = graph.neighbors().chain([graph.clone()]).collect();
    for node in nodes.iter() {
        for n in node.neighbors() {
            if n.value() % 2 != node.value() % 2 {
                node.remove_neighbor(n)
            }
        }
    }

    let expected = vec![
        (1, vec![3, 5, 7]),
        (3, vec![1, 5, 7]),
        (5, vec![1, 3, 7]),
        (7, vec![1, 3, 5]),
    ];

    let parsed_graph = parse_graph(graph);

    assert_specs_eq(parsed_graph, expected);
}

#[test]
fn generic_char() {
    let spec = vec![
        ('1', vec!['2', '3', '4', '5', '6', '7']),
        ('2', vec!['1', '3', '4', '5', '6', '7']),
        ('3', vec!['1', '2', '4', '5', '6', '7']),
        ('4', vec!['1', '2', '3', '5', '6', '7']),
        ('5', vec!['1', '2', '3', '4', '6', '7']),
        ('6', vec!['1', '2', '3', '4', '5', '7']),
        ('7', vec!['1', '2', '3', '4', '5', '6']),
    ];
    let graph = make_graph(&spec);
    let parsed_graph = parse_graph(graph);

    assert_specs_eq(parsed_graph, spec);
}

#[test]
fn generic_str() {
    let spec = vec![
        ("1", vec!["2", "3", "4", "5", "6", "7"]),
        ("2", vec!["1", "3", "4", "5", "6", "7"]),
        ("3", vec!["1", "2", "4", "5", "6", "7"]),
        ("4", vec!["1", "2", "3", "5", "6", "7"]),
        ("5", vec!["1", "2", "3", "4", "6", "7"]),
        ("6", vec!["1", "2", "3", "4", "5", "7"]),
        ("7", vec!["1", "2", "3", "4", "5", "6"]),
    ];
    let graph = make_graph(&spec);
    let parsed_graph = parse_graph(graph);

    assert_specs_eq(parsed_graph, spec);
}

#[test]
fn generic_not_ord_not_hash() {
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct S(u8);

    let spec = vec![
        (S(1), vec![S(2), S(3)]),
        (S(2), vec![S(1), S(3)]),
        (S(3), vec![S(1), S(2)]),
    ];

    // Simply ensure a graph can be constructed from
    // elements that aren't Ord or Hash.
    let _ = make_graph(&spec);
}

/// This test is very difficult to pass.
/// Remove the #[ignore] attribute to accept the challenge.
#[test]
// #[ignore]
fn not_memory_leak() {
    #[derive(PartialEq, Eq)]
    struct DropCounter {
        value: u8,
        drop_count: Rc<RefCell<isize>>,
    }
    impl Clone for DropCounter {
        fn clone(&self) -> Self {
            // don't count clones of values toward drop counter
            *RefCell::borrow_mut(&self.drop_count) -= 1;
            Self {
                value: self.value,
                drop_count: Rc::clone(&self.drop_count),
            }
        }
    }
    impl Drop for DropCounter {
        fn drop(&mut self) {
            *RefCell::borrow_mut(&self.drop_count) += 1;
        }
    }
    let drop_count = Rc::new(RefCell::new(0));
    {
        let dc = |value| DropCounter {
            value,
            drop_count: drop_count.clone(),
        };
        let dc_1 = dc(1);
        let dc_2 = dc(2);
        let dc_3 = dc(3);

        let spec = vec![
            (dc_1.clone(), vec![dc_2.clone(), dc_3.clone()]),
            (dc_2.clone(), vec![dc_1.clone(), dc_3.clone()]),
            (dc_3.clone(), vec![dc_1.clone(), dc_2.clone()]),
        ];
        let _ = make_graph(&spec);
    }
    let r = RefCell::borrow(&drop_count);
    assert_eq!(
        *r, 3,
        "
        You might have a memory leak in case of reference cycles...
        "
    )
}

//                                              //
// ---------------- test utils ---------------- //
//                                              //

type Spec<T> = Vec<(T, Vec<T>)>;

/// helper function to create a graph from a declarative specification
fn make_graph<T>(spec: &Spec<T>) -> Node<T>
where
    T: Clone + Eq,
{
    let nodes = spec
        .iter()
        .map(|(value, _)| Node::new(value.clone()))
        .collect::<Vec<_>>();

    for (node, neighbors) in nodes.iter().zip(spec.iter().map(|(_, n)| n)) {
        for neighbor in neighbors {
            let neighbor_node = nodes.iter().find(|n| &n.value() == neighbor).unwrap();
            node.add_neighbor(neighbor_node.clone());
        }
    }
    nodes[0].clone()
}

/// helper function to turn a graph back into its declarative spec for testing
fn parse_graph<T>(graph: Node<T>) -> Spec<T>
where
    T: Clone + Eq + Ord + Default + Display,
{
    let mut res = vec![(
        graph.value(),
        graph.neighbors().map(|n| n.value()).collect(),
    )];
    let mut work = graph.neighbors().collect::<Vec<_>>();

    while let Some(node) = work.pop() {
        let value = node.value();
        if res.iter().any(|(n, _)| n == &value) {
            continue;
        };
        res.push((value, node.neighbors().map(|n| n.value()).collect()));
        work.extend(node.neighbors());
    }
    res
}

fn assert_specs_eq<T>(mut parsed: Spec<T>, mut expected: Spec<T>)
where
    T: Clone + Eq + Ord + Default + Display,
{
    parsed.sort_by(|a, b| a.0.cmp(&b.0));
    expected.sort_by(|a, b| a.0.cmp(&b.0));
    // fill with dummy values to match length.
    // otherwise `zip` will discard and not check some values.
    match_len(&mut parsed, &mut expected);

    for (mut parsed, mut expected) in parsed.into_iter().zip(expected) {
        let value = match parsed.0.cmp(&expected.0) {
            Ordering::Less => panic!("unexpected node '{}'", parsed.0),
            Ordering::Greater => panic!("missing node '{}'", expected.0),
            Ordering::Equal => parsed.0, // all good
        };

        parsed.1.sort();
        expected.1.sort();
        // fill with dummy values to match length.
        // otherwise `zip` will discard and not check some values.
        match_len(&mut parsed.1, &mut expected.1);

        for (parsed, expected) in parsed.1.into_iter().zip(expected.1) {
            match parsed.cmp(&expected) {
                Ordering::Less => panic!("unexpected neighbor '{}' of node '{}'", parsed, value),
                Ordering::Greater => panic!("missing neighbor '{}' of node '{}'", expected, value),
                Ordering::Equal => { /* all good */ }
            }
        }
    }
}

/// fills smaller of two vectors with dummy values to match their length
fn match_len<T>(parsed: &mut Vec<T>, expected: &mut Vec<T>)
where
    T: Clone + Eq + Ord + Default,
{
    // I feel like I should add a comment why the dummy values are determined
    // the way they are. But I can't explain it, just think hard. 🙂
    // The order of elements is used to determine the error message
    // ("missing" or "unexpected"), so this property should be upheld.
    //
    match parsed.len().cmp(&expected.len()) {
        Ordering::Less => parsed.resize(expected.len(), expected[parsed.len()].clone()),
        Ordering::Equal => { /* all good */ }
        Ordering::Greater => expected.resize(parsed.len(), parsed[expected.len()].clone()),
    }
}
