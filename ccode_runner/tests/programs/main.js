const readline = require('node:readline').createInterface({
    input: process.stdin,
    output: process.stdout
});

readline.question('', number => {
    const square = number * number;
    console.log(square);
    readline.close();
});
