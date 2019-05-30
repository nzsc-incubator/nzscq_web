import loadImages from "./loadImages";

export default function loadImageGetter() {
  return loadImages().then(
    images =>
      function getImage(move) {
        const spaceless = move.replace(/\s/g, "");
        if (spaceless === "StrongSmash") {
          return images["Smash"];
        } else {
          return images[spaceless];
        }
      }
  );
}
