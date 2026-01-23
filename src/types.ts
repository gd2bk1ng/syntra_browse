// ================================================================================================
// SYNTRA TYPES — Shared TypeScript interfaces for feedback and federated learning
// ================================================================================================

export interface UserFeedback {
  feedbackId: string;
  intentLabel: string;
  userRating: number; // 1 to 5
  comments?: string;
  timestamp: string; // ISO string
}

export interface FederatedUpdate {
  clientId: string;
  modelUpdate: string; // Encrypted or serialized model delta
  timestamp: string;
}
