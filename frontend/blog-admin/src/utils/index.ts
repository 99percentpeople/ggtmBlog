export function mapVal<T, U>(obj: { [k: string]: T } | ArrayLike<T>, callbackfn: (value: T) => U): { [s: string]: U } {
    let newObj = {} as { [k: string]: U };
    Object.entries(obj).map(([key, val]) =>
        Object.defineProperty<{ [k: string]: U }>(newObj, key, {
            value: callbackfn(val),
            enumerable:true
        })
    );
    return newObj;
}
