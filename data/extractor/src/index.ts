import { readFileSync, writeFileSync } from "fs";

const fileSa = JSON.parse(readFileSync("../raw/vnsf.json", "utf-8"));
const fileVns = JSON.parse(readFileSync("../raw/vnsfbi.json", "utf-8"));

const resultsSa = fileSa.r as Array<{ profit: number; name: string }>;
const resultsVns = fileVns.r as Array<{ profit: number; name: string }>;
// const profitsAndName = results.map(r => ({ profit: r.profit, name: r.name }));

const orderFile = readFileSync("./src/inputs/eli51.txt", "utf-8")
  .split(/\r|\n/)
  .map(name => "./input-b/instances/eil51-thop/" + name);

const orderFile2 = readFileSync("./src/inputs/pr107.txt", "utf-8")
  .split(/\r|\n/)
  .map(name => "./input-b/instances/pr107-thop/" + name);

const orderFile3 = readFileSync("./src/inputs/a280.txt", "utf-8")
  .split(/\r|\n/)
  .map(name => "./input-b/instances/a280-thop/" + name);

const orderFile4 = readFileSync("./src/inputs/dsj1000.txt", "utf-8")
  .split(/\r|\n/)
  .map(name => "./input-b/instances/dsj1000-thop/" + name);

let vnsWins = 0;
let saWins = 0;
let draw = 0;
let vnsBetterSum = 0;
let saBetterSum = 0;

const howBetter = (better: number, worse: number): number => {
  // better -- x
  // worse -- 100
  const x = (better * 100) / worse;
  const res = x - 100;

  return res;
};

[...orderFile, ...orderFile2, ...orderFile3, ...orderFile4].forEach(name => {
  const obSa = resultsSa.find(x => x.name === name);
  const obVns = resultsVns.find(x => x.name === name);

  if (!obSa || !obVns) {
    throw Error(`${name} não existe`);
  }

  if (obSa.profit === obVns.profit) {
    draw += 1;
  } else if (obSa.profit > obVns.profit) {
    saBetterSum += howBetter(obSa.profit, obVns.profit);
    saWins += 1;
  } else {
    vnsBetterSum += howBetter(obVns.profit, obSa.profit);
    vnsWins += 1;
  }
});

const txt = `
  vns-wins: ${vnsWins}
  sa-wins: ${saWins}
  draw: ${draw}
  
  how-better-vns: ${vnsBetterSum}
  how-better-sa: ${saBetterSum}
  vns-sa: ${vnsBetterSum - saBetterSum}
  `;

writeFileSync("./results/results22222.txt", txt);

const profits = [...orderFile, ...orderFile2, ...orderFile3, ...orderFile4].map(
  name => {
    const obVns = resultsVns.find(x => x.name === name);

    if (!obVns) {
      throw Error(`${name} não existe`);
    }

    return obVns.profit;
  }
);

writeFileSync("./results/results-vnsfbi.txt", profits.join("\n"));
