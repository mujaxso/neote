; TOML highlight queries for Tree‑sitter
; Based on tree-sitter-toml grammar version 0.20

; Comments
(comment) @comment

; Strings and escapes
(string) @string
(escape_sequence) @string.escape

; Numbers
(integer) @number
(float) @number

; Constants
(boolean) @constant.builtin
; date_time node type may not exist in this version of the grammar
; (date_time) @constant.builtin
; Try alternative node types for date/time values:
; (date-time) @constant.builtin
; (datetime) @constant.builtin
; Try alternative node types for date/time values:
; (date-time) @constant.builtin
; (datetime) @constant.builtin
; If date/time values need highlighting, try one of these node types:
; (date-time) @constant.builtin
; (datetime) @constant.builtin

; Tables
(table_header (identifier) @type)
(table (identifier) @type)
(table_array_element (identifier) @type)

; Keys in key-value pairs
(pair (key) @property)
(pair (bare_key) @property)
(pair (quoted_key) @property)
(pair (dotted_key (identifier) @property))

; Array and inline table delimiters
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket

; Operators
"=" @operator
"," @punctuation.delimiter
"." @punctuation.delimiter
"+" @operator
"-" @operator
