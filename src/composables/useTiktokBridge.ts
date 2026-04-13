import { invoke } from '@tauri-apps/api/core'

/**
 * TikTok bridge — executes actions inside the TikTok webview via JS injection.
 *
 * - `init()` injects a persistent MutationObserver that hides unwanted UI elements.
 * - `setHidden(bool)` toggles visibility of hidden elements.
 * - `exec(action)` runs a named JS snippet (click, toggle, etc).
 *
 * To extend: add selectors to `HIDE_SELECTORS`, actions to `actions`.
 */

/** CSS selectors of elements to hide inside TikTok */
const HIDE_SELECTORS = [
  'button[data-e2e="arrow-left"]',
  'button[data-e2e="arrow-right"]',
  'form[data-e2e="search-box"]',
  'div[data-e2e="browse-ellipsis"]',
  'button[data-e2e="browse-close"]',
  'button[data-e2e="browse-sound"]',
  'div[class*="MiniPlayerContainer"]',
  'div[class*="StyledAnimationWrapper"]',
]

/** Build the init JS that runs once and watches forever */
function buildInitScript(): string {
  const selectors = JSON.stringify(HIDE_SELECTORS)
  return `
(function() {
  if (window.__meguri_init) return;
  window.__meguri_init = true;
  window.__meguri_hide = true;

  const SELECTORS = ${selectors};

  function applyAll() {
    var display = window.__meguri_hide ? 'none' : '';
    for (var sel of SELECTORS) {
      document.querySelectorAll(sel).forEach(function(el) {
        el.style.display = display;
      });
    }
  }

  function applyToNode(node) {
    if (!window.__meguri_hide) return;
    for (var sel of SELECTORS) {
      if (node.matches && node.matches(sel)) {
        node.style.display = 'none';
      }
    }
    if (node.querySelectorAll) {
      for (var sel of SELECTORS) {
        node.querySelectorAll(sel).forEach(function(el) {
          el.style.display = 'none';
        });
      }
    }
  }

  window.__meguri_toggle = function() {
    window.__meguri_hide = !window.__meguri_hide;
    applyAll();
    return window.__meguri_hide;
  };

  window.__meguri_setHidden = function(val) {
    window.__meguri_hide = val;
    applyAll();
  };

  function setup() {
    applyAll();
    new MutationObserver(function(mutations) {
      for (var m of mutations) {
        for (var node of m.addedNodes) {
          if (node.nodeType === 1) {
            applyToNode(node);
          }
        }
      }
    }).observe(document.body, { childList: true, subtree: true });
  }

  if (document.body) {
    setup();
  } else {
    document.addEventListener('DOMContentLoaded', setup);
  }
})();
`
}

const actions: Record<string, string> = {
  prev: `document.querySelector('button[data-e2e="arrow-left"]')?.click()`,
  next: `document.querySelector('button[data-e2e="arrow-right"]')?.click()`,
  toggleHidden: `window.__meguri_toggle?.()`,
}

async function evalJs(js: string): Promise<void> {
  try {
    await invoke('tiktok_eval', { js })
  } catch (e) {
    console.warn('TikTok bridge: eval failed', e)
  }
}

async function init(): Promise<void> {
  await evalJs(buildInitScript())
}

async function exec(action: string): Promise<void> {
  const js = actions[action]
  if (!js) {
    console.warn(`TikTok bridge: unknown action "${action}"`)
    return
  }
  await evalJs(js)
}

async function setHidden(hidden: boolean): Promise<void> {
  await evalJs(`window.__meguri_setHidden?.(${hidden})`)
}

export function useTiktokBridge() {
  return {
    init,
    exec,
    setHidden,
    prevVideo: () => exec('prev'),
    nextVideo: () => exec('next'),
    toggleHidden: () => exec('toggleHidden'),
  }
}
