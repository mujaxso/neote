; Markdown highlighting query for tree-sitter-markdown-inline
; Simplified to only include patterns that definitely work

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

; ====== Fallback for everything else ======
(_) @plain
