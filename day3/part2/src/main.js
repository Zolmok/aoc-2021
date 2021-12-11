
function find_most_common(diagnostics) {
  return '111010110111';
}

function calculate_oxygen_generator(diagnostics) {
  let most_common = find_most_common(diagnostics);
  let diagnostics_remaining = [];
  let diagnostics_to_check = diagnostics;
  let oxygen_generator_rating;

  for (let n = 0; n < 12; n++) {
    diagnostics_to_check.forEach((diagnostic_to_check) => {
      let diagnostic_digit = diagnostic_to_check[n];
      let most_common_digit = most_common[n];

      if (most_common_digit == diagnostic_digit) {
        const found = diagnostics_remaining.find((diagnostic_remaining) => diagnostic_remaining === diagnostic_to_check);

        if (!found) {
          diagnostics_remaining.push(diagnostic_to_check);
        }
      }
    }); 

    diagnostics_to_check = diagnostics_remaining;
  }

  console.log('total: {} remaining: {}', diagnostics.length, diagnostics_remaining.length);
  return oxygen_generator_rating;
}

const diagnostics = [
  '110110000100',
  '010110011100',
  '001001101010',
  '011011101010',
  '101100010000',
  '010100110111',
  '010110101101',
  '111010110111',
  '100111110001',
  '001010110000',
  '011011000001'
];

const oxygen_geneator = calculate_oxygen_generator(diagnostics);
