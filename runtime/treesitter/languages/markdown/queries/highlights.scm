; Markdown highlighting query for tree-sitter-markdown-inline
; Enhanced with more specific inline patterns for better highlighting

; ====== Escape sequences ======
(backslash_escape) @escape

; ====== Emphasis and strong emphasis ======
(emphasis) @emphasis
(strong_emphasis) @strong_emphasis

; Emphasis delimiters (capture the markers separately)
(_emphasis_open_star) @emphasis.marker
(_emphasis_open_underscore) @emphasis.marker
(emphasis_delimiter) @emphasis.marker

; ====== Code spans ======
(code_span) @inline_code
(code_span_delimiter) @inline_code.delimiter

; ====== Links ======
(link_text) @link.text
(link_destination) @link.destination
(link_title) @link.title
(link_label) @link.label

; Different link types
(shortcut_link) @link
(full_reference_link) @link
(collapsed_reference_link) @link
(inline_link) @link

; Link brackets and parentheses
"[" @link.bracket
"]" @link.bracket
"(" @link.paren
")" @link.paren

; ====== Images ======
(image) @image
(image_description) @image.description

; Image brackets and exclamation mark
"![" @image.marker

; ====== HTML ======
(html_tag) @html
(_open_tag) @html.tag
(_closing_tag) @html.tag
(_tag_name) @html.tag.name
(_attribute) @html.attribute
(_attribute_name) @html.attribute.name
(_attribute_value) @html.attribute.value
(_html_comment) @html.comment

; ====== Line breaks ======
(hard_line_break) @line_break
(_soft_line_break) @line_break.soft

; ====== Strikethrough ======
(strikethrough) @strikethrough
(_strikethrough_open) @strikethrough.marker

; ====== Autolinks ======
(uri_autolink) @link.autolink
(email_autolink) @link.autolink

; ====== Entity references ======
(entity_reference) @escape
(numeric_character_reference) @escape

; ====== LaTeX ======
(latex_block) @latex
(latex_span_delimiter) @latex.delimiter

; ====== Inline content ======
(_word) @text
(_word_no_digit) @text
(_digits) @number
(_whitespace) @whitespace
(_whitespace_token1) @whitespace
(_whitespace_ge_2) @whitespace

; ====== Punctuation ======
[
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
  "/"
  ":"
  ";"
  "<"
  "="
  ">"
  "?"
  "@"
  "\\"
  "^"
  "_"
  "`"
  "{"
  "|"
  "}"
  "~"
  "("
  ")"
] @punctuation

; ====== Fallback for everything else ======
(_) @plain
