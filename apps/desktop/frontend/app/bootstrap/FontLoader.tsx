import { useEffect, useState } from 'react';

export function FontLoader() {
  const [fontsLoaded, setFontsLoaded] = useState(false);

  useEffect(() => {
    const checkFont = async () => {
      try {
        // Load all font variations
        const fontVariations = [
          { family: 'JetBrainsMonoNL Nerd Font Mono', url: '/fonts/JetBrainsMonoNerdFont-Regular.ttf', weight: '400', style: 'normal' },
          { family: 'JetBrainsMonoNL Nerd Font Mono', url: '/fonts/JetBrainsMonoNerdFont-Bold.ttf', weight: '700', style: 'normal' },
          { family: 'JetBrainsMonoNL Nerd Font Mono', url: '/fonts/JetBrainsMonoNerdFont-Italic.ttf', weight: '400', style: 'italic' },
          { family: 'JetBrainsMonoNL Nerd Font Mono', url: '/fonts/JetBrainsMonoNerdFont-BoldItalic.ttf', weight: '700', style: 'italic' },
        ];

        const loadPromises = fontVariations.map(async (variation) => {
          const font = new FontFace(
            variation.family,
            `url("${variation.url}") format("truetype")`,
            { weight: variation.weight, style: variation.style }
          );
          await font.load();
          document.fonts.add(font);
          return font;
        });

        await Promise.all(loadPromises);
        
        // Check if the font is actually available by testing with a known character
        await document.fonts.ready;
        
        // Test if the font is actually applied
        const testElement = document.createElement('span');
        testElement.style.fontFamily = '"JetBrainsMonoNL Nerd Font Mono", monospace';
        testElement.style.fontSize = '20px';
        testElement.style.position = 'absolute';
        testElement.style.opacity = '0';
        testElement.style.pointerEvents = 'none';
        testElement.textContent = ''; // Rust icon from Nerd Font
        document.body.appendChild(testElement);
        
        // Also test with a regular character
        const testElement2 = document.createElement('span');
        testElement2.style.fontFamily = '"JetBrainsMonoNL Nerd Font Mono", monospace';
        testElement2.style.fontSize = '20px';
        testElement2.style.position = 'absolute';
        testElement2.style.opacity = '0';
        testElement2.style.pointerEvents = 'none';
        testElement2.textContent = 'A'; // Regular character
        document.body.appendChild(testElement2);
        
        // Check if the font is being used
        setTimeout(() => {
          const isFontApplied = document.fonts.check('20px "JetBrainsMonoNL Nerd Font Mono"');
          console.log('Font applied check:', isFontApplied);
          
          // Also check the computed style
          const computedStyle = window.getComputedStyle(testElement);
          console.log('Computed font-family for icon:', computedStyle.fontFamily);
          
          const computedStyle2 = window.getComputedStyle(testElement2);
          console.log('Computed font-family for regular char:', computedStyle2.fontFamily);
          
          // Check the actual rendered width (icons may have different widths)
          console.log('Icon element width:', testElement.offsetWidth);
          console.log('Regular char width:', testElement2.offsetWidth);
          
          document.body.removeChild(testElement);
          document.body.removeChild(testElement2);
          
          if (isFontApplied) {
            document.body.classList.add('fonts-loaded');
            setFontsLoaded(true);
            console.log('JetBrainsMonoNL Nerd Font Mono loaded and applied successfully');
          } else {
            console.warn('Font loaded but not applied. Using fallback:', computedStyle.fontFamily);
            setFontsLoaded(false);
          }
        }, 200);
        
      } catch (error) {
        console.error('Failed to load JetBrainsMonoNL Nerd Font Mono:', error);
        setFontsLoaded(false);
      }
    };

    checkFont();
  }, []);

  // This component doesn't render anything visible
  return null;
}
