interface iStack {
    _stack: any[];
    _stackSize: number;
    construct(size: number): void;
    create(stackLimit: number): iStack;
    push(value: any): number;
    pop(): number;
    clear(): void;
    length: number;
    value: any[];
}
declare const Stack: iStack;
export { Stack, iStack };
