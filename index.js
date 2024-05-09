const JSONbig = require('json-bigint')({ useNativeBigInt: true });
const wasmUrl = chrome.runtime.getURL('runes.wasm');
const urlObj = new URL(wasmUrl);
__webpack_public_path__ = urlObj.protocol + '//' + urlObj.host + '/';
const runes = require('./pkg');


function replaceOP_RETURN(dom, json) {
  if (json.Runestone) {
    const { etching, mint, edicts, pointer } = json.Runestone;
    const rune = etching ? etching.rune : null;
    const symbol = etching ? etching.symbol : null;
    const premine = etching ? etching.premine : null;
    const spacers = etching ? etching.spacers : null;
    const divisibility = etching ? etching.divisibility : null;
    const turbo = etching? etching.turbo : false;
    let amount = null;
    let cap = null;
    let start_height = null;
    let end_height = null;
    let start_offset = null;
    let end_offset = null;
    const terms = etching ? etching.terms : null;
    if (terms) {
      amount = terms.amount;
      cap = terms.cap;
      if (terms.height) {
        start_height = terms.height[0];
        end_height = terms.height[1];
      }
      if (terms.offset) {
        start_offset = terms.offset[0];
        end_offset = terms.offset[1];
      }
    }

    dom.innerHTML = `
      <div style="
          display: flex;
      ">
        
        <div style="
          margin: 0 8px 0 0;
        ">
          RuneStone:
        </div>
        <div style="
          
        ">
          ${
            (etching !== null || (edicts !== null && edicts.length > 0) || mint !== null || pointer !== null) ? `
              ${
                etching !== null ? `<div>
                etching: 
                  <div style="
                    margin-left: 24px;
                  ">
                    ${
                      rune ? `
                      <div>rune: ${spacers_rune(rune, spacers)}</div>
                      ` : ''
                    }
                    ${
                      symbol ? `
                      <div>symbol: ${symbol}</div>
                      ` : ''
                    }
                    ${
                      (amount !== null || cap !== null || start_height !== null || end_height !== null || start_offset !== null || end_offset !== null) ? `
                        <div>terms: 
                          <div style="
                            margin-left: 24px;
                          ">
                            ${
                              amount !== null ? `
                                <div>amount: ${amount}</div>
                              ` : ''
                            }
                            ${
                              cap !== null ? `
                                <div>cap: ${cap}</div>
                              ` : ''
                            }
                            ${
                              start_height !== null || end_height !== null ? `
                                <div>start: [${start_height === null ? 'None' : start_height}, ${end_height === null ? 'None' : end_height}]</div>
                              ` : ''
                            }
                            ${
                              start_offset !== null || end_offset !== null ? `
                                <div>offset: [${start_offset === null ? 'None' : start_offset}, ${end_offset === null ? 'None' : end_offset}]</div>
                              ` : ''
                            }
                          </div>
                        </div>
                      ` : ''
                    }
                    ${
                      premine !== null ? `
                      <div>premine: ${premine}</div>
                      ` : ''
                    }
                    ${
                      spacers !== null ? `
                      <div>spacers: ${spacers}</div>
                      ` : ''
                    }
                    ${
                      divisibility !== null ? `
                      <div>divisibility: ${divisibility}</div>
                      ` : ''
                    }
                    ${
                      turbo !== null ? `
                      <div>turbo: ${turbo}</div>
                      ` : ''
                    }
                  </div>
                </div>` : ''
              }
            
              ${
                mint !== null ? `
                  <div>
                    mint: ${mint}
                  </div>
                ` : ''
              }
    
    
              ${
                edicts && edicts.length > 0 ? `<div>
                edicts: 
                  ${
                    edicts.map((edict, index) => {
                      return `<div style="
                        margin-left: 24px;
                      ">
                        <div>id: ${edict.id}</div>
                        <div>amount: ${edict.amount}</div>
                        <div>output: ${edict.output}</div>
                      </div>`;
                    }).join(`<div style="
                      margin-left: 24px;
                    ">,</div>`)
                  }
                </div>` : ''
              }

              ${
                pointer !== null ? `
                <div>pointer: ${pointer}</div>
                ` : ''
              }
            ` : 'None'


          }
        </div>
      </div>  
    `;
  } else if (json.Cenotaph) {
    const { etching, flaw, mint } = json.Cenotaph;
    dom.innerHTML = `
      <div style="
          display: flex;
      ">
        
        <div style="
          margin: 0 8px 0 0;
        ">
          Cenotaph:
        </div>
        <div style="
          
        ">
        
          ${
            etching !== null ? `
              <div>
                etching: ${etching}
              </div>
            ` : ''
          }

          ${
            flaw !== null ? `
              <div>
                flaw: ${flaw}
              </div>
            ` : ''
          }

          ${
            mint !== null ? `
              <div>
                mint: ${mint}
              </div>
            ` : ''
          }
        </div>
      </div>  
    `;
  }
}

function replaceWitness(dom, { list }) {
  list.forEach(url => {
    const iframe = document.createElement('iframe');
    iframe.scrolling = 'no';
    iframe.loading = 'lazy';
    iframe.style.cssText = `
      display: block;
      width: 300px;
      height: 300px;
      border: 0;
      border-radius: 16px;
      cursor: pointer;
      margin-top: 8px;
    `;
    iframe.src = `https://ordinals.com${url}`;
    dom.appendChild(iframe);
  }); 
}

