"use strict";
const search = document.querySelector("#search");
const category = document.querySelector("#category");
const cards = [...document.querySelectorAll(".asset")];
function filter() {
  const query = search.value.toLowerCase().trim();
  let visible = 0;
  for (const card of cards) {
    card.hidden = !card.dataset.search.includes(query) ||
      (category.value !== "" && card.dataset.category !== category.value);
    if (!card.hidden) visible += 1;
  }
  document.querySelector("#result-count").textContent = `${visible} of ${cards.length} resources`;
  document.querySelector("#empty").hidden = visible !== 0;
}
search.addEventListener("input", filter);
category.addEventListener("change", filter);
document.querySelector("#copy").addEventListener("click", async () => {
  const field = document.querySelector("#style-json");
  const status = document.querySelector("#copy-status");
  try {
    if (navigator.clipboard && window.isSecureContext) {
      await navigator.clipboard.writeText(field.value);
    } else {
      field.focus();
      field.select();
      if (!document.execCommand("copy")) throw new Error("Clipboard unavailable");
    }
    status.textContent = "Style JSON copied.";
  } catch {
    field.focus();
    field.select();
    status.textContent = "Copy unavailable. Style JSON selected; press Ctrl+C or Command+C.";
  }
});
