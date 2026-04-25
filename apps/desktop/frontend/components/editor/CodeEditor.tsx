import { useEffect, useRef, useState, useCallback } from 'react';
import { cn } from '@/lib/utils';
import { useTabsStore } from '@/features/tabs/store';
import { LineNumberGutter } from './gutter/LineNumberGutter';
import { GUTTER_CONFIG } from './gutter/GutterConfig';

interface CodeEditorProps {
  initialValue: string;
  onChange: (value: string) => void;
  filePath?: string;
  language?: string;
  readOnly?: boolean;
  className?: string;
}

function countLines(text: string): number {
  // Count linefeed occurrences, add one for the last line (even if empty)
  let lines = 1;
  for (let i = 0; i < text.length; i++) {
    if (text.charCodeAt(i) === 10) lines++;
  }
  return lines;
}

export function CodeEditor({
  initialValue,
  onChange,
  filePath,
  language = 'plaintext',
  readOnly = false,
  className,
}: CodeEditorProps) {
  const [value, setValue] = useState(initialValue);
  const initialRef = useRef(initialValue);

  // Refs for scroll synchronisation
  const textAreaRef = useRef<HTMLTextAreaElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const gutterInnerRef = useRef<HTMLDivElement>(null);

  // Editor state we need to expose to the gutter (virtualisation)
  const [cursorLine, setCursorLine] = useState(() => {
    // compute initial cursor line from text (starts at position 0)
    const beforeNewlines = initialValue.slice(0, 0).match(/\n/g);
    return beforeNewlines ? beforeNewlines.length + 1 : 1;
  });
  const [containerHeight, setContainerHeight] = useState(0);
  const [scrollTop, setScrollTop] = useState(0);

  const scrollTopRafId = useRef<number | null>(null);

  // lineCount is stored in state so it is **not** recomputed on every render
  const [lineCount, setLineCount] = useState(() => countLines(initialValue));

  // Update lineCount only when the text value changes
  const valueRef = useRef(value);
  useEffect(() => {
    if (valueRef.current !== value) {
      valueRef.current = value;
      setLineCount(countLines(value));
    }
  }, [value]);

  // Sync when the parent supplies a new `initialValue`
  useEffect(() => {
    if (initialRef.current !== initialValue) {
      initialRef.current = initialValue;
      setValue(initialValue);
      setLineCount(countLines(initialValue));
    }
  }, [initialValue]);

  // Inject CSS that hides native scrollbars (professional IDE look)
  useEffect(() => {
    const style = document.createElement('style');
    style.innerHTML = `
      .hide-scrollbar::-webkit-scrollbar {
        display: none;
      }
    `;
    document.head.appendChild(style);
    return () => {
      document.head.removeChild(style);
    };
  }, []);

  // Re‑measure container height and update gutter on mount / resize
  const measureContainer = useCallback(() => {
    if (containerRef.current) {
      setContainerHeight(containerRef.current.clientHeight);
    }
  }, []);

  useEffect(() => {
    measureContainer();
    const observer = new ResizeObserver(measureContainer);
    if (containerRef.current) {
      observer.observe(containerRef.current);
    }
    return () => observer.disconnect();
  }, [measureContainer]);

  const lineHeight = GUTTER_CONFIG.LINE_HEIGHT;

  const handleScroll = useCallback(() => {
    const ta = textAreaRef.current;
    if (!ta) return;
    const st = ta.scrollTop;

    // Apply pixel‑perfect transform to the gutter’s inner container (no React re‑render)
    if (gutterInnerRef.current) {
      gutterInnerRef.current.style.transform = `translateY(-${st}px)`;
    }

    // Throttle scrollTop state update to at most once per frame (needed for virtualisation)
    if (scrollTopRafId.current !== null) {
      cancelAnimationFrame(scrollTopRafId.current);
    }
    scrollTopRafId.current = requestAnimationFrame(() => {
      const newVal = ta.scrollTop;
      setScrollTop(newVal);
      scrollTopRafId.current = null;
    });
  }, []);

  // Cleanup rAF on unmount
  useEffect(() => {
    return () => {
      if (scrollTopRafId.current !== null) {
        cancelAnimationFrame(scrollTopRafId.current);
      }
    };
  }, []);

  const handleSelectionChange = useCallback(() => {
    const ta = textAreaRef.current;
    if (!ta) return;
    const selStart = ta.selectionStart;
    const beforeNewlines = value.slice(0, selStart).match(/\n/g);
    const line = beforeNewlines ? beforeNewlines.length + 1 : 1;
    setCursorLine(line);
  }, [value]);

  const handleChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    if (readOnly) return;
    const newValue = e.target.value;
    setValue(newValue);
    onChange(newValue);
    if (filePath) {
      useTabsStore.getState().markDirty(filePath);
    }
    // Update cursor line after a change (will be refined on selection change)
    handleSelectionChange();
  };

  // Common class for the code area (textarea and pre)
  const codeClass = cn(
    'font-mono text-sm leading-[22px] p-0 hide-scrollbar text-editor-foreground',
  );

  const codeStyle: React.CSSProperties = {
    height: '100%',
    width: '100%',
    margin: 0,
    border: 0,
    padding: 0,
    overflow: 'auto',
    scrollbarWidth: 'none',
    msOverflowStyle: 'none',
    whiteSpace: 'pre',
    wordBreak: 'normal',
  };

  const gutter = (
    <LineNumberGutter
      lineCount={lineCount}
      cursorLine={cursorLine}
      lineHeight={lineHeight}
      scrollTop={scrollTop}
      containerHeight={containerHeight}
      innerRef={gutterInnerRef}
    />
  );

  if (readOnly) {
    return (
      <div
        ref={containerRef}
        className={cn('flex flex-row h-full w-full gap-1 bg-editor', className)}
        onScroll={handleScroll}
      >
        {gutter}
        <pre
          ref={textAreaRef as unknown as React.RefObject<HTMLPreElement>}
          className={cn(codeClass, 'bg-editor flex-1')}
          style={{
            ...codeStyle,
            overflow: 'auto',
          }}
        >
          {value}
        </pre>
      </div>
    );
  }

  return (
    <div
      ref={containerRef}
      className={cn('flex flex-row h-full w-full gap-1', className)}
      onScroll={handleScroll}
    >
      {gutter}
      <textarea
        ref={textAreaRef}
        className={cn(
          codeClass,
          'bg-transparent caret-foreground outline-none resize-none flex-1',
        )}
        style={{
          ...codeStyle,
          border: 'none',
        }}
        value={value}
        onChange={handleChange}
        onScroll={handleScroll}
        onSelect={handleSelectionChange}
        spellCheck={false}
        autoComplete="off"
        autoCorrect="off"
        wrap="off"
      />
    </div>
  );
}
