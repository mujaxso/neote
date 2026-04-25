import React, { useRef, useLayoutEffect, useState, useMemo } from 'react';
import { GutterModel } from './GutterModel';
import { GutterView } from './GutterView';

interface Props {
  lineCount: number;
  cursorLine: number;
  lineHeight: number;
  scrollTop: number;
}

/**
 * Thin wrapper that measures container height and creates a GutterModel.
 *
 * This component is the public API for the gutter subsystem.
 * All layout logic lives in `GutterModel`; all rendering lives in `GutterView`.
 */
export const LineNumberGutter: React.FC<Props> = ({
  lineCount,
  cursorLine,
  lineHeight,
  scrollTop,
}) => {
  const outerRef = useRef<HTMLDivElement>(null);
  const [containerHeight, setContainerHeight] = useState(0);

  // Measure container height on mount and on resize
  useLayoutEffect(() => {
    const updateHeight = () => {
      if (outerRef.current) {
        setContainerHeight(outerRef.current.clientHeight);
      }
    };
    updateHeight();
    const observer = new ResizeObserver(updateHeight);
    if (outerRef.current) {
      observer.observe(outerRef.current);
    }
    return () => observer.disconnect();
  }, []);

  // Create the model (memoized based on inputs)
  const model = useMemo(
    () =>
      new GutterModel(
        scrollTop,
        lineHeight,
        lineCount,
        containerHeight,
        cursorLine,
        3, // overscan
      ),
    [scrollTop, lineHeight, lineCount, containerHeight, cursorLine],
  );

  // Early return for empty document
  if (lineCount === 0) {
    return (
      <div
        ref={outerRef}
        className="h-full shrink-0 border-r border-[rgba(128,128,128,0.18)]"
        style={{
          width: model.width,
          pointerEvents: 'none',
          position: 'relative',
          overflow: 'visible',
        }}
      />
    );
  }

  return (
    <div
      ref={outerRef}
      className="h-full shrink-0 border-r border-[rgba(128,128,128,0.18)]"
      style={{
        width: model.width,
        pointerEvents: 'none',
        position: 'relative',
        overflow: 'visible',
      }}
    >
      <GutterView model={model} />
    </div>
  );
};
