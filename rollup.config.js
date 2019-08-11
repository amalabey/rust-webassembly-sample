import babel from "rollup-plugin-babel"
import uglify from "rollup-plugin-uglify"

export default {
    input: './target/deploy/sample.js',
    output: {
        name: 'sample',
        file: './release/sample.js',
        format: 'es',
    },
    plugins: [
        babel({
            exclude: 'node_modules/**'
        }),
        uglify
    ]
};