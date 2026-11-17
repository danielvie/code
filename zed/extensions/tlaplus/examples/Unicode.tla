------------------------------ MODULE Unicode ------------------------------
EXTENDS Naturals

VARIABLE x
vars == ⟨x⟩
AlternateTuple == 〈x〉
Init == x = 0
Next == x < 10 ∧ x' = x + 1
ChangingStep == ⟨Next⟩_vars
Spec == Init ∧ □[Next]_vars
TypeOK == x ∈ Nat
THEOREM Safety == Spec ⇒ □TypeOK
=============================================================================