function spacers_rune(rune, spacers) {
  let result = '';
  for (let i = 0; i < rune.length; i++) {
    result += rune[i];
    if (i < rune.length - 1 && (spacers & (1 << i)) !== 0) {
      result += '•';
    }
  }
  return result;
}

const cache = {};
async function fetchTransction(txid, network) {
  const url = `${location.protocol}//${location.host}${network === 'testnet' ? '/testnet' : ''}/api/tx/${txid}`;
  if (cache[url]) {
    return cache[url];
  }
  const response = await fetch(url);
  const json = await response.json();
  cache[url] = json;
  return json;
}


async function fetchAddressTransctions(address, after_txid, network) {
  const response = await fetch(`${location.protocol}//${location.host}${network === 'testnet' ? '/testnet' : ''}/api/address/${address}/txs${after_txid ? `?after_txid=${after_txid}` : ''}`);
  const arr = await response.json();
  return arr;
}



async function fetchBlockHash(height, network) {
  const url = `${location.protocol}//${location.host}${network === 'testnet' ? '/testnet' : ''}/api/block-height/${height}`;
  if (cache[url]) {
    return cache[url];
  }
  const response = await fetch(url);
  const hash = await response.text();
  cache[url] = hash;
  return hash;
}


async function fetchBlockTransctions(block, page, network) {
  if (block.length < 64) {
    block = await fetchBlockHash(block, network);
  }
  const url = `${location.protocol}//${location.host}${network === 'testnet' ? '/testnet' : ''}/api/block/${block}/txs/${(page - 1) * 25}`;
  if (cache[url]) {
    return cache[url];
  }
  const response = await fetch(url);
  const arr = await response.json();
  cache[url] = arr;
  return arr;
}

function replace_runes(arr, vout_len, container = document) {
  const addresses = container.querySelectorAll('.table-tx-vout .address-cell');
  if (addresses.length > 0) {
    if (addresses.length < vout_len) {
      const observer_vout = new MutationObserver((mutations, obs) => {
        const list = container.querySelectorAll('.table-tx-vout .address-cell');
        if (list.length === vout_len) {
          arr.forEach(({ index, json }) => {
            try {
              replaceOP_RETURN(list[index], json);
            } catch (err) {
            }
          });
          obs.disconnect();
          return;
        }
      });

      container.querySelectorAll('.table-tx-vout').forEach((table) => {
        observer_vout.observe(table, {
          childList: true,
          subtree: true
        });
      })
    }

    arr.forEach(({ index, json }) => {
      try {
        replaceOP_RETURN(addresses[index], json);
      } catch (err) {
      }
    });
    return true;
  }
}

