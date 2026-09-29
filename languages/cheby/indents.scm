(_ "{" "}" @end) @indent
(_ "(" ")" @end) @indent
(_ "[" "]" @end) @indent
(_ "<" ">" @end) @indent
(turbofish "::<" ">" @end) @indent

[
  (let_statement)
  (const_declaration)
  (binary_expression)
] @indent
