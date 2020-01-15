import { iStack } from './stack';
declare type tCBraceToken = '{' | '}';
interface iBrace {
    token: tCBraceToken;
    index: number;
    raw: string;
    depth: number;
    incr: () => void;
    decr: () => void;
    onToken(token: tCBraceToken, callback: (callback: iBraceOnTokenDetail) => void): void;
    captureUntil(token: tCBraceToken): void;
    stopCapture(callback: (detail: iBraceStopCaptureCallbackDetail) => void): void;
    resetCapturedContent(): void;
}
interface iBraceCaptureLine {
    raw: string;
    depth: number;
}
interface iBraceStopCaptureCallbackDetail {
    capturedStack: iBraceCaptureLine[];
    done: boolean;
}
interface iBraceOnTokenDetail {
    raw: string;
    depth: number;
}
declare const curlyBraceStack: iStack;
declare function parseCurlyBraces(line: string): iBrace[];
export { parseCurlyBraces, curlyBraceStack };
