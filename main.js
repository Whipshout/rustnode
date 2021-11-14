const { fibonacci_rs, multiply_each_item_for_two_rs } = require("./index.node");

try {
    const result = fibonacci_rs('10');
    console.log(result);
} catch (err) {
    console.log('Fibonacci error');
    console.log(err);
}

try {
    const numbers = multiply_each_item_for_two_rs([1, 2, 3]);
    console.log(numbers);
} catch (err) {
    console.log('Array error');
    console.log(err);
}
