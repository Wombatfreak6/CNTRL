import { onMount } from 'solid-js';
import { browserActions } from './stores/browserStore';
import { TabBar } from './components/TabBar';
import { UrlBar } from './components/UrlBar';
import { WebView } from './components/WebView';
import './App.css';

function App() {
  onMount(() => {
    browserActions.fetchTabs();
  });

  return (
    <div class="app-container">
      <TabBar />
      <UrlBar />
      <WebView />
    </div>
  );
}

export default App;
