/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.rs",
    "../frontend/src/**/*.rs"
  ],
  purge: {
    enabled: true,
    content: [
      "./src/**/*.rs",
      "../frontend/src/**/*.rs"
    ],
    options: {
      safelist: [
        /^bg-/,  // Preserve all background classes
        /^text-/  // Preserve all text classes
      ]
    }
  },
  theme: {
    extend: {
      colors: {
        primary: 'var(--primary)',
        secondary: 'var(--secondary)',
        background: 'var(--background)',
        surface: 'var(--surface)',
        text: 'var(--text)',
        'text-muted': 'var(--text-muted)',
        accent: 'var(--accent)',
        'primary': 'var(--primary)',
        'secondary': 'var(--secondary)',
        'accent': 'var(--accent)',
      },
      spacing: {
        '128': '32rem',
      },
      keyframes: {
        'fade-in': {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' }
        }
      },
      animation: {
        'fade-in': 'fade-in 0.1s ease-out forwards'
      }
    },
  },
  plugins: [],
} 