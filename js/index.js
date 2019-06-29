import loadImageGetter from "./loadImageGetter";

Promise.all([import("../crate/pkg"), loadImageGetter()]).then(
  ([module, getImage]) => {
    const app = module.run(getImage);
    window.app = app;

    window.addEventListener("resize", () => {
      app.resize();
    });
    window.addEventListener("orientationchange", () => {
      app.resize();
    });
    app.call_with_canvas(canvas => {
      canvas.addEventListener("click", ({ clientX, clientY }) => {
        app.on_click(clientX, clientY);
      });
    });

    drawLoop();

    function drawLoop() {
      requestAnimationFrame(drawLoop);

      app.draw_if_needed();
    }
  }
);

if ("serviceWorker" in navigator) {
  if (process.env.NODE_ENV === "production") {
    navigator.serviceWorker.register("./service-worker.js");
  } else {
    navigator.serviceWorker.ready.then(registration => {
      registration.unregister();
    });
  }
}
