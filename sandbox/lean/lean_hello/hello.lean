theorem add_zero (n : Nat) : n + 0 = n := by
  rfl

example : 42 + 0 = 42 := by
  exact add_zero 42

def main : IO Unit :=
  IO.println "Hello, world!"
