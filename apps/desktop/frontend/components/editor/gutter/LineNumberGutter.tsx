import { useMemo } from 'react';
import { GUTTER_CONFIG } from './GutterConfig';

interface Props {
  lineCount: number;
  cursorLine: number;
  lineHeight: number;
  scrollTop: number;
  containerHeight: number;
}

export function LineNumberGutter({
  lineCount,
  cursorLine,
  lineHeight,
  scrollTop,
  containerHeight,
}: Props) {
  // Effective container height (fallback to 500px until measured)
  const effectiveH = containerHeight > 0 ? containerHeight : 500;

  // Virtualised visible line range (with a small overscan)
  const start = useMemo(() => {
    const scrollLine = Math.floor(scrollTop / lineHeight);
    return Math.max(0, scrollLine - 1);
  }, [scrollTop, lineHeight]);

  const visibleLines = useMemo(
    () => Math.ceil(effectiveH / lineHeight),
    [effectiveH, lineHeight],
  );

  const end = useMemo(() => {
    if (lineCount === 0) return 0;
    return Math.min(lineCount, start + visibleLines + 3);
  }, [start, visibleLines, lineCount]);

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

  // Build the virtualised line‑number list using absolute positioning
  const numbers = [];
  for (let i = start; i < end; i++) {
    const lineNum = i + 1;
    const isCurrent = lineNum === cursorLine;
    numbers.push(
      <div
        key={i}
        style={{
          position: 'absolute',
          top: i * lineHeight,
          left: 0,
          right: 0,
          height: lineHeight,
          lineHeight: `${lineHeight}px`,
          paddingRight: GUTTER_CONFIG.PADDING_RIGHT,
          paddingLeft: GUTTER_CONFIG.PADDING_LEFT,
        }}
        className={`text-right text-sm font-mono tabular-nums select-none ${
          isCurrent
            ? 'text-accent font-semibold bg-accent/15'
            : 'text-editor-foreground opacity-40'
        }`}
      >
        {lineNum}
      </div>,
    );
  }

  return (
    <div
      className="h-full overflow-hidden shrink-0 border-r border-[rgba(128,128,128,0.18)]"
      style={{
        width: gutterWidth,
        pointerEvents: 'none',
      }}
    >
      <div
        className="min-w-full relative"
        style={{
          height: lineCount * lineHeight,
          willChange: 'transform',
          transform: `translateY(-${scrollTop}px)`,
        }}
      >
        {numbers}
      </div>
    </div>
  );
}
