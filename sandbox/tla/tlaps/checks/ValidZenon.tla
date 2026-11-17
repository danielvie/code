-------------------------- MODULE ValidZenon --------------------------
EXTENDS TLAPS

THEOREM ConjunctionSwap ==
    ASSUME NEW P, NEW Q, P /\ Q
    PROVE Q /\ P
BY Zenon
=======================================================================
