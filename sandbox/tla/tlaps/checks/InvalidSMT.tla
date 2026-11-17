-------------------------- MODULE InvalidSMT --------------------------
EXTENDS Integers, TLAPS

THEOREM DeliberatelyFalse == \A n \in Int : n + 1 < n
BY SMT
=======================================================================
