(operator_definition (def_eq) @start) @indent
(function_definition (def_eq) @start) @indent
(let_in "LET" @start "IN" @end) @indent

(_ "(" ")" @end) @indent
(_ "[" "]" @end) @indent
(_ "{" "}" @end) @indent
(_ (langle_bracket) (rangle_bracket) @end) @indent
(step_expr_or_stutter "[" @start "]_" @end) @indent
(step_expr_no_stutter (langle_bracket) @start (rangle_bracket_sub) @end) @indent
