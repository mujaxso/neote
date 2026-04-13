; TOML highlight queries from the official tree-sitter-toml grammar
; This ensures correct node types for the exact grammar version (0.20)

; Comments
(comment) @comment

; Strings
(string) @string
(escape_sequence) @escape

; Numbers
(integer) @number
(float) @number

; Booleans
(boolean) @boolean

; Dates and times
(date_time) @string.special
(local_date) @string.special
(local_time) @string.special
(local_date_time) @string.special

; Tables
(table) @type
(table_array) @type

; Keys
(key) @property
(dotted_key) @property

; Punctuation
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
"=" @operator
"," @punctuation.delimiter
"." @punctuation.delimiter
"+" @operator
"-" @operator
