const { fibonacci_rs, multiply_each_item_for_two_rs, generate_hashes_rs, generate_hash_rs } = require("./index.node");
const { randomBytes, createHash } = require('crypto')

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

console.log('------------------------------------------------------')
console.time('Generate 100K hashes in Rust')
generate_hashes_rs(100000);
console.timeEnd('Generate 100K hashes in Rust')
console.log('------------------------------------------------------')

const inputStrings = Array.from({ length: 100000 }, () => randomBytes(70).toString('hex'))

console.time('Generate 100K hashes in JS')
for (let i = 0; i < inputStrings.length - 1; i++) {
    createHash('sha256').update(inputStrings[i]).digest('hex')
}
console.timeEnd('Generate 100K hashes in JS')
console.log('------------------------------------------------------')

let time = 0n
for (let i = 0; i < inputStrings.length - 1; i++) {
    const start = process.hrtime.bigint()
    generate_hash_rs(inputStrings[i])
    const end = process.hrtime.bigint()
    time = time + (end - start)
}

console.log('Generate 100K hashes using Rust hash function:')
const averageTime = time / 100000n
console.log('Average time in nanoseconds:', Number(averageTime), 'ns')
console.log('Average time in milliseconds:', Number((averageTime * 10000n) / 1000000n) / 10000, 'ms')
console.log('------------------------------------------------------')

let time2 = 0n
for (let i = 0; i < inputStrings.length - 1; i++) {
    const start2 = process.hrtime.bigint()
    createHash('sha256').update(inputStrings[i]).digest('hex')
    const end2 = process.hrtime.bigint()
    time2 = time2 + (end2 - start2)
}

console.log('Generate 100K hashes using JS hash function:')
const averageTime2 = time2 / 100000n
console.log('Average time in nanoseconds:', Number(averageTime2), 'ns')
console.log('Average time in milliseconds:', Number((averageTime2 * 10000n) / 1000000n) / 10000, 'ms')
console.log('------------------------------------------------------')