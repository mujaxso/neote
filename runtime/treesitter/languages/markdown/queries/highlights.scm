
; Minimal markdown highlighting query that should work with tree-sitter-markdown-inline
; Using only the most common node types

; Try to match headings with various possible node names
[
  (atx_heading)
  (setext_heading)
  (heading)
  (atx_h1_marker)
  (atx_h2_marker)
  (atx_h3_marker)
  (atx_h4_marker)
  (atx_h5_marker)
  (atx_h6_marker)
] @heading

; Emphasis - try common patterns
[
  (emphasis)
  (strong_emphasis)
] @emphasis

; Links
(link) @link
(link_destination) @string

; Code
[
  (inline_code_span)
  (code_span)
] @inline_code

; Block quotes
(block_quote) @block_quote

; Lists
(list_item) @list

; Thematic break
(thematic_break) @thematic_break

; Keep it simple - if the above nodes don't exist, the query won't fail
; because we're using a list of alternatives
