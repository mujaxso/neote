import { useRef, useMemo } from 'react';
import { GUTTER_CONFIG } from './GutterConfig';

interface Props {
  lineCount: number;
  cursorLine: number;
  lineHeight: number;
  outerRef?: React.RefObject<HTMLDivElement>;
}

export function LineNumberGutter({
  lineCount,
  cursorLine,
  lineHeight,
  outerRef,
}: Props) {
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

  // Build all line numbers using static block layout (no absolute positioning)
  const numbers = [];
  for (let i = 0; i < lineCount; i++) {
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
      ref={outerRef}
      className="h-full overflow-y-auto overflow-x-hidden shrink-0 border-r border-[rgba(128,128,128,0.18)]"
      style={{
        width: gutterWidth,
        pointerEvents: 'none',
        scrollbarWidth: 'none',
      }}
    >
      <div
        className="min-w-full"
        style={{
          height: lineCount * lineHeight,
        }}
      >
        {numbers}
      </div>
    </div>
  );
}
