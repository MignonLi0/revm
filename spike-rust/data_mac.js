import * as fs from 'fs';

const content = fs.readFileSync('./output_mac.log', 'utf-8');
const lines = content.split('\n');

const entries= [];

let current = {};
for (const line of lines) {
  if (line.startsWith('=== ') && line.endsWith(' ===')) {
    current = { name: line.slice(4, -4).trim() };
  } else if (line.includes('Unit Count')) {
    current.unitCount = parseInt(line.split('=')[1].trim());
  } else if (line.includes('Gas/Unit')) {
    current.gasPerUnit = parseFloat(line.split('=')[1].trim());
  } else if (line.includes('instructions retired')) {
    const match = line.match(/^\s*([\d,]+)\s+instructions retired/);
    current.instructions = parseInt(match[1].replace(/,/g, ''));

    if (Object.keys(current).length === 4) {
      entries.push(current);
    }
    current = {};
  }
}

console.log(
  [
    'Gas/Unit'.padStart(10),
    'Unit Count'.padStart(12),
    'Total insn'.padStart(15),
    'insn/Unit'.padStart(15),
    'insn/Gas'.padStart(15),
    'Operation'.padStart(40),
  ].join('  ')
);

for (const e of entries) {
  const insnPerUnit = e.instructions / e.unitCount;
  const insnPerGas =insnPerUnit / e.gasPerUnit;
  console.log(
    [
      e.gasPerUnit.toString().padStart(10),
      e.unitCount.toString().padStart(12),
      e.instructions.toString().padStart(15),
      insnPerUnit.toFixed(0).padStart(15),
      insnPerGas.toFixed(2).padStart(15),
      `${e.name.padStart(40)}`,
    ].join('  ')
  );
}
