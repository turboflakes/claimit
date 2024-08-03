module.exports = {
    darkMode: 'selector',
    purge: {
      mode: "all",
      content: [
        "./src/**/*.rs",
        "./index.html",
        "./src/**/*.html",
        "./src/**/*.css",
      ],
    },
    theme: {
      borderRadius: {
        'lg': '1.5rem',
        'full': '9999px',
      },
      colors: {
        transparent: 'transparent',
        current: 'currentColor',
        'white': '#ffffff',
        'gray': {
          50: 'hsl(210, 17% 98%)',
          100: 'hsl(210, 17% 98%)',
          200: 'hsl(210, 16%, 93%)',
          300: 'hsl(210, 14%, 89%)',
          400: 'hsl(210, 14%, 83%)',
          500: 'hsl(210, 11%, 71%)',
          600: 'hsl(208, 7%, 46%)',
          700: 'hsl(210, 9%, 31%)',
          800: 'hsl(210, 10%, 23%)',
          900: 'hsl(210, 11%, 15%)',
        },
        'red': {
          light: 'hsl(346, 84%, 81%)',
          DEFAULT: 'hsl(346, 84%, 61%)',
          dark: 'hsl(346, 84%, 41%)',
        },
        'green': {
          light: 'hsl(164, 95%, 62%)',
          DEFAULT: 'hsl(164, 95%, 42%)',
          dark: 'hsl(164, 95%, 22%)',
        },
        'yellow': {
          light: 'hsl(58, 100%, 80%)',
          DEFAULT: 'hsl(58, 100%, 68%)',
        }
      },
    },
    variants: {},
    plugins: [],
  };