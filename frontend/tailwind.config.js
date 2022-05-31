module.exports = {
  content: [
    "./pages/**/*.{js,ts,jsx,tsx}",
    "./components/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      fontFamily: {
        sans: ["CoFo Sans"],
      },

      colors: {
        "slakker": '#FFE9EC',
        "tblur": '#ea4c89'
      }
    },
  },
  plugins: [],
};
