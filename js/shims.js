if (!Object.fromEntries) {
  Object.fromEntries = function fromEntries(entries) {
    return [...entries].reduce(
      (obj, [key, value]) => ({ ...obj, [key]: value }),
      {}
    );
  };
}

if (!Object.entries) {
  Object.entries = function entries(obj) {
    return Object.keys(obj).map(key => [key, obj[key]]);
  };
}
