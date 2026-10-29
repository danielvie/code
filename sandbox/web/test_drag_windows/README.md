# Cross-window drag prototype

Question: can a tree item be dragged between two browser windows in headless CI?

Verdict: application drag/drop behavior can be verified headlessly by capturing the source window's `DataTransfer` payload and synthetically dropping that data in the destination window. This does not simulate an OS pointer crossing window boundaries.

```sh
task setup
task run
task test
```

The page is intentionally throwaway prototype code at `http://localhost:3000/prototype.html`.
