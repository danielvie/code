; Adapted from tree-sitter-tlaplus queries/highlights.scm.
; See THIRD_PARTY_NOTICES.md for the upstream MIT license.
; Generic captures come first, followed by more specific Zed highlights.

[
  (identifier)
  (identifier_ref)
] @variable

[
  (prefix_op_symbol)
  (infix_op_symbol)
  (postfix_op_symbol)
] @operator

(bound_infix_op symbol: (_) @operator)
(bound_nonfix_op symbol: (_) @operator)
(bound_postfix_op symbol: (_) @operator)
(bound_prefix_op symbol: (_) @operator)

[
  "ACTION"
  "ASSUME"
  "ASSUMPTION"
  "AXIOM"
  "BY"
  "CASE"
  "CHOOSE"
  "CONSTANT"
  "CONSTANTS"
  "COROLLARY"
  "DEF"
  "DEFINE"
  "DEFS"
  "ELSE"
  "EXCEPT"
  "EXTENDS"
  "HAVE"
  "HIDE"
  "IF"
  "IN"
  "INSTANCE"
  "LAMBDA"
  "LEMMA"
  "LET"
  "LOCAL"
  "MODULE"
  "NEW"
  "OBVIOUS"
  "OMITTED"
  "ONLY"
  "OTHER"
  "PICK"
  "PROOF"
  "PROPOSITION"
  "PROVE"
  "QED"
  "RECURSIVE"
  "SF_"
  "STATE"
  "SUFFICES"
  "TAKE"
  "TEMPORAL"
  "THEN"
  "THEOREM"
  "USE"
  "VARIABLE"
  "VARIABLES"
  "WF_"
  "WITH"
  "WITNESS"
] @keyword

[
  (address)
  (all_map_to)
  (assign)
  (case_arrow)
  (case_box)
  (def_eq)
  (exists)
  (forall)
  (gets)
  (label_as)
  (maps_to)
  (set_in)
  (temporal_exists)
  (temporal_forall)
  (bullet_conj)
  (bullet_disj)
  (prev_func_val)
] @operator

[
  (nat_number)
  (real_number)
  (binary_number)
  (octal_number)
  (hex_number)
] @number

(boolean) @boolean

[
  (boolean_set)
  (int_number_set)
  (nat_number_set)
  (real_number_set)
  (string_set)
] @type.builtin

(string) @string
(escape_char) @string.escape

(module name: (identifier) @type)
(module_definition name: (identifier) @type)
(extends (identifier_ref) @type)
(instance (identifier_ref) @type)

(constant_declaration (identifier) @constant)
(constant_declaration (operator_declaration name: (_) @constant))
(variable_declaration (identifier) @variable)

(record_literal (identifier) @property)
(set_of_records (identifier) @property)
(record_value (identifier_ref) @property)
(except_update_record_field (identifier_ref) @property)

(operator_definition parameter: (identifier) @variable.parameter)
(operator_definition (operator_declaration name: (_) @variable.parameter))
(module_definition parameter: (identifier) @variable.parameter)
(quantifier_bound (identifier) @variable.parameter)
(quantifier_bound (tuple_of_identifiers (identifier) @variable.parameter))
(unbounded_quantification (identifier) @variable.parameter)
(choose (identifier) @variable.parameter)
(choose (tuple_of_identifiers (identifier) @variable.parameter))
(lambda (identifier) @variable.parameter)

(operator_definition name: (_) @function)
(function_definition name: (identifier) @function)
(recursive_declaration (identifier) @function)
(recursive_declaration (operator_declaration name: (_) @function))
(theorem name: (identifier) @constant)
(assumption name: (identifier) @constant)

[
  (langle_bracket)
  (rangle_bracket)
  (rangle_bracket_sub)
  "{" "}"
  "[" "]" "]_"
  "(" ")"
] @punctuation.bracket

["," ":" "." "!" ";" (placeholder)] @punctuation.delimiter
[(header_line) (double_line)] @punctuation.special

(proof_step_id "<" @punctuation.bracket ">" @punctuation.bracket)
(proof_step_ref "<" @punctuation.bracket ">" @punctuation.bracket)
(proof_step_id (level) @tag)
(proof_step_id (name) @tag)
(proof_step_ref (level) @tag)
(proof_step_ref (name) @tag)
(label name: (_) @label)

(comment) @comment
(block_comment "(*" @comment "*)" @comment)
(block_comment_text) @comment
(single_line) @comment
