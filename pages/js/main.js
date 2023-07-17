console.log("Ahoj Vašek!");

fetch("https://localhost:7878/", {
  method: "POST",
  headers: {
    "Content-Type": "application/json",
  },
  body: JSON.stringify({
    name: "Václav",
    age: 20,
  }),
});
