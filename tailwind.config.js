import flowbitePlugin from "flowbite/plugin";
import defaultTheme from "tailwindcss/defaultTheme";

/** @type {import('tailwindcss').Config} */
export default {
    content: [
        "./src/**/*.{html,js,svelte,ts}",
        "./node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}",
        "./node_modules/flowbite-svelte-icons/**/*.{html,js,svelte,ts}"
    ],
    darkMode: "selector",
    theme: {
        extend: {
            fontFamily: {
                sans: ["Noto Sans KR Variable", ...defaultTheme.fontFamily.sans],
                gothic: ["Nanum Gothic Coding", ...defaultTheme.fontFamily.sans]
            },
            fontSize: {
                xxs: "0.875rem", // * 14px
                "3xs": "0.75rem",
                ...defaultTheme.fontSize
            },
            colors: {
                // ? flowbite-svelte
                primary: {
                    50: "#FFF5F2",
                    100: "#FFF1EE",
                    200: "#FFE4DE",
                    300: "#FFD5CC",
                    400: "#FFBCAD",
                    500: "#FE795D",
                    600: "#EF562F",
                    700: "#EB4F27",
                    800: "#CC4522",
                    900: "#A5371B"
                }
            }
        }
    },
    plugins: [flowbitePlugin],
    safelist: [
        // ? 동적 클래스 사용을 할 때 JIT 모드로 클래스가 누락되지 않도록 safelist에 추가
        {
            pattern:
                /text-(slate|gray|zinc|neutral|stone|red|orange|amber|yellow|lime|green|emerald|teal|cyan|sky|blue|indigo|violet|purple|fuchsia|pink|rose)-(500)/
        }
    ]
};
