// ================================================================================================
// SYNTRA APP — Main entry point combining all components with polished layout
// ================================================================================================

import React, { useState } from 'react';
import { Container, Typography, Divider } from '@mui/material';
import FeedbackForm from './components/FeedbackForm';
import FeedbackHistory from './components/FeedbackHistory';
import FederatedLearningClient from './components/FederatedLearningClient';

const App: React.FC = () => {
  const [refreshHistory, setRefreshHistory] = useState(false);

  return (
    <Container maxWidth="md" sx={{ py: 4 }}>
      <Typography variant="h3" component="h1" gutterBottom align="center" sx={{ fontWeight: 'bold' }}>
        Syntra AGI Feedback & Federated Learning
      </Typography>

      <FeedbackForm onFeedbackSubmitted={() => setRefreshHistory((prev) => !prev)} />

      <Divider sx={{ my: 4 }} />

      <FeedbackHistory key={refreshHistory ? 'refresh1' : 'refresh0'} />

      <Divider sx={{ my: 4 }} />

      <FederatedLearningClient />
    </Container>
  );
};

export default App;
