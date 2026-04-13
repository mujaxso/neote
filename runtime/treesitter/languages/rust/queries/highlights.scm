; Rust highlight queries for Tree‑sitter (compatible with tree‑sitter‑rust 0.20)

; Comments
(line_comment) @comment
(block_comment) @comment

; Strings
(string_literal) @string
(raw_string_literal) @string
(char_literal) @string

; Keywords
"as" @keyword
"async" @keyword
"await" @keyword
"break" @keyword
"const" @keyword
"continue" @keyword
"crate" @keyword
"dyn" @keyword
"else" @keyword
"enum" @keyword
"extern" @keyword
"fn" @keyword
"for" @keyword
"if" @keyword
"impl" @keyword
"in" @keyword
"let" @keyword
"loop" @keyword
"match" @keyword
"mod" @keyword
"move" @keyword
"mut" @keyword
"pub" @keyword
"ref" @keyword
"return" @keyword
"static" @keyword
"struct" @keyword
"super" @keyword
"trait" @keyword
"type" @keyword
"union" @keyword
"unsafe" @keyword
"use" @keyword
"where" @keyword
"while" @keyword
"yield" @keyword

; Built-in types
"bool" @type
"char" @type
"f32" @type
"f64" @type
"i8" @type
"i16" @type
"i32" @type
"i64" @type
"i128" @type
"isize" @type
"str" @type
"u8" @type
"u16" @type
"u32" @type
"u64" @type
"u128" @type
"usize" @type

; Function definitions
(function_item (identifier) @function)
(function_signature_item (identifier) @function)

; Function calls
(call_expression
  function: (identifier) @function)
(call_expression
  function: (field_expression
    field: (field_identifier) @function))

; Type definitions
(type_identifier) @type
(primitive_type) @type

; Variables
(identifier) @variable

; Constants
(const_item (identifier) @constant)

; Operators
"+" @operator
"-" @operator
"*" @operator
"/" @operator
"%" @operator
"=" @operator
"==" @operator
"!=" @operator
"<" @operator
"<=" @operator
">" @operator
">=" @operator
"!" @operator
"&&" @operator
"||" @operator
"&" @operator
"|" @operator
"^" @operator
"<<" @operator
">>" @operator
"+=" @operator
"-=" @operator
"*=" @operator
"/=" @operator
"%=" @operator
"&=" @operator
"|=" @operator
"^=" @operator
"<<=" @operator
">>=" @operator
".." @operator
"..=" @operator
"->" @operator
"=>" @operator

; Punctuation
"," @operator
";" @operator
":" @operator
"::" @operator
"." @operator
"(" @operator
")" @operator
"[" @operator
"]" @operator
"{" @operator
"}" @operator

; Literals
(boolean_literal) @constant
(integer_literal) @constant
(float_literal) @constant

; Attributes
(attribute_item) @attribute
(inner_attribute_item) @attribute

; Macros
(macro_invocation
  macro: (identifier) @function)
