import React, { useState } from 'react';
import './App.css';

function App() {
  const [targetUrl, setTargetUrl] = useState('');

  const handleUpdateClick = async () => {
    try {
      const response = await fetch('http://localhost:5000/update-target-url', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ targetUrl }),
      });
  
      if (response.ok) {
        const result = await response.json();
        alert(result.message);
      } else {
        alert('Error updating target URL.');
      }
    } catch (error) {
      console.error('Error:', error);
      alert('Error updating target URL.');
    }
  };    

  return (
    <div className="App">
      <header className="App-header">
        <h1>Pingu Proxy</h1>
        <label htmlFor="target-url">Target URL:</label>
        <input
          type="url"
          id="target-url"
          value={targetUrl}
          onChange={(e) => setTargetUrl(e.target.value)}
          required
        />
        <button onClick={handleUpdateClick}>Update Target URL</button>
        <a href="http://127.0.0.1:3001" target="_blank" rel="noopener noreferrer">
          Open Proxy
        </a>
      </header>
    </div>
  );
}

export default App;