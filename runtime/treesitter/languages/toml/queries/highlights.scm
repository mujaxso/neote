; TOML highlight query (compatible with tree-sitter-toml 0.20)
; This file is kept for reference; the actual query used is from the crate's HIGHLIGHT_QUERY constant

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

; Dates and times (if supported by grammar)
(date_time) @string.special
(local_date) @string.special
(local_time) @string.special
(local_date_time) @string.special

; Tables
(table) @type

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
