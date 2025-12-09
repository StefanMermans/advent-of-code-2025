export type Problem = [number[], string];

export interface Input {
    next(): null|Problem;
}

export function solveProblem(numbers: number[], symbol: string): number {
    switch (symbol) {
        case '+':
            return numbers.reduce((a, b) => a + b, 0);
        case '*':
            return numbers.reduce((a, b) => a * b, 1);
        default:
            throw new Error(`Unknown symbol: ${symbol}`);
    }
}

export function solvePuzzle(input: Input) {
    let sum = 0;
    
    while (true) {
        const problem = input.next();   

        if (!problem) {
            break;  
        }

        const answer = solveProblem(...problem);
        sum += answer;
    }

    return sum;
}
