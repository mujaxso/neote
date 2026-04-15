
; Minimal markdown highlighting query
; Using wildcard patterns that should work with any grammar

; Match any node - this will always compile
(_) @plain

; Try to match paragraph if it exists
(paragraph) @paragraph

; The query will compile even if paragraph doesn't exist
; because (_) matches everything
