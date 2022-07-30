const { defineConfig } = require("vite");

module.exports = defineConfig({
    base: '/perlin/',
    server: {
        fs: {
            allow: [".."]
        }
    },
    build: {
        target: 'es2022'
    }
})