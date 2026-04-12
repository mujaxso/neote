; Rust syntax highlighting queries for Tree-sitter

(comment) @comment

(string_literal) @string
(raw_string_literal) @string

"fn" @keyword
"let" @keyword
"mut" @keyword
"pub" @keyword
"use" @keyword
"mod" @keyword
"struct" @keyword
"enum" @keyword
"impl" @keyword
"trait" @keyword
"type" @keyword
"where" @keyword
"match" @keyword
"if" @keyword
"else" @keyword
"for" @keyword
"in" @keyword
"while" @keyword
"loop" @keyword
"return" @keyword
"break" @keyword
"continue" @keyword
"self" @keyword
"Self" @keyword
"as" @keyword
"async" @keyword
"await" @keyword
"const" @keyword
"unsafe" @keyword
"extern" @keyword
"crate" @keyword
"super" @keyword
"dyn" @keyword
"move" @keyword
"ref" @keyword
"static" @keyword

(function_item name: (identifier) @function)
(call_expression function: (identifier) @function)
(call_expression function: (field_expression field: (field_identifier) @function))

(type_identifier) @type
(primitive_type) @type

(identifier) @variable

; Match constants (uppercase identifiers)
((identifier) @constant
 (#match? @constant "^[A-Z][A-Z_]*$"))

(attribute_item) @attribute

(binary_expression operator: _ @operator)
(unary_expression operator: _ @operator)
(assignment_expression operator: _ @operator)

(integer_literal) @number
(float_literal) @number