function replace_inscriptions(arr, vin_len, container = document) {
  const addresses = container.querySelectorAll('.table-tx-vin .address-cell');
  if (addresses.length > 0) {
    if (addresses.length < vin_len) {
      const observer_vout = new MutationObserver((mutations, obs) => {
        const list = container.querySelectorAll('.table-tx-vin .address-cell');
        if (list.length === vin_len) {
          arr.forEach(({ index, json }) => {
            try {
              replaceWitness(list[index], json);
            } catch (err) {
            }
          });
          obs.disconnect();
          return;
        }
      });

      container.querySelectorAll('.table-tx-vin').forEach((table) => {
        observer_vout.observe(table, {
          childList: true,
          subtree: true
        });
      })
    }

    arr.forEach(({ index, json }) => {
      try {
        replaceWitness(addresses[index], json);
      } catch (err) {
      }
    });
    return true;
  }
}

async function getRunes(vout) {
  const arr = (await Promise.all(vout.map(async (output, index) => {
    if (!output || output.scriptpubkey_type !== 'op_return' || !output.scriptpubkey.startsWith('6a5d')) {
      return;
    }
    const { decipher } = await runes;
    try {
      const result = decipher(vout.length, output.scriptpubkey);
      return {
        index,
        json: JSONbig.parse(result),
      };
    } catch(err) {
      return;
    }
  }))).filter(item => !!item);
  return arr;
}

async function getInscriptions(txid, vin) {
  const input = vin.find(input => input.witness && input.witness.length === 3)
  if (!input) {
    return [];
  }
  const url = `https://ordinals.com/tx/${txid}`;
  try {
    const res = await fetch(url);
    if (!res.ok) {
      return [];
    }
    const html = await res.text();
    const parser = new DOMParser();
    const doc = parser.parseFromString(html, 'text/html');
    const list = [...doc.querySelectorAll('.thumbnails iframe')].map(iframe => iframe.getAttribute('src'));
    return [{
      index: vin.indexOf(input),
      json: {
        list,
      },
    }];
  } catch(err) {
    return [];
  }
}


async function run_txid(txid, network) {
  let currentURL = url;
  const { vin, vout } = await fetchTransction(txid, network);
  if (currentURL !== url) {
    return;
  }
  const [vout_arr, vin_arr] = await Promise.all([getRunes(vout), getInscriptions(txid, vin)]);
  
  onReady(() => {
    if (currentURL !== url) {
      return;
    }
    replace_runes(vout_arr, vout.length);
    replace_inscriptions(vin_arr, vin.length);
  });
}


async function getPage() {
  const obj = new URL(location.href);
  const page = obj.searchParams.get('page');
  if (page) {
    return page;
  }
  while (true) {
    try {
      const page = document.querySelector('.pagination .active a').innerText;
      return page;
    } catch {
      await new Promise(r => setTimeout(r, 100));
    }
  }
}

async function run_block(block, network) {
  let currentURL = url;
  const page = await getPage();
  if (currentURL !== url) {
    return;
  }
  const list = await fetchBlockTransctions(block, page, network);
  if (currentURL !== url) {
    return;
  }
  onReady(() => {
    if (currentURL !== url) {
      return;
    }
    const tx = document.querySelectorAll('.tx-page-container');
    [...tx].forEach(async (dom, index) => {
      const [vout_arr, vin_arr] = await Promise.all([getRunes(list[index].vout), getInscriptions(list[index].txid, list[index].vin)]);
      replace_runes(vout_arr, list[index].vout.length, dom.nextSibling.nextSibling);
      replace_inscriptions(vin_arr, list[index].vin.length, dom.nextSibling.nextSibling);
    });
  });
}

