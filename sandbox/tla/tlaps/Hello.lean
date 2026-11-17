import Lean.Elab.Tactic.Omega

-- For every integer n, adding 1 gives a number greater than n.
theorem addingOneMakesItBigger :
    ∀ n : Int, n + 1 > n := by
  intro n
  omega
