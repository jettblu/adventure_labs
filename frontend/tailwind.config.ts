import type { Config } from "tailwindcss";

const config: Config = {
  content: [
    "./pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./components/**/*.{js,ts,jsx,tsx,mdx}",
    "./app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      colors: {
        primary: "#5f975b",
        secondary: "#73a9cd", // #E3FF70
        tertiary: "#f3af4a",
        quaternary: "#fdfbfc",
      },
    },
  },
  plugins: [require("@tailwindcss/typography")],
};
export default config;
