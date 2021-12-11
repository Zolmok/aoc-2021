const lineReader = require('readline').createInterface({
  input: require('fs').createReadStream('commands.txt')
});

function processCommands(line, values) {
  const words = line.split(' ');
  const command = words[0];
  const value = parseInt(words[1], 10);

  if (command == 'forward') {
    values.horizontal = values.horizontal + value;
    values.depth = values.depth + (values.aim * value);
  }
  if (command == 'up') {
    values.aim = values.aim - value;
  }
  if (command == 'down') {
    values.aim = values.aim + value;
  }
}

function calculateCourse(values) {
  const position = values.horizontal * values.depth;

  console.log('Position:', position);
  return position;
}

function pilotSubmarine() {
  const values = {
    horizontal: 0,
    depth: 0,
    aim: 0
  };

  function processCommandsCurried(line) {
    return processCommands(line, values);
  }
  function calculateCourseCurried() {
    return calculateCourse(values);
  }

  lineReader.on('line', processCommandsCurried);
  lineReader.on('close', calculateCourseCurried)
}

pilotSubmarine();
