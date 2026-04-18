; TOML highlight query (compatible with tree-sitter-toml 0.20.0)

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

; Tables
(table) @type
(array_table) @type
(inline_table) @type

; Keys
(bare_key) @property
(quoted_key) @property
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
