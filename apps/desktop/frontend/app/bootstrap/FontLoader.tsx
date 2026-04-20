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
        
        // Test with multiple known Nerd Font icons
        const testIcons = ['', '', '', '', '']; // Rust, folder, folder-open, settings, search
        const testElements = testIcons.map((icon, i) => {
          const el = document.createElement('span');
          el.style.fontFamily = '"JetBrainsMonoNL Nerd Font Mono", monospace';
          el.style.fontSize = '20px';
          el.style.position = 'fixed';
          el.style.top = `${10 + i * 30}px`;
          el.style.left = '10px';
          el.style.backgroundColor = 'rgba(0,0,0,0.7)';
          el.style.color = 'white';
          el.style.padding = '5px';
          el.style.zIndex = '99999';
          el.textContent = `Icon ${i}: ${icon}`;
          document.body.appendChild(el);
          return el;
        });
        
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
          
          // Log each test icon
          testElements.forEach((el, i) => {
            console.log(`Test icon ${i}:`, {
              text: el.textContent,
              width: el.offsetWidth,
              height: el.offsetHeight,
              computedFont: window.getComputedStyle(el).fontFamily
            });
          });
          
          document.body.removeChild(testElement);
          document.body.removeChild(testElement2);
          testElements.forEach(el => {
            if (document.body.contains(el)) {
              document.body.removeChild(el);
            }
          });
          
          if (isFontApplied) {
            document.body.classList.add('fonts-loaded');
            setFontsLoaded(true);
            console.log('JetBrainsMonoNL Nerd Font Mono loaded and applied successfully');
          } else {
            console.warn('Font loaded but not applied. Using fallback:', computedStyle.fontFamily);
            setFontsLoaded(false);
          }
        }, 500);
        
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
