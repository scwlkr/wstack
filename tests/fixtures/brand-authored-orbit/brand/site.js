"use strict";
for (const button of document.querySelectorAll("[data-color]")) button.addEventListener("click", async () => {
  const value=button.dataset.color, field=document.querySelector("#color-value"), status=document.querySelector("#color-status");
  field.value=value;
  try { if(navigator.clipboard && window.isSecureContext) await navigator.clipboard.writeText(value);
    else {field.focus();field.select();if(!document.execCommand("copy"))throw Error("denied");}
    status.textContent="Color copied: "+value;
  } catch {field.focus();field.select();status.textContent="Color selected. Press Command+C or Ctrl+C.";}
});
for(const button of document.querySelectorAll("[data-preview]")) button.addEventListener("click",()=>{
  const dialog=document.querySelector("#art-preview"); dialog.querySelector("img").src=button.dataset.preview;
  dialog.querySelector("h2").textContent=button.dataset.title;dialog.showModal();
});
document.querySelector("#close-preview").addEventListener("click",()=>document.querySelector("#art-preview").close());
