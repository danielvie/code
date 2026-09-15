use std::collections::{BTreeSet, VecDeque};
use ticket_reservation::{Action, Ticket};

#[test]
fn every_reachable_state_and_labeled_transition_matches_tlc() {
    let mut expected_initial = None;
    let mut expected_states = BTreeSet::new();
    let mut expected_edges = BTreeSet::new();
    for line in include_str!("fixtures/tlc-graph.tsv").lines() {
        if line.starts_with('#') {
            continue;
        }
        let fields: Vec<_> = line.split('\t').collect();
        match fields.as_slice() {
            ["init", state] => assert!(expected_initial.replace(state.to_string()).is_none()),
            ["state", state] => assert!(expected_states.insert(state.to_string())),
            ["edge", source, action, target] => assert!(expected_edges.insert((
                source.to_string(),
                action.to_string(),
                target.to_string()
            ))),
            _ => panic!("invalid TLC fixture line: {line}"),
        }
    }

    let initial = Ticket::new();
    assert_eq!(Some(initial.state().model_key()), expected_initial);
    assert!(!expected_edges.is_empty());
    let mut states = BTreeSet::from([initial.state().model_key()]);
    let mut edges = BTreeSet::new();
    let mut queue = VecDeque::from([initial]);
    let mut attempts = 0;
    while let Some(ticket) = queue.pop_front() {
        let source = ticket.state().model_key();
        for action in Action::ALL {
            attempts += 1;
            let mut next = ticket;
            match next.apply(action) {
                Ok(()) => {
                    let target = next.state().model_key();
                    assert!(
                        expected_states.contains(&target),
                        "unexpected state: {target}"
                    );
                    edges.insert((source.clone(), action.to_string(), target.clone()));
                    if states.insert(target) {
                        queue.push_back(next);
                    }
                }
                Err(_) => assert_eq!(ticket, next, "rejected {action} mutated {source}"),
            }
        }
    }
    assert_eq!(states, expected_states, "reachable states differ");
    assert_eq!(
        edges, expected_edges,
        "enabled actions or destinations differ"
    );
    assert_eq!(attempts, expected_states.len() * Action::ALL.len());
    println!(
        "Matched {} states, {} labeled edges, {attempts} command attempts",
        states.len(),
        edges.len()
    );
}