function onReady(cb) {
  const addresses = document.querySelectorAll('.table-tx-vout .address-cell');
  if (addresses.length > 0) {
    cb();
    return true;
  }
  const observer = new MutationObserver((mutations, obs) => {
    const addresses = document.querySelectorAll('.table-tx-vout .address-cell');
    if (addresses.length > 0) {
      obs.disconnect();
      cb();
    }
  });
  observer.observe(document.body, {
    childList: true,
    subtree: true
  });
}

async function run_address(address, network) {
  let currentURL = url;
  const list = [];
  const arr = await fetchAddressTransctions(address, undefined, network);
  if (currentURL !== url) {
    return;
  }

  onReady(async () => {
    if (currentURL !== url) {
      return;
    }
    let after_txid;
    if (arr.length < 50) {
      last_page = true;
    }
    after_txid = arr[arr.length - 1].txid;
    list.push(...arr);

    async function update() {
      const tx = document.querySelectorAll('.tx-page-container');
      let last_page;
      while(tx.length > list.length) {
        const arr = await fetchAddressTransctions(address, after_txid, network);
        if (currentURL !== url) {
          return;
        }
        if (arr.length === 0) {
          break;
        }
        after_txid = arr[arr.length - 1].txid;
        list.push(...arr);
        if (arr.length < 50) {
          last_page = true;
          break;
        }
      }
      
  
      [...tx].forEach(async (dom, index) => {
        const [vout_arr, vin_arr] = await Promise.all([getRunes(list[index].vout), getInscriptions(list[index].txid, list[index].vin)]);
        replace_runes(vout_arr, list[index].vout.length, dom.nextSibling.nextSibling);
        replace_inscriptions(vin_arr, list[index].vin.length, dom.nextSibling.nextSibling);
      });
  
      if (last_page && tx.length === list.length) {
        return true;
      }
    }

    if (await update()) {
      return;
    };

    const observer_vout = new MutationObserver((mutations, obs) => {
      if (currentURL!== url) {
        obs.disconnect();
        return;
      }
      update().then(last_page => {
        if (last_page) {
          obs.disconnect();
        }
      }).finally(() => {
        if (currentURL!== url) {
          obs.disconnect();
        }
      });
    });

    observer_vout.observe(document.querySelector('app-transactions-list > div'), {
      childList: true,
      subtree: false
    });
  });
}



function routeChange() {  
  const obj = new URL(location.href);
  const pathname = obj.pathname;
  let network = 'mainnet';
  if(pathname.startsWith('/testnet')) {
    network = 'testnet';
  }
  const txid_match = pathname.match(/\/tx\/([a-fA-F0-9]+)/);
  if (txid_match) {
    const txid = txid_match[1];
    if (document.readyState === "loading") {
      let currentURL = url;
      document.addEventListener("DOMContentLoaded", () => {
        if (currentURL !== url) {
          return;
        }
        run_txid(txid, network);
      });
    } else {
      run_txid(txid, network);
    }
    return;
  }

  const block_match = pathname.match(/\/block\/([a-fA-F0-9]+)/);
  if (block_match) {
    const block = block_match[1];
    if (document.readyState === "loading") {
      document.addEventListener("DOMContentLoaded", () => {
        run_block(block, network);
      });
    } else {
      run_block(block, network);
    }
  }

  const address_match = pathname.match(/\/address\/([a-zA-Z0-9]+)/);
  if (address_match) {
    const address = address_match[1];
    if (document.readyState === "loading") {
      document.addEventListener("DOMContentLoaded", () => {
        run_address(address, network);
      });
    } else {
      run_address(address, network);
    }
  }
}






let url = location.href;
routeChange();

setInterval(() => {
  if (location.href === url) {
    return;
  }
  url = location.href;
  routeChange();
}, 20);
