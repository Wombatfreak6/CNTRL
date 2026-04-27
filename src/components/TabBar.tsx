import { Component, For } from 'solid-js';
import { browserState, browserActions } from '../stores/browserStore';
import './TabBar.css';

export const TabBar: Component = () => {
  const handleNewTab = () => {
    browserActions.openTab('about:blank');
  };

  return (
    <div class="tab-bar">
      <For each={browserState.tabs.filter(t => !t.is_background)}>
        {(tab) => (
          <div
            class={`tab ${browserState.activeTabId === tab.id ? 'active' : ''}`}
            onClick={() => browserActions.setActiveTab(tab.id)}
          >
            <span class="title">{tab.title}</span>
            <button
              class="close-btn"
              onClick={(e) => {
                e.stopPropagation();
                browserActions.closeTab(tab.id);
              }}
            >
              ×
            </button>
          </div>
        )}
      </For>
      <button class="new-tab-btn" onClick={handleNewTab}>+</button>
    </div>
  );
};
