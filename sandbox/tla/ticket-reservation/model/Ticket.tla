---------------------------- MODULE Ticket ----------------------------
EXTENDS TLC

CONSTANTS Customers, NoCustomer, BuggyConfirm
ASSUME /\ Customers # {}
       /\ NoCustomer \notin Customers
       /\ BuggyConfirm \in BOOLEAN

VARIABLES status, owner, buyer
vars == <<status, owner, buyer>>

Init ==
    /\ status = "available"
    /\ owner = NoCustomer
    /\ buyer = NoCustomer

Reserve(c) ==
    /\ status = "available"
    /\ status' = "reserved"
    /\ owner' = c
    /\ UNCHANGED buyer

Confirm(c) ==
    /\ status = "reserved"
    \* The broken version forgets to authenticate the reservation holder.
    /\ (BuggyConfirm \/ c = owner)
    /\ status' = "sold"
    /\ buyer' = c
    \* Keep the previous reservation owner as evidence for the invariant.
    /\ UNCHANGED owner

Cancel(c) ==
    /\ status = "reserved"
    /\ c = owner
    /\ status' = "available"
    /\ owner' = NoCustomer
    /\ UNCHANGED buyer

Expire ==
    /\ status = "reserved"
    /\ status' = "available"
    /\ owner' = NoCustomer
    /\ UNCHANGED buyer

\* A sold ticket is intentionally terminal, not an accidental deadlock.
SoldIdle ==
    /\ status = "sold"
    /\ UNCHANGED vars

Next ==
    \/ \E c \in Customers : Reserve(c) \/ Confirm(c) \/ Cancel(c)
    \/ Expire
    \/ SoldIdle

Spec == Init /\ [][Next]_vars

TypeOK ==
    /\ status \in {"available", "reserved", "sold"}
    /\ owner \in Customers \cup {NoCustomer}
    /\ buyer \in Customers \cup {NoCustomer}

StateConsistency ==
    /\ (status = "available" => owner = NoCustomer /\ buyer = NoCustomer)
    /\ (status = "reserved" => owner \in Customers /\ buyer = NoCustomer)
    /\ (status = "sold" => owner \in Customers /\ buyer \in Customers)

OnlyOwnerCanBuy == status = "sold" => buyer = owner

=============================================================================
