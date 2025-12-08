const [freshList, availableList] = getInput();

type Range = [number, number];

function parseRange(range: string): Range {
    return range.split("-").map(Number);
}

function getInput(): [Range, string[]] {
    const input = Deno.readTextFileSync("input.txt");
    const [freshList, availableList] = input.split("\n\n");

    return [freshList.split("\n").map(parseRange), availableList.split("\n")];
}

function isInRange([min, max]: Range, ingredient: string): boolean {
    const value = Number(ingredient);

    if (value >= min && value <= max) {
        return true;
    }

    return false;
}

function totalFreshIngredients(targetRanges: Range[]): number {
    let total = 0;
    const rangesToCheck: Range[] = targetRanges.slice();
    const checkedRanges: Range[] = [];

    while (rangesToCheck.length > 0) {
        const range = rangesToCheck.shift()!;
        const [min, max] = range;
        let checked = true;

        if (checkedRanges.length === 0) {
            total += max - min + 1;
            checkedRanges.push(range);
            continue;
        }

        for (const checkedRange of checkedRanges) {
            const [checkedMin, checkedMax] = checkedRange;

            if (max >= checkedMin && max <= checkedMax) {
                if (min >= checkedMin && min <= checkedMax) {
                    checked = false
                    break;
                } else {
                    rangesToCheck.push([min, checkedMin - 1]);
                    checked = false
                    break;
                }
            } else if (min >= checkedMin && min <= checkedMax) {
                const newRange = [checkedMax + 1, max];
                checked = false
                rangesToCheck.push(newRange);
                break;
            } else if (min < checkedMin && max > checkedMax) {
                checked = false
                rangesToCheck.push([min, checkedMin - 1]);
                rangesToCheck.push([checkedMax + 1, max]);
                break;
            }
            
        }

        if (checked) {
            total += max - min + 1;
            checkedRanges.push(range);
        }
    }

    return total;
}

function isFresh(ingredient: string): boolean {
    for (const range of freshList) {
        if (isInRange(range, ingredient)) {
            return true;
        }
    }

    return false;
}

console.log(`Fresh ingredients: ${availableList.filter(isFresh).length}`);
console.log(`Total fresh ingredients: ${totalFreshIngredients(freshList)}`);
