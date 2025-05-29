var tailwind;
(function () {
  if (tailwind !== undefined) {
    tailwind.config = {
      darkMode: "class",
    };
  } else {
    tailwind = {
      config: {
        darkMode: "class",
      },
    };
    console.log("tailwind not found");
  }
})();
