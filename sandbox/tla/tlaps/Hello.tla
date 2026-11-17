---------------- MODULE Hello ----------------
\* .\tlapm.cmd --nofp --toolbox 0 0 .\Hello.tla
\* $LASTEXITCODE
\* 
\* --nofp : don't reuse cached proof results
\* --toolbox : make TLAPS print structured progress reports

\* "for every integer `n`, adding 1 gives a number greater than `n`"

EXTENDS Integers, TLAPS

THEOREM AddingOneMakesItBigger == \A n \in Int : n + 1 > n
BY SMT
\* prove this using a SMT solver
\* SMT tells TLAPS to use an automated solver. It stands for 
\* Satisfiability Modulo Theories, meaning logical reasoning with built-in
\* support for subjects such as arithmetic

==============================================
