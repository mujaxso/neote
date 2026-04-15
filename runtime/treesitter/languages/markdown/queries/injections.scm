; Markdown injections for fenced code blocks
(fenced_code_block
  (info_string) @injection.language
  (code_fence_content) @injection.content)

; HTML injection if html_tag exists
; ((html_tag) @injection.content
;   (#set! injection.language "html"))

; LaTeX injection if latex_block exists  
; ((latex_block) @injection.content
;   (#set! injection.language "latex"))

; Support for inline code if needed
; (code_span) @injection.content
