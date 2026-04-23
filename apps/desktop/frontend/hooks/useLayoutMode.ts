import { useEffect, useState } from 'react';
import { LAYOUT } from '@/features/workbench/config/layoutConstants';

export type LayoutMode = 'wide' | 'medium' | 'narrow';

export function useLayoutMode(): LayoutMode {
  const [mode, setMode] = useState<LayoutMode>(() => getLayoutMode(window.innerWidth));

  useEffect(() => {
    const handleResize = () => {
      setMode(getLayoutMode(window.innerWidth));
    };
    window.addEventListener('resize', handleResize);
    return () => window.removeEventListener('resize', handleResize);
  }, []);

  return mode;
}

function getLayoutMode(width: number): LayoutMode {
  if (width >= LAYOUT.breakpoints.wide) return 'wide';
  if (width >= LAYOUT.breakpoints.medium) return 'medium';
  return 'narrow';
}
