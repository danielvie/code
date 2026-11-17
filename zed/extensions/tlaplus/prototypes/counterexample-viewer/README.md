# Counterexample explorer visual proposal

Throwaway, standalone HTML. No build, dependencies, network requests, TLC execution, or Zed integration. All traces, outcomes, and source mapping are illustrative fixtures.

Open `index.html` directly, or run `task run` here and visit http://127.0.0.1:8765.

## Compare layouts

- `?variant=1`: Workbench. Trace list, specification excerpt, and variable diffs.
- `?variant=2`: State map. Clickable trace graph, process activity, and a liveness back-edge.
- `?variant=3`: Walkthrough. A transition explanation and before/after snapshots.

Switch using the floating bottom controls or left/right arrow keys. J/K move forward/back through states. Playback, the scrubber, state selection, and the changed-only filter work on the sample data. The sample selector switches between a safety failure and a liveness loop. Playback loops in the liveness sample.

The safety example separates checking a lock from entering a critical section, allowing two processes to enter. The liveness example illustrates starvation without a fairness assumption. Neither example is a verified executable specification. The source pane is a shortened illustrative excerpt.

`task check` checks the inline JavaScript syntax only. This prototype does not change extension capabilities or establish that Zed supports this custom interface.
