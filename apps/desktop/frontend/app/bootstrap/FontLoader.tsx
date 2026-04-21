import { useEffect, useState } from 'react';

export function FontLoader() {
  const [fontsLoaded, setFontsLoaded] = useState(false);

  useEffect(() => {
    const loadFonts = async () => {
      try {
        console.log('[FontLoader] Starting font loading...');
        // Create a FontFace for each variation
        const regular = new FontFace(
          'Zaroxi Mono',
          'url("/fonts/JetBrainsMonoNerdFont-Regular.ttf") format("truetype")',
          { weight: '400', style: 'normal' }
        );
        const bold = new FontFace(
          'Zaroxi Mono',
          'url("/fonts/JetBrainsMonoNerdFont-Bold.ttf") format("truetype")',
          { weight: '700', style: 'normal' }
        );
        const italic = new FontFace(
          'Zaroxi Mono',
          'url("/fonts/JetBrainsMonoNerdFont-Italic.ttf") format("truetype")',
          { weight: '400', style: 'italic' }
        );
        const boldItalic = new FontFace(
          'Zaroxi Mono',
          'url("/fonts/JetBrainsMonoNerdFont-BoldItalic.ttf") format("truetype")',
          { weight: '700', style: 'italic' }
        );

        // Load all fonts
        const loadedFonts = await Promise.all([
          regular.load(),
          bold.load(),
          italic.load(),
          boldItalic.load(),
        ]);

        // Add them to the document
        loadedFonts.forEach(font => document.fonts.add(font));

        // Wait for fonts to be ready
        await document.fonts.ready;
        console.log('[FontLoader] FontFace objects added to document.fonts');

        // Verify the font is available
        const isLoaded = document.fonts.check('12px "Zaroxi Mono"');
        if (isLoaded) {
          console.log('[FontLoader] Font verified as available via check()');
        } else {
          console.warn('[FontLoader] Font check failed, may fallback to system font');
        }

        document.documentElement.setAttribute('data-fonts-loaded', 'true');
        document.body.classList.add('fonts-loaded');
        setFontsLoaded(true);
        console.log('[FontLoader] Fonts loaded successfully');
        // Debug: check actual applied font on a test element
        const testEl = document.createElement('div');
        testEl.style.fontFamily = '"Zaroxi Mono", monospace';
        testEl.style.position = 'absolute';
        testEl.style.opacity = '0';
        testEl.style.pointerEvents = 'none';
        testEl.textContent = 'test';
        document.body.appendChild(testEl);
        setTimeout(() => {
          const computed = window.getComputedStyle(testEl);
          console.log('[FontLoader] Computed font-family:', computed.fontFamily);
          document.body.removeChild(testEl);
        }, 100);
      } catch (error) {
        console.error('[FontLoader] Failed to load fonts:', error);
        setFontsLoaded(false);
      }
    };

    loadFonts();
  }, []);

  // While fonts are loading, we could show a loading indicator
  // For now, just return null
  return null;
}
