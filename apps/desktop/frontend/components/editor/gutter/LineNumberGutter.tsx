import { useRef, useMemo, useLayoutEffect } from 'react';
import { GUTTER_CONFIG } from './GutterConfig';

interface Props {
  lineCount: number;
  cursorLine: number;
  scrollTop: number;
  containerHeight: number;
  lineHeight: number;
}

export function LineNumberGutter({
  lineCount,
  cursorLine,
  scrollTop,
  containerHeight,
  lineHeight,
}: Props) {
  const ref = useRef<HTMLDivElement>(null);

  // Virtualized visible line range
  const start = useMemo(() => {
    if (containerHeight <= 0) return 0;
    const scrollLine = Math.floor(scrollTop / lineHeight);
    return Math.max(0, scrollLine - 1);
  }, [scrollTop, containerHeight, lineHeight]);

  const end = useMemo(() => {
    if (containerHeight <= 0 || lineCount === 0) return 0;
    const visibleLines = Math.ceil(containerHeight / lineHeight);
    const scrollLine = Math.floor(scrollTop / lineHeight);
    return Math.min(lineCount, start + visibleLines + 3);
  }, [start, scrollTop, containerHeight, lineHeight, lineCount]);

  // Gutter width based on number of digits of the last line
  const gutterWidth = useMemo(() => {
    const digits = String(lineCount).length;
    return Math.max(
      GUTTER_CONFIG.MIN_WIDTH,
      digits * GUTTER_CONFIG.DIGIT_WIDTH +
        GUTTER_CONFIG.PADDING_LEFT +
        GUTTER_CONFIG.PADDING_RIGHT,
    );
  }, [lineCount]);

  // Mirror the editor scroll position onto the gutter DOM element
  useLayoutEffect(() => {
    if (ref.current) {
      ref.current.scrollTop = scrollTop;
    }
  }, [scrollTop]);

  // Build the virtualized line‑number list
  const numbers = [];
  for (let i = start; i < end; i++) {
    const lineNum = i + 1;
    const isCurrent = lineNum === cursorLine;
    numbers.push(
      <div
        key={i}
        style={{
          height: lineHeight,
          lineHeight: `${lineHeight}px`,
          paddingRight: GUTTER_CONFIG.PADDING_RIGHT,
          paddingLeft: GUTTER_CONFIG.PADDING_LEFT,
        }}
        className={`relative text-right text-sm font-mono tabular-nums select-none ${
          isCurrent
            ? 'text-accent font-semibold'       // current line marker
            : 'text-editor-foreground opacity-40' // quiet, technical lines
        }`}
      >
        {lineNum}
      </div>,
    );
  }

  return (
    <div
      ref={ref}
      className="overflow-hidden shrink-0 border-r border-[rgba(128,128,128,0.18)]"
      style={{
        width: gutterWidth,
        paddingTop: start * lineHeight,
        scrollbarWidth: 'none',
        msOverflowStyle: 'none',
      }}
    >
      <div
        className="min-w-full"
        style={{ height: lineCount * lineHeight - start * lineHeight }}
      >
        {numbers}
      </div>
    </div>
  );
}
