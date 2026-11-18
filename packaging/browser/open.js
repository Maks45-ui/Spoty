const params = new URLSearchParams(location.search);
const kind = params.get("kind") ?? "";
const id = params.get("id") ?? "";
const status = document.getElementById("status");
const web = document.getElementById("web");

if (window !== window.top || !/^(track|album|artist|playlist|show|episode)$/.test(kind) || !/^[A-Za-z0-9]{22}$/.test(id)) {
  status.textContent = "Это неподдерживаемая ссылка Spotify.";
} else {
  web.href = `https://open.spotify.com/${kind}/${id}?spoty-web=1`;
  web.hidden = false;
  chrome.runtime.sendNativeMessage("io.github.maks45_ui.spoty", { kind, id }, (response) => {
    const failure = chrome.runtime.lastError;
    status.textContent = !failure && response?.ok
      ? "Ссылка передана в Spoty."
      : "Не удалось открыть Spoty. Установи его последнюю версию или открой ссылку в браузере.";
  });
}
