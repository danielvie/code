-------------------- MODULE Euclid --------------------
EXTENDS Integers, TLAPS


\* `==` means: "is defined as"
\* `=` means: "equality"

p | q == \E d \in 1..q : q = p * d
\* “p divides q if there exists an integer d between 1 and q such that q = p * d.”

Divisors(q) == {d \in 1..q : d | q}
\* the set of all integers `d` from `1` through `q` for which `d` divies `q`.

Maximum(S) == CHOOSE x \in S : \A y \in S : x \geq y
\* choose an element `x` of `S` such that `x` is greater than or equal to every element `y` of `S`
\* `\A` means "for every"
\* `\geq` means "greater than or equal to"
\* `CHOOSE` means "pick randomly"

GCD(p,q) == Maximum(Divisors(p) \cap Divisors(q))
\* Greatest Common Divisor
\* `\cap` means `set intersection`

Number == Nat \ {0}
\* `Number` is a natual numbers with zero removed

CONSTANTS M, N
VARIABLES x, y

ASSUME NumberAssumption == M \in Number /\ N \in Number
\* Assume that `M` and `N` are both positibe integers. Call this assumption `NumberAssumption`

Init == /\ (x = M) 
        /\ (y = N)

Next == \/ /\ x < y
           /\ y' = y - x
           /\ x' = x
        \/ /\ y < x
           /\ x' = x-y
           /\ y' = y

Spec == Init /\ [][Next]_<<x,y>>
\* Start in a state satisfying `Init`. Then, at every step, either follow `Next` or leave both `x` and `y` unchanged
\* `[]` means `always`
\* `[Next]_<<x,y>>` is equivalent to `Next \/ UNCHANGED <<x,y>>`
\*                  and therefore `Next \/ (x' = x /\ y' = y)`
\* the `_` does not mean "unchanged" by itself. It is part of the notation `[Action]_variables`

ResultCorrect == (x = y) => x = GCD(M, N)
\* if the algorithm has reached equal values, its result is correct

InductiveInvariant ==
  /\ x \in Number
  /\ y \in Number
  /\ GCD(x, y) = GCD(M, N)
\* defines a condition intended to remain true throughout the algorithm
\* all three parts must hold
\* why "inductive"?
\* to prove this is an `inductive invariant`, you establish two things:
\*    1. it holds initially. `Init` sets `x = M` and `y = N`
\*    2. every allowed step preserves it. 

USE DEF Number
\* proof directive. it tells TLAPS to make the definition of `Number` available for use in subsequent proof steps in the current scope

THEOREM InitProperty == Init => InductiveInvariant
  BY NumberAssumption DEF Init, InductiveInvariant
\* ( THEOREM InitProperty == Init => InductiveInvariant )
\* 
\* if the system is in an initial state, then the invariant holds
\* THEOREM introduces a claim to prove
\* => means "implies"
\* 
\* unlike a normal definition, this line creates a proof obligation
\* 
\* ( BY NumberAssumption DEF Init, InductiveInvariant )
\* 
\* this asks TLAPS to prove the claim using:
\*   - the assumption `NumberAssumption`
\*   - the definitions of `Init` and `InductiveInvariant`
\* 
\* `BY` gives a short proof by telling the prover which facts and definitions to use

AXIOM GCDProperty1 == \A p \in Number : GCD(p, p) = p
\* for every positive integer `p`, the GCD of `p` with itself if `p`
\* `:` separates that restriction

AXIOM GCDProperty2 == \A p, q \in Number : GCD(p, q) = GCD(q, p)
\* for every positive integer `p`, swapping their order does not change their GCD

AXIOM GCDProperty3 == \A p, q \in Number : (p < q) => GCD(p, q) = GCD(p, q-p)
\* for every positive integer `p`, if `p` is smaller than `q`, subtracting `p` from `q` does not change their GCD

THEOREM NextProperty ==
  InductiveInvariant /\ Next => InductiveInvariant'

  <1> SUFFICES ASSUME InductiveInvariant, Next
               PROVE  InductiveInvariant'
    OBVIOUS
  \* `<1>` marks the outer-level proof step
  \* `SUFFICES` means "it is enough to prove the following". it replaces the current goal with a sufficient goal
  \* 
  \* ASSUME InductiveInvariant, Next
  \* makes two facts available throughout the remaining proof:
  \*    - the current state satisfies `InductiveInvariant`
  \*    - the current and next states are related by `Next`
  \* 
  \* PROVE  InductiveInvariant'
  \* sets the remaining goal: show that the invariant holds in the `next state` 
  \* 
  \* OBVIOUS
  \* asks TLAPS to check that this reformulation is sufficient to prove the original goal
  \* it does not prove that the invariant is preserved. the steps below still have to establish that

  <1> USE DEF InductiveInvariant, Next

  <1>1. (x < y) \/ (y < x)
    OBVIOUS

  <1>a. CASE x < y
    <2>1. (y - x \in Number) /\ ~(y < x)
      BY <1>a DEF Number

    <2>2. QED
      BY <1>a, <2>1, GCDProperty3

  <1>b. CASE y < x
    <2>1. (x - y \in Number) /\ ~(x < y)
      BY <1>b DEF Number

    <2>2. GCD(y', x') = GCD(y, x)
      BY <1>b, <2>1, GCDProperty3

    <2>4. QED
      BY <1>b, <2>1, <2>2, GCDProperty2

  <1> QED
    BY <1>1, <1>a, <1>b

THEOREM Correctness == Spec => []ResultCorrect
<1>1 InductiveInvariant /\ UNCHANGED <<x,y>> => InductiveInvariant'
  BY DEF InductiveInvariant
<1>2 Spec => []InductiveInvariant
  BY PTL, InitProperty, NextProperty, <1>1 DEF Spec
<1>3 InductiveInvariant => ResultCorrect
  BY GCDProperty1 DEF InductiveInvariant, ResultCorrect
<1> QED
  BY PTL, <1>2, <1>3

=======================================================
