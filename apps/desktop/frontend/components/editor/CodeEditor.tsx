import { useEffect, useRef, useState, useCallback } from 'react';
import { cn } from '@/lib/utils';
import { useTabsStore } from '@/features/tabs/store';
import { LineNumberGutter } from './gutter/LineNumberGutter';
import { GUTTER_CONFIG } from './gutter/GutterConfig';
import { computeGutterWidth } from './gutter/GutterLayout';
import { FONT_TOKENS } from '@/lib/theme/font-tokens';
import { invoke } from '@tauri-apps/api/core';

/* ------------------------------------------------------------------ */
/*  Custom hook: fetch syntax highlights for the current viewport    */
/* ------------------------------------------------------------------ */
interface HighlightSpan {
  start: number;
  end: number;
  token_type: string;
}
interface HighlightLine {
  index: number;
  text: string;
  spans: HighlightSpan[];
}
interface HighlightResponse {
  lines: HighlightLine[];
}

function useHighlight(
  documentId: string | null,
  startLine: number,
  count: number,
  enabled: boolean,
) {
  const [lines, setLines] = useState<HighlightLine[]>([]);

  useEffect(() => {
    if (!documentId || !enabled || count === 0) {
      setLines([]);
      return;
    }

    let cancelled = false;

    async function fetch() {
      try {
        const res: HighlightResponse = await invoke('highlight_document', {
          documentId,
          startLine,
          count,
        });
        if (!cancelled) {
          setLines(res.lines);
        }
      } catch (err) {
        console.warn('highlight_document failed:', err);
        if (!cancelled) setLines([]);
      }
    }

    fetch();
    return () => {
      cancelled = true;
    };
  }, [documentId, startLine, count, enabled]);

  return lines;
}

/* ------------------------------------------------------------------ */
/*  Simple token‑type → CSS class mapping (can be extended later)     */
/* ------------------------------------------------------------------ */
const tokenStyleMap: Record<string, string> = {
  keyword: 'text-purple-400',
  string: 'text-green-400',
  comment: 'text-gray-500 italic',
  function: 'text-blue-400',
  type: 'text-cyan-400',
  variable: 'text-orange-300',
  constant: 'text-yellow-300',
  number: 'text-pink-400',
  operator: 'text-red-300',
  punctuation: 'text-slate-400',
};

function renderSpans(spans: HighlightSpan[], lineText: string) {
  if (spans.length === 0) {
    return lineText;
  }

  const segments: React.ReactNode[] = [];
  let last = 0;
  for (const sp of spans) {
    if (sp.start > last) {
      segments.push(lineText.slice(last, sp.start));
    }
    const tokenClass = tokenStyleMap[sp.token_type] ?? '';
    segments.push(
      <span key={`s${sp.start}`} className={tokenClass}>
        {lineText.slice(sp.start, sp.end)}
      </span>,
    );
    last = sp.end;
  }
  if (last < lineText.length) {
    segments.push(lineText.slice(last));
  }
  return segments;
}

/* ------------------------------------------------------------------ */
/*  CodeEditor component                                              */
/* ------------------------------------------------------------------ */

interface CodeEditorProps {
  initialValue: string;
  onChange: (value: string) => void;
  filePath?: string;
  language?: string;
  readOnly?: boolean;
  className?: string;
  contentTruncated?: boolean;
}

/** Maximum characters used for the legacy length‑check (kept for backward compat). */
const TRUNCATE_CHARS = 50_000;

/** Fast line‑counting (O(n), not used for large files where the gutter is hidden). */
function fastLineCount(text: string): number {
  let lines = 1;
  const len = text.length;
  let i = 0;
  while (i < len) {
    if (text.charCodeAt(i) === 10) lines++;
    i++;
  }
  return lines;
}

