#!/usr/bin/env node

for (var n = 1; n <= 100; ++n) {
  console.log ((function() {
    var d3 = n % 3 == 0;
    var d5 = n % 5 == 0;
    if (d3 && d5) return "fizzbuzz";
    else if (d3) return "fizz";
    else if (d5) return "buzz";
    else return `${n}`;
  })());
}
