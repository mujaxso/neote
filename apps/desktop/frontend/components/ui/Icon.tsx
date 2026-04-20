import { cn } from '@/lib/utils';
import { nerdFontIcons } from '@/lib/theme/nerd-font-icons';

interface IconProps {
  name: keyof typeof nerdFontIcons;
  size?: number;
  className?: string;
  label?: string;
}

export function Icon({ name, size = 16, className, label }: IconProps) {
  const iconGlyph = nerdFontIcons[name] || '?';
  
  return (
    <span 
      className={cn(
        'font-icon inline-flex items-center justify-center antialiased',
        'leading-none tracking-normal',
        'select-none',
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
    >
      {iconGlyph}
    </span>
  );
}
