"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const Stack = {
    _stack: [],
    _stackSize: 1,
    construct(size) {
        this._stackSize = size;
    },
    create(stackLimit) {
        this.construct(stackLimit);
        const proto = Object.create(this);
        return proto;
    },
    push(value) {
        if (this._stack.length === this._stackSize) {
            this.pop();
        }
        this._stack.push(value);
        return this._stack.length;
    },
    pop() {
        delete this._stack[0];
        this._stack = this._stack.filter(Boolean);
        return this._stack.length;
    },
    clear() {
        this._stack.length = 0;
    },
    get length() {
        return this._stack.length;
    },
    get value() {
        return this._stack;
    }
};
exports.Stack = Stack;
