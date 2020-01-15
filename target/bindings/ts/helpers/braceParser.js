"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const stack_1 = require("./stack");
const L_C_BRACE = '{';
const R_C_BRACE = '}';
const CURLY_BRACES = new RegExp(`${L_C_BRACE}|${R_C_BRACE}`, 'g');
const curlyBraceStack = stack_1.Stack.create(Infinity);
exports.curlyBraceStack = curlyBraceStack;
const curlyBraceParserState = {
    captureUntil: '',
    capturedStack: [],
    captureRunning: false
};
function parseCurlyBraces(line) {
    const lineMatcher = line;
    const lineMatches = [...lineMatcher.matchAll(CURLY_BRACES)];
    const brace = lineMatches.map(function (m) {
        const result = {
            token: m[0],
            index: m.index,
            raw: m.input,
            depth: 0,
            onToken(token, callback) {
                const state = curlyBraceParserState;
                if (state.captureUntil) {
                    state.capturedStack.push({
                        raw: m.input,
                        depth: 0,
                    });
                }
                if (m[0] === token) {
                    callback();
                }
            },
            incr() {
                this.depth++;
            },
            decr() {
                this.depth--;
            },
            captureUntil(token) {
                const state = curlyBraceParserState;
                if (!state.captureUntil) {
                    state.captureRunning = true;
                    state.captureUntil = token;
                }
            },
            stopCapture(callback) {
                const state = curlyBraceParserState;
                if (m[0] === state.captureUntil) {
                    result.resetCapturedContent();
                }
                callback({
                    capturedStack: state.capturedStack,
                    done: !state.captureRunning
                });
            },
            resetCapturedContent() {
                const state = curlyBraceParserState;
                state.captureUntil = '';
                state.capturedStack.length = 0;
                state.captureRunning = false;
                console.log(state);
            }
        };
        return result;
    });
    return brace;
}
exports.parseCurlyBraces = parseCurlyBraces;
