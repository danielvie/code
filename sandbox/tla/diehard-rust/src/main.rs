use std::error::Error;
use std::path::Path;

use tla_checker::checker::{self, CheckResult, CheckStats, Counterexample};
use tla_checker::load::{PrepareError, Prepared, prepare_from_path};

fn load_example() -> Result<Prepared, PrepareError> {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("model");
    prepare_from_path(
        &directory.join("DieHard.tla"),
        Some(&directory.join("DieHard.cfg")),
        &[],
    )
}

fn solve(prepared: &Prepared) -> Result<(Counterexample, CheckStats), String> {
    match checker::check(&prepared.spec, &prepared.domains, &prepared.checker_config) {
        CheckResult::InvariantViolation(trace, stats)
            if prepared
                .spec
                .invariant_names
                .get(trace.violated_invariant)
                .and_then(|name| name.as_deref())
                == Some("NotSolved") =>
        {
            Ok((trace, stats))
        }
        other => Err(format!(
            "Expected a NotSolved counterexample, got {other:#?}"
        )),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let prepared = load_example()?;
    for warning in &prepared.warnings {
        eprintln!("Warning: {warning:?}");
    }
    let (solution, stats) = solve(&prepared)?;
    println!("NotSolved is violated: the four-gallon solution was found.");
    println!(
        "{} actions, {} states explored.\n",
        solution.trace.len() - 1,
        stats.states_explored
    );
    print!(
        "{}",
        checker::format_trace_with_actions(&solution.trace, &solution.actions, &prepared.spec.vars,)
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tla_checker::ast::Value;

    #[test]
    fn original_config_finds_the_six_action_solution() {
        let prepared = load_example().unwrap();
        let (solution, _) = solve(&prepared).unwrap();
        let big = prepared
            .spec
            .vars
            .iter()
            .position(|v| &**v == "big")
            .unwrap();
        let small = prepared
            .spec
            .vars
            .iter()
            .position(|v| &**v == "small")
            .unwrap();
        let volumes: Vec<_> = solution
            .trace
            .iter()
            .map(|state| (state.values[big].clone(), state.values[small].clone()))
            .collect();
        // Independently checked against the bundled Java TLC example.
        let expected: Vec<_> = [(0, 0), (5, 0), (2, 3), (2, 0), (0, 2), (5, 2), (4, 3)]
            .into_iter()
            .map(|(big, small)| (Value::Int(big), Value::Int(small)))
            .collect();
        assert_eq!(volumes, expected);
    }

    #[test]
    fn type_ok_holds_for_all_sixteen_reachable_states() {
        let mut prepared = load_example().unwrap();
        // Remove only the puzzle goal so exploration does not stop at a solution.
        let index = prepared
            .spec
            .invariant_names
            .iter()
            .position(|name| name.as_deref() == Some("NotSolved"))
            .unwrap();
        prepared.spec.invariants.remove(index);
        prepared.spec.invariant_names.remove(index);
        match checker::check(&prepared.spec, &prepared.domains, &prepared.checker_config) {
            CheckResult::Ok(stats) => assert_eq!(stats.states_explored, 16),
            other => panic!("Expected TypeOK to hold, got {other:#?}"),
        }
    }
}
