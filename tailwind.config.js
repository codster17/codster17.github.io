/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "class",
  content: ["./src/**/*.{html,rs}", "./index.html" ,"./styles/**/*.css"],
  theme: {
    fontFamily: {
      head: ["Lato"],
      body: ["Roboto"],
    },
    extend: {},
  },
  plugins: [],
};
