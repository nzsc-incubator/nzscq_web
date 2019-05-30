import("../crate/pkg").then(module => {
  const app = module.run();

  window.addEventListener("resize", () => {
    app.resize();
  });
  window.addEventListener("orientationchange", () => {
    app.resize();
  });
});
