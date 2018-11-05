import { readFileSync, writeFileSync } from "fs";

const file = JSON.parse(readFileSync("../raw/sa3.json", "utf-8"));

const results = file.r as Array<{ profit: number; name: string }>;
const profitsAndName = results.map(r => ({ profit: r.profit, name: r.name }));

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

const profits = [...orderFile, ...orderFile2, ...orderFile3, ...orderFile4].map(
  name => {
    const ob = profitsAndName.find(x => x.name === name);

    if (!ob) {
      throw Error(`${name} não existe`);
    }

    return ob.profit;
  }
);

writeFileSync("./results/sa3.txt", profits.join("\n"));
