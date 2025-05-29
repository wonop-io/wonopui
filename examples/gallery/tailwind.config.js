module.exports = {
  mode: "jit",
  content: {
    files: [
      "../**/*.rs",
      "../*.rs",
      "index.html",
      "./target/**/wonopui.json",
      "./target/**/tailwindcss.txt",
      "./target/wonopui.json",
      "./target/tailwindcss.txt",
    ],
  },
  darkMode: "class", // 'selector' or 'class'
  theme: {
    extend: {},
  },
  variants: {
    extend: {},
  },
  plugins: [],
};
