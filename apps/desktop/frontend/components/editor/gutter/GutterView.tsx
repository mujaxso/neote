//! React view component for the line-number gutter.
//! Renders only visible line numbers using the GutterModel.

import React, { useMemo } from 'react';
import { GutterModel, VisibleLineInfo } from './GutterModel';
import { GUTTER_CONFIG } from './GutterConfig';

interface GutterViewProps {
  model: GutterModel;
  className?: string;
}

// ── Single line number element (memoized) ────────────────────────────

const LineNumber: React.FC<{
  lineIndex: number;
  lineNum: number;
  isCurrent: boolean;
  lineHeight: number;
}> = React.memo(({ lineIndex, lineNum, isCurrent, lineHeight }) => {
  const style: React.CSSProperties = {
    position: 'absolute',
    top: lineIndex * lineHeight,
    left: 0,
    right: 0,
    height: lineHeight,
    lineHeight: `${lineHeight}px`,
    paddingRight: GUTTER_CONFIG.PADDING_RIGHT,
    paddingLeft: GUTTER_CONFIG.PADDING_LEFT,
  };

  const className = [
    'text-right text-sm font-mono tabular-nums select-none',
    isCurrent
      ? 'text-accent font-semibold bg-accent/15'
      : 'text-editor-foreground opacity-40',
  ].join(' ');

  return (
    <div style={style} className={className}>
      {lineNum}
    </div>
  );
});

LineNumber.displayName = 'LineNumber';

// ── Gutter view ──────────────────────────────────────────────────────

export const GutterView: React.FC<GutterViewProps> = React.memo(
  ({ model, className }) => {
    const { width, visibleLines, totalHeight } = model.layout;

    const lineNumbers = useMemo(() => {
      return visibleLines.map((info) => (
        <LineNumber
          key={info.lineIndex}
          lineIndex={info.lineIndex}
          lineNum={info.lineNum}
          isCurrent={info.isCurrent}
          lineHeight={model.lineHeight}
        />
      ));
    }, [visibleLines, model.lineHeight]);

    const containerStyle: React.CSSProperties = {
      width,
      pointerEvents: 'none',
      position: 'relative',
      overflow: 'visible',
      height: '100%',
    };

    const innerStyle: React.CSSProperties = {
      position: 'relative',
      height: totalHeight,
      width: '100%',
      overflow: 'visible',
    };

    return (
      <div
        className={`h-full shrink-0 border-r border-[rgba(128,128,128,0.18)] ${className || ''}`}
        style={containerStyle}
      >
        <div style={innerStyle}>{lineNumbers}</div>
      </div>
    );
  },
);

GutterView.displayName = 'GutterView';
