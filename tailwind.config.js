/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.rs",
  ],
  theme: {
    extend: {
      colors: {
        'primary-start': '#667eea',
        'primary-end': '#764ba2',
        'button-red-start': '#ff6b6b',
        'button-red-end': '#ee5a52',
        'button-blue-start': '#4f46e5',
        'button-blue-end': '#6366f1',
        'button-green-start': '#10b981',
        'button-green-end': '#059669',
        'button-orange-start': '#f59e0b',
        'button-orange-end': '#f97316',
      },
      backdropBlur: {
        xs: '2px',
      }
    },
  },
  plugins: [],
}