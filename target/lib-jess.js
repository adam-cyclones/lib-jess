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
    const sourceMap = {};
    return {
        /**
         * Compiles a Jess sourse file and returns the output/s
         * @param {path} - path to file.
        */
        async compileFile(path, opts = {}) {
            // assign default options
            Object.assign(opts, {
                tess: false,
                'dump-tokens': false,
            });
            const file = await readFile(path, 'utf8');
            const lexicalAnalysis = JSON.parse(wasm32.ast(lex(file)).replace(/""/gm, '\"'));
            // positions
            let col = 0;
            let row = 0;
            // Add lexing results to this file
            sourceMap[path] = {
                lexicalAnalysis
            };
            // Add positions to this file
            sourceMap[path].lexicalAnalysis.forEach((item) => {
                item.position = {
                    col,
                    row,
                    formatted: `${path}:${row}:${col}`
                };
            });
            // par: for (const line of rowView(file)) {
            //   let atCol = 0;
            //   atCol = 0;
            // }
            // if atleast one panic is found, locate line and col numbers
            if (lexicalAnalysis.find((token) => token.token_id === "PANIC")) {
                function* locateTokens() {
                    let streamCopy = [...lexicalAnalysis].map(entry => entry.value);
                    let lines = rowView(file);
                    let colAt = 0;
                    let rowAt = 0;
                    let line;
                    let lineNo;
                    let ind = -1;
                    while (true) {
                        line = lines[rowAt][1];
                        lineNo = lines[rowAt][0];
                        const findThisToken = streamCopy[0];
                        colAt = line.indexOf(findThisToken, colAt);
                        const tokenOnThisLine = colAt >= 0;
                        if (findThisToken === 'EOF') {
                            return;
                        }
                        if (tokenOnThisLine) {
                            ind++;
                            yield [lineNo, colAt + 1, findThisToken, `${path}:${lineNo}:${colAt + 1}`, ind];
                            streamCopy.shift();
                        }
                        else {
                            // move to next line
                            rowAt++;
                        }
                    }
                }
                for (const [row, col, token, formatted, tokenStreamIndex] of locateTokens()) {
                    sourceMap[path].lexicalAnalysis[tokenStreamIndex].position = {
                        col,
                        row,
                        formatted
                    };
                }
            }
            // Early error handling
            // brace unbalenced
            const eofBraceDepth = sourceMap[path].lexicalAnalysis.find((token) => token.token_id === 'EOF').brace_depth;
            if (eofBraceDepth !== 0) {
                // ERROR_UNBALENCED
                console.log(sourceMap[path].lexicalAnalysis.reverse().filter((last_token) => last_token.brace_depth > 0 && last_token.token_id === 'L_C_BRACE'));
                // throw new Error(`ERR_UNBALENCED: m`);
            }
            console.log(sourceMap[path].lexicalAnalysis);
            // return wasm32.compile(file);
        }
    };
})();
function lex(sourse) {
    return sourse
        .replace(/((".*?")|('.*?')|#([0-9a-fA-F]{3}){1,2}|\W(?<!(\s|^[\W]|@)))/gm, ' $1 ')
        .split(/\s/)
        .filter(Boolean)
        .join(' ');
}
function rowView(sourse) {
    return sourse.split(/\n/)
        .map((text, ind) => [ind + 1, text])
        .filter(row => row[1]);
}
