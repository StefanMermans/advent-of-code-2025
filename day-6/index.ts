import { Command } from 'commander';

const program = new Command();

program
  .argument('<inputFile>', 'path to the puzzle input file')
  .action(async (inputFile) => {
    const input = await Bun.file(inputFile).text();

    part1(input);
  });

program.parse(process.argv);

class Data {
    numbers: number[][];
    symbols: string[];

    constructor(numbers: number[][], symbols: string[]) {
        this.numbers = numbers;
        this.symbols = symbols;
    }

    getNext(): null|[number[], string] {
        if (this.symbols.length === 0) {
            return null;
        } 

        const numbers = this.numbers.reduce((result, numbers) => {
            result.push(numbers.shift()!);

            return result;
        }, []);
        
        const symbol = this.symbols.shift()!;

        return [numbers, symbol];
    }
}

function splitLine(line: string): string[] {
    return line.split(/ +/);
}

function parseNumberLine(line: string): number[] {
    return splitLine(line).map(Number);
}

function parseInput(input: string): Data {
    const lines = input
        .split('\n')
        .filter(line => line.length > 0)
        .map(line => line.trim());
    const symbolLine = splitLine(lines.pop()!);
    const numbers = lines.map(parseNumberLine);

    return new Data(numbers, symbolLine);
}

function solveProblem(numbers: number[], symbol: string): number {
    switch (symbol) {
        case '+':
            return numbers.reduce((a, b) => a + b, 0);
        case '*':
            return numbers.reduce((a, b) => a * b, 1);
        default:
            throw new Error(`Unknown symbol: ${symbol}`);
    }
}

function part1(input: string) {
    const data = parseInput(input);
    let sum = 0;

    while (true) {
        const problem = data.getNext();
        console.log("problem:", problem);
        
        if (!problem) {
            break;  
        }

        const answer = solveProblem(...problem);
        console.log("answer:", answer);
        sum += answer;
    }

    console.log("part1 result:", sum);
}
