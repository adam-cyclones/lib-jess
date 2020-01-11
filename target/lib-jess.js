"use strict";
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (Object.hasOwnProperty.call(mod, k)) result[k] = mod[k];
    result["default"] = mod;
    return result;
};
Object.defineProperty(exports, "__esModule", { value: true });
// @ts-ignore
const wasm32 = __importStar(require("../target/wasm32-bindgen/jess"));
const constants_1 = require("./constants");
exports.JESS_EXTNAME = constants_1.JESS_EXTNAME;
exports.TESS_EXTNAME = constants_1.TESS_EXTNAME;
const fs_1 = require("fs");
const { readFile } = fs_1.promises;
exports.compiler = (async () => {
    // expose to public
    return {
        /**
         * Compiles a Jess sourse file and returns the output/s
         * @param {path} - path to file.
        */
        async compileFile(path, opts = {}) {
            // assign default options
            Object.assign(opts, {
                tess: false
            });
            const file = await readFile(path, 'utf8');
            console.log('WS', wasm32.ast(lex(file)));
            // return wasm32.compile(file);
        }
    };
})();
function lex(sourse) {
    return sourse
        .replace(/((".*?")|('.*?')|\W(?<!(\s|^[\W]|@)))/gm, ' $1 ')
        .split(/\s/)
        .filter(Boolean)
        .join(' ');
}
