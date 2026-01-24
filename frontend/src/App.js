// =============================================================================
//  Syntra AGI - React Chat UI
//  File: App.js
//
//  Description:
//  Minimal React frontend for chatting with Syntra AGI backend.
//  Displays Astra’s replies, thaught process, emotional state, personality traits,
//  all recent log events and recent ecosystem changes and events.
//
//  Author:      Alex Roussinov
//  Created:     2025-12-25
//  Updated:     2025-12-25
//
//  This file is dual licensed under the MIT and Apache 2.0 licenses.
// =============================================================================

import React, { useState } from 'react';

function App() {
  const [input, setInput] = useState('');
  const [messages, setMessages] = useState([]);
  const [emotion, setEmotion] = useState('');
  const [personality, setPersonality] = useState('');
  const [events, setEvents] = useState([]);

  async function sendMessage() {
    if (!input.trim()) return;

    const userMessage = { sender: 'You', text: input };
    setMessages((msgs) => [...msgs, userMessage]);

    const response = await fetch('http://localhost:8080/chat', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ message: input }),
    });

    const data = await response.json();

    const astraMessage = { sender: 'Astra', text: data.reply };
    setMessages((msgs) => [...msgs, astraMessage]);
    setEmotion(data.emotion_state);
    setPersonality(data.personality_traits);
    setEvents(data.recent_events);

    setInput('');
  }

  return (
    <div style={{ maxWidth: 600, margin: 'auto', fontFamily: 'Arial, sans-serif' }}>
      <h1>Astra AGI Chat</h1>
      <div style={{ border: '1px solid #ccc', padding: 10, height: 300, overflowY: 'scroll' }}>
        {messages.map((m, i) => (
          <div key={i} style={{ margin: '8px 0' }}>
            <strong>{m.sender}:</strong> {m.text}
          </div>
        ))}
      </div>
      <input
        style={{ width: '80%', padding: 8, marginTop: 10 }}
        value={input}
        onChange={(e) => setInput(e.target.value)}
        onKeyDown={(e) => { if (e.key === 'Enter') sendMessage(); }}
        placeholder="Type your message..."
      />
      <button onClick={sendMessage} style={{ padding: '8px 16px', marginLeft: 10 }}>
        Send
      </button>

      <div style={{ marginTop: 20 }}>
        <h3>Current Emotion State</h3>
        <pre>{emotion}</pre>

        <h3>Personality Traits</h3>
        <pre>{personality}</pre>

        <h3>Recent Narrative Events</h3>
        <ul>
          {events.map((e, i) => (
            <li key={i} style={{ fontSize: '0.9em' }}>{e}</li>
          ))}
        </ul>
      </div>
    </div>
  );
}

export default App;
