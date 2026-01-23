// ================================================================================================
// SYNTRA API — HTTP client for feedback and federated learning endpoints
// ================================================================================================

import axios from 'axios';
import { UserFeedback, FederatedUpdate } from './types';

const API_BASE = 'http://localhost:3030'; // Change as needed

export async function submitUserFeedback(feedback: Omit<UserFeedback, 'feedbackId' | 'timestamp'>): Promise<UserFeedback> {
  const response = await axios.post(`${API_BASE}/submit`, feedback);
  return {
    feedbackId: response.data.feedback_id,
    timestamp: new Date().toISOString(),
    ...feedback,
  };
}

export async function sendFederatedUpdate(update: FederatedUpdate): Promise<void> {
  await axios.post(`${API_BASE}/federated_update`, update);
}
