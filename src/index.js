const addon = require('../native');

const data = 'Some Keys Here What You Would Like To Hash'
const seed = 123456789

const hash = addon.hash(data, seed)

console.log(hash)
