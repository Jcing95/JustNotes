const { invoke } = window.__TAURI__.tauri;

let userInput;
let content;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  const newPara = document.createElement("p");
  newPara.textContent = await invoke("format", { name: userInput.value });
  content.appendChild(newPara);
}

window.addEventListener("DOMContentLoaded", () => {
  userInput = document.querySelector("#user-input");
  content = document.querySelector("#content");
  document.querySelector("#user-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});
