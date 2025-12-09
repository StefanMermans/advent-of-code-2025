import { solvePuzzle, type Input } from "./shared";

class Part1Input implements Input {
    numbers: number[][];
    symbols: string[];

    constructor(numbers: number[][], symbols: string[]) {
        this.numbers = numbers;
        this.symbols = symbols;
    }

    next(): null|[number[], string] {
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

function parseInput1(input: string): Part1Input {
    const lines = input
        .split('\n')
        .filter(line => line.length > 0)
        .map(line => line.trim())
    const symbolLine = splitLine(lines.pop()!);
    const numbers = lines.map(line => splitLine(line).map(Number));

    return new Part1Input(numbers, symbolLine);
}

export function part1(input: string) {
    const data = parseInput1(input);
    const result = solvePuzzle(data);
    console.log("part1 result:", result);
}