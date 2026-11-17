------------------------------ MODULE Counter ------------------------------
EXTENDS Naturals, Sequences

CONSTANT Max
VARIABLES count, history

\* A bounded counter with a history of its values.
(* Block comments can contain (* nested comments *). *)
vars == <<count, history>>
Label == "counter\n"
View == [value |-> count, label |-> Label]
Values == {n \in 0..Max : n % 2 = 0}
Bounded == \A n \in Values : n <= Max
Clamp(n) == IF n > Max THEN Max ELSE n
Enabled == TRUE /\ ~FALSE
Numbers == <<\b101, \o17, \hFF, 1.5>>
f[n \in 0..Max] == Clamp(n)
LocalValue == LET n == 1 IN n + count

TypeOK ==
    /\ count \in 0..Max
    /\ history \in Seq(0..Max)

Init ==
    /\ count = 0
    /\ history = <<>>

Next ==
    /\ count < Max
    /\ count' = count + 1
    /\ history' = Append(history, count')

ChangingStep == <<Next>>_vars
Spec == Init /\ [][Next]_vars /\ WF_vars(Next)

THEOREM Safety == Spec => []TypeOK
=============================================================================
