// rollup.config.mjs
import nodeResolve from "@rollup/plugin-node-resolve";

/** @type {import('rollup').RollupOptions} */
const config = {
    input: [
        "./rollup_inputs/button.js",
        "./rollup_inputs/fab.js",
        "./rollup_inputs/chip.js",
        "./rollup_inputs/icon-button.js",
        "./rollup_inputs/chips.js",
        "./rollup_inputs/field.js",
        "./rollup_inputs/select.js",
        "./rollup_inputs/tabs.js",
        "./rollup_inputs/textfield.js",
        "@material/web/checkbox/checkbox.js",
        "@material/web/radio/radio.js",
        "@material/web/menu/menu.js",
        "@material/web/menu/menu-item.js",
        "@material/web/menu/sub-menu.js",
        "@material/web/list/list.js",
        "@material/web/list/list-item.js",
        "@material/web/progress/circular-progress.js",
        "@material/web/progress/linear-progress.js",
        "@material/web/slider/slider.js",
        "@material/web/switch/switch.js",
        // Added new components below
        "@material/web/dialog/dialog.js",
        "@material/web/divider/divider.js",
        "@material/web/ripple/ripple.js",
    ],
    plugins: [nodeResolve()],
    output: {
        dir: `../md-web`,
        chunkFileNames: "[name].js",
        banner: (chunk) => {
            // if (chunk.name !== "core") {
            //
            // } else { return "" }
            return 'export const __dummy_loader = () => {};\n'
        },
        manualChunks: (id, { getModuleInfo }) => {
            const info = getModuleInfo(id);
            if (info.importers.length <= 1) {
                // This will be inlined anyway
                return;
            }

            return "core";
        },
    }
}

// noinspection JSUnusedGlobalSymbols
export default config;