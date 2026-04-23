import { useEffect, useRef, useState, useCallback, useMemo } from 'react';
import { cn } from '@/lib/utils';

interface CodeEditorProps {
  initialValue: string;
  onChange: (value: string) => void;
  language?: string;
  readOnly?: boolean;
  className?: string;
  /** Total number of lines in the document (for virtual scrolling) */
  totalLines?: number;
  /** Callback to request more lines when scrolling */
  onRequestLines?: (startLine: number, count: number) => Promise<{ lines: { index: number; text: string }[] }>;
  /** Whether the document is in large file mode */
  largeFileMode?: boolean;
}

const LINE_HEIGHT = 22; // pixels
const OVERSCAN_LINES = 10; // lines to render above/below viewport

export function CodeEditor({
  initialValue,
  onChange,
  language = 'plaintext',
  readOnly = false,
  className,
  totalLines,
  onRequestLines,
  largeFileMode = false,
}: CodeEditorProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const textareaRef = useRef<HTMLTextAreaElement>(null);
  const [value, setValue] = useState(initialValue);
  const [scrollTop, setScrollTop] = useState(0);
  const [containerHeight, setContainerHeight] = useState(600);
  const [visibleLines, setVisibleLines] = useState<{ index: number; text: string }[]>([]);
  const [isLoading, setIsLoading] = useState(false);

  // Compute visible range
  const visibleRange = useMemo(() => {
    const startLine = Math.max(0, Math.floor(scrollTop / LINE_HEIGHT) - OVERSCAN_LINES);
    const endLine = Math.min(
      totalLines ?? Infinity,
      Math.ceil((scrollTop + containerHeight) / LINE_HEIGHT) + OVERSCAN_LINES
    );
    return { startLine, endLine };
  }, [scrollTop, containerHeight, totalLines]);

  // Fetch visible lines when range changes
  useEffect(() => {
    if (!onRequestLines) return;
    
    const fetchLines = async () => {
      setIsLoading(true);
      try {
        const result = await onRequestLines(visibleRange.startLine, visibleRange.endLine - visibleRange.startLine);
        setVisibleLines(result.lines);
      } catch (error) {
        console.error('Failed to fetch visible lines:', error);
      } finally {
        setIsLoading(false);
      }
    };
    
    fetchLines();
  }, [visibleRange.startLine, visibleRange.endLine, onRequestLines]);

  // Handle scroll
  const handleScroll = useCallback((e: React.UIEvent<HTMLDivElement>) => {
    setScrollTop(e.currentTarget.scrollTop);
  }, []);

  // Observe container size
  useEffect(() => {
    const container = containerRef.current;
    if (!container) return;
    
    const observer = new ResizeObserver((entries) => {
      for (const entry of entries) {
        setContainerHeight(entry.contentRect.height);
      }
    });
    
    observer.observe(container);
    return () => observer.disconnect();
  }, []);

  // Sync initial value
  useEffect(() => {
    setValue(initialValue);
  }, [initialValue]);

  const handleChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    const newValue = e.target.value;
    setValue(newValue);
    onChange(newValue);
  };

  // Compute total height for scroll container
  const totalHeight = useMemo(() => {
    if (totalLines) {
      return totalLines * LINE_HEIGHT;
    }
    // Fallback: estimate from current content
    return Math.max(containerHeight, value.split('\n').length * LINE_HEIGHT);
  }, [totalLines, value, containerHeight]);

  // Compute line numbers for visible range
  const lineNumbers = useMemo(() => {
    const numbers: number[] = [];
    for (let i = visibleRange.startLine; i < visibleRange.endLine && i < (totalLines ?? Infinity); i++) {
      numbers.push(i + 1);
    }
    return numbers;
  }, [visibleRange, totalLines]);

  // Build text content for textarea from visible lines
  const visibleText = useMemo(() => {
    if (visibleLines.length > 0) {
      return visibleLines.map(l => l.text).join('\n');
    }
    // Fallback to initial value for small files
    return value;
  }, [visibleLines, value]);

  return (
    <div className={cn('relative h-full', className)} ref={containerRef}>
      {largeFileMode && (
        <div className="absolute top-0 left-0 right-0 z-10 bg-yellow-100 dark:bg-yellow-900 text-yellow-800 dark:text-yellow-200 text-xs px-2 py-1 text-center">
          Large file mode — some features are disabled for performance
        </div>
      )}
      <div className="absolute inset-0 bg-editor code-editor-font">
        {/* Scrollable container */}
        <div
          className="absolute inset-0 overflow-auto"
          onScroll={handleScroll}
          style={{ paddingTop: largeFileMode ? '20px' : '0' }}
        >
          {/* Spacer for scroll height */}
          <div style={{ height: totalHeight, position: 'relative' }}>
            {/* Line numbers gutter */}
            <div
              className="absolute left-0 top-0 w-8 bg-editor border-r border-border flex flex-col items-center font-mono text-xs text-muted-foreground"
              style={{
                transform: `translateY(${visibleRange.startLine * LINE_HEIGHT}px)`,
              }}
            >
              {lineNumbers.map((num) => (
                <div key={num} style={{ height: LINE_HEIGHT, lineHeight: `${LINE_HEIGHT}px` }} className="py-0">
                  {num}
                </div>
              ))}
            </div>
            
            {/* Text area for visible lines */}
            <textarea
              ref={textareaRef}
              value={visibleText}
              onChange={handleChange}
              readOnly={readOnly}
              className="absolute left-8 top-0 right-0 bg-transparent text-editor-foreground resize-none outline-none pr-4 whitespace-pre overflow-hidden font-mono"
              spellCheck="false"
              style={{
                tabSize: 2,
                fontFamily: 'var(--font-mono)',
                fontSize: '14px',
                lineHeight: `${LINE_HEIGHT}px`,
                letterSpacing: '0',
                height: visibleLines.length * LINE_HEIGHT,
                transform: `translateY(${visibleRange.startLine * LINE_HEIGHT}px)`,
              }}
              placeholder="Start typing..."
            />
          </div>
        </div>
        
        {/* Loading indicator */}
        {isLoading && (
          <div className="absolute bottom-2 right-2 bg-background/80 text-xs px-2 py-1 rounded">
            Loading...
          </div>
        )}
      </div>
    </div>
  );
}
