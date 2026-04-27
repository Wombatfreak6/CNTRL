import { Component, createEffect, createSignal } from 'solid-js';
import { browserState, browserActions } from '../stores/browserStore';
import './UrlBar.css';

export const UrlBar: Component = () => {
  const [inputUrl, setInputUrl] = createSignal('');

  // Sync with active tab
  createEffect(() => {
    const activeTab = browserState.tabs.find(t => t.id === browserState.activeTabId);
    if (activeTab) {
      setInputUrl(activeTab.url);
    } else {
      setInputUrl('');
    }
  });

  const handleKeyDown = (e: KeyboardEvent) => {
    if (e.key === 'Enter' && browserState.activeTabId) {
      let url = inputUrl();
      if (!url.startsWith('http://') && !url.startsWith('https://')) {
        url = `https://${url}`;
      }
      browserActions.navigate(browserState.activeTabId, url);
    }
  };

  const isHttps = () => inputUrl().startsWith('https://');

  return (
    <div class="url-bar-container">
      <div class="url-bar">
        <span class="icon">{isHttps() ? '🔒' : '⚠️'}</span>
        <input
          type="text"
          value={inputUrl()}
          onInput={(e) => setInputUrl(e.target.value)}
          onKeyDown={handleKeyDown}
          class="url-input"
          placeholder="Enter URL or search"
        />
      </div>
    </div>
  );
};
