const { defineConfig } = require("vite");

module.exports = defineConfig({
    server: {
        fs: {
            allow: [".."]
        }
    }
})