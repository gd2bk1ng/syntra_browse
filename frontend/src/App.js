/* ================================================================================================
   SYNTRA KERNEL — FRONTEND UI
   ------------------------------------------------------------------------------------------------
        .\s/.
       :: S ::
        '/s\'

   File:        /frontend/src/App.js
   Module:      Syntra Kernel — React Chat Interface
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Minimal React-based frontend for interacting with the Syntra Kernel cognitive
                runtime. Displays Syntra’s replies, thought process, emotional state, personality
                traits, and recent narrative/log events emitted by the backend.

   Notes:
     - This UI is intentionally lightweight and serves as a diagnostic and interaction surface.
     - Future versions may integrate the Renderer, ThoughtStream, or World‑Model visualization.
     - This file is dual-licensed under MIT and Apache 2.0.
   ================================================================================================ */

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

    try {
      const response = await fetch('http://localhost:8080/chat', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ message: input }),
      });

      const data = await response.json();

      const syntraMessage = { sender: 'Syntra', text: data.reply };
      setMessages((msgs) => [...msgs, syntraMessage]);

      setEmotion(data.emotion_state || '');
      setPersonality(data.personality_traits || '');
      setEvents(data.recent_events || []);
    } catch (err) {
      setMessages((msgs) => [
        ...msgs,
        { sender: 'System', text: 'Error communicating with Syntra Kernel backend.' },
      ]);
    }

    setInput('');
  }

  return (
    <div style={{ maxWidth: 600, margin: 'auto', fontFamily: 'Arial, sans-serif' }}>
      <h1>Syntra Kernel — Chat Interface</h1>

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
