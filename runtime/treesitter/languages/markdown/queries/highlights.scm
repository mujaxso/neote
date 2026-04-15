;; Official Tree-sitter Markdown highlight queries
;; Placeholder - users should replace with official queries from:
;; https://github.com/tree-sitter/tree-sitter-markdown/blob/master/queries/highlights.scm

(atx_heading) @heading
(setext_heading) @heading
(emphasis) @emphasis
(strong_emphasis) @strong
(inline_code_span) @inline_code
(code_fence) @code_fence
(block_quote) @blockquote
(list_item) @list
(thematic_break) @thematic_break
(link) @link
(link_text) @link_text
(link_destination) @link_url
(image) @link
(image_description) @link_text
