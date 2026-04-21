/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      fontFamily: {
        sans: ['"JetBrainsMonoNL Nerd Font Mono"', '"JetBrainsMonoNL NFM"', 'ui-sans-serif', 'system-ui', 'sans-serif'],
        mono: ['"JetBrainsMonoNL Nerd Font Mono"', '"JetBrainsMonoNL NFM"', 'ui-monospace', 'SFMono-Regular', 'monospace'],
        icon: ['"JetBrainsMonoNL Nerd Font Mono"', '"JetBrainsMonoNL NFM"', 'ui-monospace', 'SFMono-Regular', 'monospace'],
      },
      colors: {
        // Semantic colors mapped to CSS variables
        app: 'var(--color-app-background)',
        shell: 'var(--color-shell-background)',
        panel: 'var(--color-panel-background)',
        'elevated-panel': 'var(--color-elevated-panel-background)',
        editor: 'var(--color-editor-background)',
        input: 'var(--color-input-background)',
        'status-bar': 'var(--color-status-bar-background)',
        'title-bar': 'var(--color-title-bar-background)',
        'activity-rail': 'var(--color-activity-rail-background)',
        sidebar: 'var(--color-sidebar-background)',
        tab: 'var(--color-tab-background)',
        'tab-active': 'var(--color-tab-active-background)',
        'assistant-panel': 'var(--color-assistant-panel-background)',
        
        // Text colors
        'text-primary': 'var(--color-text-primary)',
        'text-secondary': 'var(--color-text-secondary)',
        'text-muted': 'var(--color-text-muted)',
        'text-faint': 'var(--color-text-faint)',
        'text-on-accent': 'var(--color-text-on-accent)',
        'text-on-surface': 'var(--color-text-on-surface)',
        'text-disabled': 'var(--color-text-disabled)',
        'text-link': 'var(--color-text-link)',
        
        // UI colors
        border: 'var(--color-border)',
        'border-subtle': 'var(--color-border-subtle)',
        divider: 'var(--color-divider)',
        accent: 'var(--color-accent)',
        'accent-hover': 'var(--color-accent-hover)',
        'accent-soft': 'var(--color-accent-soft)',
        'accent-soft-bg': 'var(--color-accent-soft-background)',
        
        // State colors
        'hover-bg': 'var(--color-hover-background)',
        'active-bg': 'var(--color-active-background)',
        'selected-bg': 'var(--color-selected-background)',
        'selected-text-bg': 'var(--color-selected-text-background)',
        'selected-editor-bg': 'var(--color-selected-editor-background)',
        
        // Status colors
        success: 'var(--color-success)',
        warning: 'var(--color-warning)',
        error: 'var(--color-error)',
        info: 'var(--color-info)',
        
        // Editor colors
        'editor-gutter': 'var(--color-editor-gutter-background)',
        'editor-line-highlight': 'var(--color-editor-line-highlight)',
        'editor-cursor': 'var(--color-editor-cursor)',
        'editor-selection': 'var(--color-editor-selection)',
        'editor-find-highlight': 'var(--color-editor-find-highlight)',
        
        // Syntax colors
        'syntax-keyword': 'var(--color-syntax-keyword)',
        'syntax-function': 'var(--color-syntax-function)',
        'syntax-string': 'var(--color-syntax-string)',
        'syntax-comment': 'var(--color-syntax-comment)',
        'syntax-type': 'var(--color-syntax-type)',
        'syntax-variable': 'var(--color-syntax-variable)',
        'syntax-constant': 'var(--color-syntax-constant)',
      },
    },
  },
  plugins: [],
}