export function CodeEditor({
  initialValue,
  onChange,
  filePath,
  readOnly = false,
  className,
  contentTruncated,
}: CodeEditorProps) {
  // ── Local state ──────────────────────────────────────────────────
  const [value, setValue] = useState(initialValue);
  const textareaRef = useRef<HTMLTextAreaElement>(null);
  const highlightLayerRef = useRef<HTMLDivElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const [scrollTop, setScrollTop] = useState(0);
  const [cursorLine, setCursorLine] = useState(1);

  const largeFile = contentTruncated ?? (initialValue.length >= TRUNCATE_CHARS);

  // Keep local value in sync with prop changes (e.g. when switching tabs)
  useEffect(() => {
    setValue(initialValue);
  }, [initialValue]);

  // ── Scroll & selection ───────────────────────────────────────────
  const syncScroll = useCallback(() => {
    const ta = textareaRef.current;
    const hl = highlightLayerRef.current;
    if (ta && hl) {
      hl.scrollTop = ta.scrollTop;
      hl.scrollLeft = ta.scrollLeft;
    }
    setScrollTop(ta?.scrollTop ?? 0);
  }, []);

  const handleTextareaScroll = useCallback(() => {
    syncScroll();
  }, [syncScroll]);

  const handleSelectionChange = useCallback(() => {
    const ta = textareaRef.current;
    if (!ta) return;
    const sel = ta.selectionStart;
    const before = value.slice(0, sel).match(/\n/g);
    const line = before ? before.length + 1 : 1;
    setCursorLine(line);
  }, [value]);

  const handleChange = useCallback(
    (e: React.ChangeEvent<HTMLTextAreaElement>) => {
      if (readOnly) return;
      const newVal = e.target.value;
      setValue(newVal);
      onChange(newVal);
      if (filePath) {
        useTabsStore.getState().markDirty(filePath);
      }
      const selStart = e.target.selectionStart;
      const before = newVal.slice(0, selStart).match(/\n/g);
      const line = before ? before.length + 1 : 1;
      setCursorLine(line);
    },
    [onChange, readOnly, filePath],
  );

  // ── Visible range for highlight fetching ─────────────────────────
  const lineHeight = GUTTER_CONFIG.LINE_HEIGHT;
  const containerHeight = containerRef.current?.clientHeight ?? 0;
  const visibleStartLine = Math.floor(scrollTop / lineHeight);
  const visibleCount = Math.ceil((containerHeight + lineHeight) / lineHeight) * 2; // generous overscan

  const highlightsEnabled = !largeFile && !!filePath;

  const highlightedLines = useHighlight(
    filePath ?? null,
    visibleStartLine,
    visibleCount,
    highlightsEnabled,
  );

  // ── Derived metrics (only needed when gutter is shown) ───────────
  const totalLines = largeFile ? 0 : fastLineCount(value);
  const gutterWidth = largeFile ? 0 : computeGutterWidth(totalLines);

  // Large files are always read‑only (content is truncated on the backend)
  const effectiveReadOnly = readOnly || largeFile;

  /* ---------- Layout ---------- */
  return (
    <div ref={containerRef} className={cn('flex h-full', className)}>
      {/* Gutter – disabled for large files */}
      {!largeFile && (
        <div className="shrink-0 relative overflow-hidden" style={{ width: gutterWidth }}>
          <LineNumberGutter
            lineCount={totalLines}
            cursorLine={cursorLine}
            lineHeight={lineHeight}
            scrollTop={scrollTop}
            containerHeight={containerHeight}
          />
        </div>
      )}

      {/* Scrollable text area with syntax overlay */}
      <div className="flex-1 flex flex-col overflow-hidden relative">
        {largeFile && (
          <div className="text-muted-foreground text-xs p-1 bg-muted/80 shrink-0">
            File &gt; 100 MB – read‑only preview (first 50 000 characters shown)
          </div>
        )}

        {/* Highlighted background layer */}
        {highlightsEnabled && (
          <div
            ref={highlightLayerRef}
            aria-hidden="true"
            className="absolute inset-0 overflow-hidden pointer-events-none font-mono text-sm whitespace-pre select-none"
            style={{
              lineHeight: `${lineHeight}px`,
              fontFamily: FONT_TOKENS.editor,
              whiteSpace: 'pre',
              overflowWrap: 'normal',
              color: 'transparent',
            }}
          >
            <div style={{ minWidth: '100%' }}>
              {Array.from({ length: totalLines }).map((_, idx) => {
                const hl = highlightedLines.find((l) => l.index === idx);
                return (
                  <div key={idx}>
                    {hl ? renderSpans(hl.spans, hl.text) : idx < totalLines ? value.split('\n')[idx] ?? '' : ''}
                  </div>
                );
              })}
            </div>
          </div>
        )}

        {/* Editable textarea */}
        <textarea
          ref={textareaRef}
          className="flex-1 resize-none outline-none bg-transparent text-editor-foreground font-mono text-sm p-0 overflow-auto scrollbar-none relative z-10"
          style={{
            lineHeight: `${lineHeight}px`,
            fontFamily: FONT_TOKENS.editor,
            whiteSpace: 'pre',
            overflowWrap: 'normal',
            wrap: 'off',
            caretColor: effectiveReadOnly ? 'transparent' : undefined,
          }}
          value={value}
          readOnly={effectiveReadOnly}
          onChange={handleChange}
          onScroll={handleTextareaScroll}
          onSelect={handleSelectionChange}
          onClick={() => textareaRef.current?.focus()}
          spellCheck={false}
          autoComplete="off"
          autoCorrect="off"
        />
      </div>
    </div>
  );
}
