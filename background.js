
const wasmUrl = chrome.runtime.getURL('runes.wasm');
const obj = new URL(wasmUrl);
__webpack_public_path__ = obj.protocol + '//' + obj.host + '/';
const runes =require('./pkg');


chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
  if (!sender || sender.id !== chrome.runtime.id) {
    return;
  }
  if (request.method !== 'decipher') {
    return;
  }
  runes.then(({ decipher }) => {
    try {
      const result = decipher(...request.args);
      sendResponse(result);
    } catch(err) {
      sendResponse('error');
    }
  });
  return true;
});
