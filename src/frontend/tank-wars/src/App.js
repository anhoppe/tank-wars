import logo from './logo.svg';
import './App.css';
import { useState } from 'react';
import {useNavigate} from 'react-router-dom';
import {getOrCreatePlayer} from './api';


function App() {
  const navigate = useNavigate();
  const [name, setName] = useState('');

  async function handleStart() {
    const player = await getOrCreatePlayer(name);
    navigate('/main', { state: { player } });
  }

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.js</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
        <div>
          <textarea value={name} onChange={e => setName(e.target.value)}></textarea>
          <button onClick={handleStart}>Enter Player Name and Start Game</button>
        </div>
      </header>
    </div>
  );
}

export default App;
