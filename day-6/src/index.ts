import { Command } from 'commander';
import { part2 } from './part2';
import { part1 } from './part1';

const program = new Command();

program
  .argument('<inputFile>', 'path to the puzzle input file')
  .action(async (inputFile) => {
    const input = await Bun.file(inputFile).text();

    part1(input);
    part2(input);
  });

program.parse(process.argv);









