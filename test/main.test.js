const { fibonacci_rs, multiply_each_item_for_two_rs } = require("../index.node");

describe("Fibonacci api js", () => {
    it("should return 55 using Fibonacci with 10 as param", () => {
        const result = fibonacci_rs('10');

        expect(result).toEqual(55);
    });

    it("should fail using Fibonacci with incorrect param", () => {
        expect(() => {
            fibonacci_rs(10);
        }).toThrow(new TypeError('failed to downcast any to string'));
    });

    it("should fail using Fibonacci without param", () => {
        expect(() => {
            fibonacci_rs();
        }).toThrow(new TypeError('not enough arguments'));
    });
});

describe("Array api js", () => {
    it("should return [2, 4, 6] using Array converter with [1, 2, 3] as param", () => {
        const result = multiply_each_item_for_two_rs([1, 2, 3]);

        expect(result).toEqual([2, 4, 6]);
    });

    it("should fail using Array converter with incorrect param", () => {
        expect(() => {
            multiply_each_item_for_two_rs([1, 'asdf']);
        }).toThrow(new TypeError('failed to downcast any to number'));
    });

    it("should fail using Array converter without param", () => {
        expect(() => {
            multiply_each_item_for_two_rs();
        }).toThrow(new TypeError('not enough arguments'));
    });
});