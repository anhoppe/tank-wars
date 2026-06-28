import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import reportWebVitals from './reportWebVitals';

import App from './App';
import Fleet from './Fleet';
import Game from './Game/Game';
import GameEditor from './GameEditor';
import Main from './main';
import OpponentSelection from './Game/OpponentSelection';
import ResearchBlueprints from './ResearchBlueprints';

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
  <React.StrictMode>
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<App />} />
        <Route path="/fleet" element={<Fleet />} />
        <Route path="/game" element={<Game />} />
        <Route path="/game-editor" element={<GameEditor />} />
        <Route path="/main" element={<Main />} />
        <Route path="/opponent-selection" element={<OpponentSelection />} />
        <Route path="/research-blueprints" element={<ResearchBlueprints />} />
      </Routes>
    </BrowserRouter>
  </React.StrictMode>
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
