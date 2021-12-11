const lineReader = require('readline').createInterface({
  input: require('fs').createReadStream('commands.txt')
});

let horizontal = 0;
let depth = 0;

function getCommands(line) {
  const words = line.split(' ');
  const command = words[0];
  const value = parseInt(words[1], 10);

  if (command == 'forward') {
    horizontal = horizontal + value;
  }
  if (command == 'up') {
    depth = depth - value;
  }
  if (command == 'down') {
    depth = depth + value;
  }
}

function processCommands() {
  const position = horizontal * depth;

  console.log('Position:', position);
}

lineReader.on('line', getCommands);
lineReader.on('close', processCommands)
