; TOML highlight queries for Tree‑sitter
; Based on tree-sitter-toml grammar version 0.20

(comment) @comment

(string) @string
(escape_sequence) @string.escape

(integer) @number
(float) @number

(boolean) @boolean

; Keys
(bare_key) @property

; Table headers
(table_header (bare_key) @type)
(table_array_header (bare_key) @type)

; Punctuation
"=" @operator
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
"," @punctuation.delimiter
