"use strict";
const search = document.querySelector("#search");
const category = document.querySelector("#category");
const scope = document.querySelector("#scope");
const cards = [...document.querySelectorAll(".asset")];
for (const card of cards) {
  const link = card.querySelector("a[data-asset][download]");
  if (!link) continue;
  link.dataset.wstackDownload = link.dataset.asset;
  const row = window.WstackBrand.assets.find(item => item.path === link.dataset.asset);
  card.dataset.role = (row && row.role) || "current";
  card.querySelector(".eyebrow").append(` · ${card.dataset.role}`);
  const image = card.querySelector(".preview img");
  if (image) {
    const button = document.createElement("button");
    button.type = "button";
    button.dataset.wstackPreview = link.dataset.asset;
    button.setAttribute("aria-label", `Enlarge ${image.alt}`);
    button.style.cssText = "border:0;background:transparent;width:100%;height:100%;cursor:pointer";
    image.replaceWith(button);
    button.append(image);
  }
}
function filter() {
  const query = search.value.toLowerCase().trim();
  let visible = 0;
  for (const card of cards) {
    card.hidden = !card.dataset.search.includes(query) ||
      (category.value !== "" && card.dataset.category !== category.value) ||
      (scope.value !== "all" && card.dataset.role !== "current");
    if (!card.hidden) visible += 1;
  }
  const total = scope.value === "all" ? cards.length : cards.filter(card => card.dataset.role === "current").length;
  document.querySelector("#result-count").textContent = `${visible} of ${total} ${scope.value === "all" ? "resources" : "current assets"}`;
  document.querySelector("#empty").hidden = visible !== 0;
}
search.addEventListener("input", filter);
category.addEventListener("change", filter);
scope.addEventListener("change", filter);
document.querySelector("#copy").addEventListener("click", () => {
  window.WstackBrand.copy(document.querySelector("#style-json"), document.querySelector("#copy-status"));
});
filter();
