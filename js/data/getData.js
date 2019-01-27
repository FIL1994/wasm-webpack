let names = Array.from(document.querySelectorAll("tbody tr td"))
  .filter((x, i) => i % 4 === 0)
  .map(({ innerText: x }) => x[0] + x.slice(1).toLowerCase());
