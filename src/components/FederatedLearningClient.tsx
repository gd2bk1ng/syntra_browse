// ================================================================================================
// SYNTRA FEDERATED LEARNING CLIENT — Simulated federated learning module for privacy-first updates
// ================================================================================================

import React, { useEffect, useState, useRef } from 'react';
import { Box, Typography, Button, LinearProgress, Alert } from '@mui/material';
import { FederatedUpdate } from '../types';
import { sendFederatedUpdate } from '../api';
import CryptoJS from 'crypto-js';

// Utility: Simulate model update generation and encryption
function generateModelUpdate(): string {
  // Simulate model delta as random float array serialized as JSON string
  const modelDelta = Array.from({ length: 10 }, () => Math.random());
  const serialized = JSON.stringify(modelDelta);
  return serialized;
}

// Encrypt update with simple symmetric key (demo only)
function encryptUpdate(update: string, key: string): string {
  return CryptoJS.AES.encrypt(update, key).toString();
}

const CLIENT_ID = `client-${Math.floor(Math.random() * 10000)}`;

const FederatedLearningClient: React.FC = () => {
  const [status, setStatus] = useState('Idle');
  const [progress, setProgress] = useState(0);
  const [lastSent, setLastSent] = useState<string | null>(null);
  const intervalRef = useRef<NodeJS.Timer | null>(null);

  // Simulate periodic federated update sending
  useEffect(() => {
    intervalRef.current = setInterval(async () => {
      setStatus('Generating model update...');
      const update = generateModelUpdate();
      const encrypted = encryptUpdate(update, CLIENT_ID);

      setStatus('Sending update securely...');
      try {
        await sendFederatedUpdate({
          clientId: CLIENT_ID,
          modelUpdate: encrypted,
          timestamp: new Date().toISOString(),
        });
        setLastSent(new Date().toLocaleTimeString());
        setStatus('Update sent successfully.');
        setProgress(100);
      } catch {
        setStatus('Failed to send update.');
        setProgress(0);
      }
    }, 60000); // Every 60 seconds

    return () => {
      if (intervalRef.current) clearInterval(intervalRef.current);
    };
  }, []);

  return (
    <Box sx={{ p: 2, border: '1px solid #ddd', borderRadius: 2, mt: 4, bgcolor: '#f7f7f7' }}>
      <Typography variant="h6" gutterBottom>
        Federated Learning Client
      </Typography>
      <Typography variant="body2" gutterBottom>
        Client ID: <strong>{CLIENT_ID}</strong>
      </Typography>
      <Typography variant="body2" gutterBottom>Status: {status}</Typography>
      <LinearProgress variant="determinate" value={progress} sx={{ height: 10, borderRadius: 5, mb: 2 }} />
      {lastSent && <Alert severity="success">Last update sent at {lastSent}</Alert>}
      <Typography variant="caption" color="textSecondary" display="block" mt={1}>
        Updates are encrypted locally and sent securely to preserve privacy.
      </Typography>
      <Button
        variant="outlined"
        size="small"
        sx={{ mt: 2 }}
        onClick={() => {
          setStatus('Manual update triggered...');
          setProgress(0);
        }}
      >
        Trigger Manual Update
      </Button>
    </Box>
  );
};

export default FederatedLearningClient;
