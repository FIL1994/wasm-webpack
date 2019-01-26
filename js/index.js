import("../crate/pkg").then(module => {
  module.run();
  window.module = module;
});
