
; Markdown highlighting for tree-sitter-markdown-inline
; Using patterns compatible with the inline variant

; Headings - try different possible node names
[
  (atx_heading)
  (setext_heading)
  (atx_h1_marker)
  (atx_h2_marker)
  (atx_h3_marker)
  (atx_h4_marker)
  (atx_h5_marker)
  (atx_h6_marker)
  (setext_h1_underline)
  (setext_h2_underline)
] @heading

; Emphasis
[
  (emphasis)
  (strong_emphasis)
  (emphasis_marker)
  (strong_emphasis_marker)
] @emphasis

; Links
[
  (link)
  (inline_link)
] @link
[
  (link_text)
  (link_title)
] @link
(link_destination) @string

; Code
[
  (inline_code_span)
  (code_span)
] @inline_code
[
  (code_fence)
  (fenced_code_block)
  (code_fence_content)
] @code_fence

; Block quotes
[
  (block_quote)
  (block_quote_marker)
] @block_quote

; Lists
[
  (list)
  (list_item)
  (list_marker)
] @list
(task_list_marker) @operator

; Thematic break
(thematic_break) @thematic_break

; Paragraph
(paragraph) @paragraph

; Images
(image) @link

; References
[
  (reference_link)
  (reference_definition)
] @link

; Footnotes
[
  (footnote_reference)
  (footnote_definition)
] @link

; Strikethrough
(strikethrough) @emphasis

; Escape sequences
(escape_sequence) @string

; Line breaks
[
  (hard_line_break)
  (soft_line_break)
] @operator

; Tables
[
  (table)
  (table_header)
  (table_row)
  (table_cell)
] @table

; HTML
[
  (html_block)
  (html_inline)
] @html

; URLs and emails
[
  (url)
  (email)
] @string

; Generic text nodes for fallback
[
  (text)
  (inline)
] @plain
