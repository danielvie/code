--------------------------- MODULE ValidSMT ---------------------------
EXTENDS Integers, TLAPS

THEOREM SuccessorGreater == \A n \in Int : n + 1 > n
BY SMT
=======================================================================
