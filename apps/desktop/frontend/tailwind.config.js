/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      fontFamily: {
        sans: ['"JetBrainsMonoNL Nerd Font Mono"', '"JetBrainsMonoNL NFM"', 'ui-sans-serif', 'system-ui', 'sans-serif'],
        mono: ['"JetBrainsMonoNL Nerd Font Mono"', '"JetBrainsMonoNL NFM"', 'ui-monospace', 'SFMono-Regular', 'monospace'],
        icon: ['"JetBrainsMonoNL Nerd Font Mono"', '"JetBrainsMonoNL NFM"', 'ui-monospace', 'SFMono-Regular', 'monospace'],
      },
    },
  },
  plugins: [],
}
