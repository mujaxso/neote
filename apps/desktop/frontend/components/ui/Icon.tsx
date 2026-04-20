import { cn } from '@/lib/utils';
import { nerdFontIcons } from '@/lib/theme/nerd-font-icons';

interface IconProps {
  name: keyof typeof nerdFontIcons;
  size?: number;
  className?: string;
  label?: string;
  debug?: boolean;
}

export function Icon({ name, size = 16, className, label, debug = false }: IconProps) {
  const iconGlyph = nerdFontIcons[name] || '?';
  
  return (
    <span 
      className={cn(
        'font-icon inline-flex items-center justify-center antialiased',
        'leading-none tracking-normal',
        'select-none',
        debug && 'outline outline-1 outline-red-500',
        className
      )}
      style={{ 
        fontSize: size,
        width: size,
        height: size,
        fontVariantLigatures: 'normal',
        fontFeatureSettings: '"liga" 1, "calt" 1',
      }}
      role="img"
      aria-label={label || name}
      title={label || name}
      data-icon-name={name}
      data-debug-font="JetBrainsMonoNL Nerd Font Mono"
    >
      {iconGlyph}
    </span>
  );
}
