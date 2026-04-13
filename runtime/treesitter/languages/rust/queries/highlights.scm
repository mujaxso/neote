					; Rust highlight queries for Tree‑sitter (compatible with tree‑sitter‑rust 0.20)

; Comments
(block_comment) @comment
(line_comment) @comment

; Strings
(string_literal) @string
(raw_string_literal) @string
(char_literal) @string

; Keywords - minimal set to avoid parsing errors
[
  "fn"
  "let"
  "if"
  "else"
  "for"
  "while"
  "match"
  "struct"
  "enum"
  "impl"
  "trait"
  "use"
  "pub"
  "mod"
  "type"
  "const"
  "static"
  "unsafe"
  "return"
  "break"
  "continue"
  "as"
  "in"
  "where"
  "loop"
  "move"
  "ref"
  "mut"
  "self"
  "Self"
  "super"
  "extern"
  "crate"
  "true"
  "false"
  "async"
  "await"
  "dyn"
] @keyword

; Function definitions
(function_item
  name: (identifier) @function)

; Type definitions
(type_identifier) @type

; Variables
(identifier) @variable

; Constants
(const_item
  name: (identifier) @constant)

; Operators
[
  "+" "-" "*" "/" "%"
  "=" "==" "!=" "<" "<=" ">" ">="
  "!" "&&" "||"
  "&" "|" "^" "<<" ">>"
  "+=" "-=" "*=" "/=" "%="
  "&=" "|=" "^=" "<<=" ">>="
  ".." "..=" "->" "=>"
] @operator

; Punctuation
[
  "," ";" ":" "::" "." "(" ")" "[" "]" "{" "}"
] @punctuation

; Literals
(boolean_literal) @constant.builtin
(integer_literal) @number
(float_literal) @number
