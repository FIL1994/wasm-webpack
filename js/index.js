import maleNames from "./data/maleNames";
import femaleNames from "./data/femaleNames";

window.maleNames = maleNames;
window.femaleNames = femaleNames;

function getRandomNumber(max) {
  return Math.floor(Math.random() * max);
}

import("../crate/pkg").then(module => {
  module.run();
  window.module = module;
  const names = maleNames.concat(femaleNames);

  const { NameBuilder } = module;
  const nameBuilder = NameBuilder.new(names);

  const TIMES = 2000;
  loop(
    TIMES,
    () => {
      const name = nameBuilder.get_random_name();
    },
    "WASM"
  );

  loop(
    TIMES,
    () => {
      const name = names[getRandomNumber(names.length)];
    },
    "JS"
  );

  const start = performance.now();
  const results = nameBuilder.get_random_names(TIMES);
  const end = performance.now();
  console.log(`WASM Looped - Total: ${end - start}`);

  console.log(results);
});

function loop(times, callback, title) {
  const start = performance.now();
  for (let i = 0; i < times; i++) {
    callback();
  }
  const end = performance.now();
  console.log(`${title} - Total: ${end - start}`);
}
