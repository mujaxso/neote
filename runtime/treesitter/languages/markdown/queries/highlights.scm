; Markdown highlighting query for tree-sitter-markdown-inline
; Using node types we know exist from debug output

; Escape sequences and references
(_backslash_escape) @string.escape
(entity_reference) @string
(numeric_character_reference) @string

; Punctuation characters (as individual node types)
[
  "["
  "]"
  "<"
  ">"
  "!"
  "\""
  "#"
  "$"
  "%"
  "&"
  "'"
  "*"
  "+"
  ","
  "-"
  "."
] @operator

; Fallback for everything else
(_) @plain
