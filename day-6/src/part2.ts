import { solvePuzzle, type Input, type Problem } from "./shared";

class Part2Input implements Input {
    problems: Problem[];

    constructor(problems: Problem[]) {
        this.problems = problems;
    }

    next(): null|Problem {
        return this.problems.pop() ?? null;
    }
}

function parseInput2(input: string): Part2Input {
    const lines = input
        .split('\n')
        .filter(line => line.length > 0)
    const symbols = lines
        .pop()!
        .trim()
        .split(/ +/);
    let problemNumbers: number[] = [];
    const problems: Problem[] = [];

    while (true) {
        if (lines[0]?.length === 0) {
              const problem: Problem = [
                problemNumbers,
                symbols.pop()!,
            ]
            problems.push(problem);
            break;
        }

        let currentValue = '';
        
        for (let i = 0; i < lines.length; i++) {
            const line = lines[i];
            currentValue += line!.at(-1);
            lines[i] = line!.slice(0, -1);
        }

        currentValue = currentValue.trim();

        if (currentValue.length === 0) {
            const problem: Problem = [
                problemNumbers,
                symbols.pop()!,
            ]
            problemNumbers = [];
            problems.push(problem);
        } else {
            problemNumbers.push(Number(currentValue.trim()));
        }
    }

    return new Part2Input(problems);
}

export function part2(input: string) {
    const data = parseInput2(input);
    const result = solvePuzzle(data);

    console.log("part2 result:", result);
}