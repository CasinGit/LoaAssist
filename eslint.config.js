import js from "@eslint/js";
import prettier from "eslint-config-prettier";
import importPlugin from "eslint-plugin-import";
import svelte from "eslint-plugin-svelte";
import globals from "globals";
import ts from "typescript-eslint";

export default ts.config(
    js.configs.recommended,
    ...ts.configs.recommended,
    ...svelte.configs["flat/recommended"],
    prettier,
    ...svelte.configs["flat/prettier"],
    importPlugin.flatConfigs.recommended,
    {
        languageOptions: {
            globals: {
                ...globals.browser,
                ...globals.node
            }
        }
    },
    {
        files: ["**/*.svelte"],
        languageOptions: {
            parserOptions: {
                parser: ts.parser
            }
        }
    },
    {
        ignores: ["build/", ".svelte-kit/", "dist/"]
    },
    {
        rules: {
            // * analysis/correctness
            "import/no-unresolved": "off",
            "import/named": "error",
            "import/namespace": "error",
            "import/default": "error",
            "import/export": "error",

            // * red flags (thus, warnings)
            "import/no-named-as-default": "warn",
            "import/no-named-as-default-member": "warn",
            "import/no-duplicates": "warn",

            "import/order": [
                "error",
                {
                    groups: [
                        ["builtin", "external"],
                        ["internal", "parent", "sibling", "index"]
                    ],
                    "newlines-between": "always",
                    alphabetize: { order: "asc", caseInsensitive: true }
                }
            ],

            "@typescript-eslint/no-unused-vars": [
                "warn",
                {
                    args: "after-used",
                    argsIgnorePattern: "^_",
                    ignoreRestSiblings: true,
                    varsIgnorePattern: "^_.*$"
                }
            ],

            "@typescript-eslint/no-explicit-any": "off",
            "svelte/no-unused-svelte-ignore": "off",
            "prefer-const": "off"
        }
    }
);
