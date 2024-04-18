/** @type {import("tailwindcss").Config} */
export default {
  content: ["./src/**/*.{html,js,svelte,ts}", "index.html"],
  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],
  daisyui: {
    themes: [
      "corporate",
      {
        wlsn: {
          primary: "#1B467Eff",
          secondary: "#ED8CEFff",
          accent: "#5FF3EBff",
          neutral: "#F4F4F4ff",
          "base-100": "#ffffff",
          "--rounded-box": "0",
          "--rounded-btn": "0",
        },
      },
    ],
  },
};
/* CSS HEX */
// --polynesian-blue: #1B467Eff;
// --violet-web-color: #ED8CEFff;
// --white: #FEFEFEff;
// --fluorescent-cyan: #5FF3EBff;
// --white-smoke: #F4F4F4ff;
// --black: #000000ff;
// --straw: #F4EC87ff;
// --magnolia: #F0E9F0ff;
// --violet-web-color-2: #DD8AEBff;
// --silver: #AEAFAFff;
