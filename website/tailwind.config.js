/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,jsx}",
  ],
  theme: {
    extend: {
      colors: {
        pistachio: {
          50: '#fafbf6',
          100: '#f3f6eb',
          200: '#e6ecd6',
          300: '#d4deb5',
          400: '#b8ca87',
          500: '#93c572',
          600: '#7ab15a',
          700: '#609847',
          800: '#4d7a3a',
          900: '#3e6230',
          950: '#23391a',
        },
      },
      fontFamily: {
        sans: ['Inter var', 'Inter', 'system-ui', 'sans-serif'],
      },
    },
  },
  plugins: [],
}
