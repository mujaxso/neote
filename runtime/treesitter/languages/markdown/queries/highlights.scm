; Markdown highlighting query for tree-sitter-markdown-inline
; Enhanced to better capture headings and other elements

; ====== Escape sequences ======
(backslash_escape) @escape

; ====== Emphasis and strong emphasis ======
(emphasis) @emphasis
(strong_emphasis) @strong_emphasis

; ====== Code spans ======
(code_span) @inline_code

; ====== Links ======
(link_text) @link.text
(link_destination) @link.destination
(link_title) @link.title

; Different link types
(shortcut_link) @link
(full_reference_link) @link
(collapsed_reference_link) @link
(inline_link) @link

; ====== Images ======
(image) @image
(image_description) @image.description

; ====== HTML ======
(html_tag) @html

; ====== Line breaks ======
(hard_line_break) @line_break

; ====== Strikethrough ======
(strikethrough) @strikethrough

; ====== Autolinks ======
(uri_autolink) @link.autolink
(email_autolink) @link.autolink

; ====== Entity references ======
(entity_reference) @escape
(numeric_character_reference) @escape

; ====== LaTeX ======
(latex_block) @latex

; ====== Heading markers and content ======
; Capture # characters as heading markers - use a node type that might work
; Try using the actual node type for "#" from debug output
"#" @heading.marker

; Also try to capture multiple # characters
[
  "#"
  "##"
  "###"
  "####"
  "#####"
  "######"
] @heading.marker

; Try to capture text that might be in headings
; Since the inline grammar doesn't have proper heading nodes,
; we'll capture inline content that could be part of headings
(inline) @heading.content

; ====== Fallback for everything else ======
(_) @plain
