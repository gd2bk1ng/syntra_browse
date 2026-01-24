// =============================================================================
//  Syntra AGI - Visualization Frontend
//  File: /frontend/src/Dashboard.tsx
//
//  Description:
//      React component displaying Syntra's learning progress and reasoning chains.
//
//  Author:      Alex Roussinov
//  Created:     2025-12-26
// =============================================================================

import React, { useEffect, useState } from 'react';
import axios from 'axios';
import {
  BarChart, Bar, XAxis, YAxis, Tooltip, ResponsiveContainer,
} from 'recharts';

interface LearningProgress {
  concepts_learned: number;
  research_sessions: number;
  code_modules_created: number;
  last_updated: string;
}

interface ReasoningChains {
  [key: string]: string[];
}

const Dashboard: React.FC = () => {
  const [progress, setProgress] = useState<LearningProgress | null>(null);
  const [reasoningChains, setReasoningChains] = useState<ReasoningChains>({});

  useEffect(() => {
    async function fetchData() {
      try {
        const progressRes = await axios.get<LearningProgress>('/api/visualization/learning_progress');
        setProgress(progressRes.data);

        const chainsRes = await axios.get<ReasoningChains>('/api/visualization/reasoning_chains');
        setReasoningChains(chainsRes.data);
      } catch (error) {
        console.error('Error fetching visualization data:', error);
      }
    }
    fetchData();
  }, []);

  if (!progress) {
    return <div>Loading Syntra's learning progress...</div>;
  }

  const barData = [
    { name: 'Concepts Learned', value: progress.concepts_learned },
    { name: 'Research Sessions', value: progress.research_sessions },
    { name: 'Code Modules Created', value: progress.code_modules_created },
  ];

  return (
    <div style={{ padding: 20, fontFamily: 'Arial, sans-serif' }}>
      <h1>Astra AGI Learning Dashboard</h1>
      <p>Last Updated: {new Date(progress.last_updated).toLocaleString()}</p>

      <h2>Learning Progress</h2>
      <ResponsiveContainer width="100%" height={300}>
        <BarChart data={barData} margin={{ top: 20, right: 30, left: 20, bottom: 5 }}>
          <XAxis dataKey="name" />
          <YAxis allowDecimals={false} />
          <Tooltip />
          <Bar dataKey="value" fill="#4caf50" />
        </BarChart>
      </ResponsiveContainer>

      <h2>Reasoning Chains</h2>
      {Object.keys(reasoningChains).length === 0 && <p>No reasoning chains available.</p>}
      <ul>
        {Object.entries(reasoningChains).map(([key, steps]) => (
          <li key={key}>
            <strong>{key}:</strong>
            <ol>
              {steps.map((step, idx) => (
                <li key={idx}>{step}</li>
              ))}
            </ol>
          </li>
        ))}
      </ul>
    </div>
  );
};

export default Dashboard;
