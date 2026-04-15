; Markdown highlighting query for tree-sitter-markdown-inline
; Based on actual node types from debug output

; Escape sequences
(backslash_escape) @string.escape

; Emphasis
(emphasis) @emphasis
(strong_emphasis) @strong_emphasis

; Code
(code_span) @inline_code

; Links
(link_text) @link
(link_destination) @string
(link_title) @string
(shortcut_link) @link
(full_reference_link) @link
(collapsed_reference_link) @link
(inline_link) @link

; Images
(image) @link
(image_description) @link

; HTML
(html_tag) @html

; Line breaks
(hard_line_break) @operator

; Strikethrough
(strikethrough) @emphasis

; Autolinks
(uri_autolink) @string
(email_autolink) @string

; Fallback for everything else
(_) @plain
