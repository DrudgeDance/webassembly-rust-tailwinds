/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.rs",
    "../frontend/src/**/*.rs"
  ],
  theme: {
    extend: {
      colors: {
        primary: '#4F46E5',
        secondary: '#10B981',
      },
      spacing: {
        '128': '32rem',
      },
    },
  },
  plugins: [],
} 