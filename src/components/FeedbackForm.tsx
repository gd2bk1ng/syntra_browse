// ================================================================================================
// SYNTRA FEEDBACK FORM — User feedback submission UI component
// ================================================================================================

import React, { useState } from 'react';
import { TextField, Button, Rating, Box, Typography, CircularProgress, Alert } from '@mui/material';
import { submitUserFeedback } from '../api';

interface FeedbackFormProps {
  onFeedbackSubmitted: () => void;
}

const FeedbackForm: React.FC<FeedbackFormProps> = ({ onFeedbackSubmitted }) => {
  const [intentLabel, setIntentLabel] = useState('');
  const [userRating, setUserRating] = useState<number | null>(null);
  const [comments, setComments] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState(false);

  const isValid = intentLabel.trim() !== '' && userRating !== null && userRating >= 1 && userRating <= 5;

  const handleSubmit = async () => {
    if (!isValid) {
      setError('Please fill all required fields with valid values.');
      return;
    }
    setLoading(true);
    setError(null);
    setSuccess(false);

    try {
      await submitUserFeedback({ intentLabel, userRating, comments: comments.trim() || undefined });
      setSuccess(true);
      setIntentLabel('');
      setUserRating(null);
      setComments('');
      onFeedbackSubmitted();
    } catch (e) {
      setError('Failed to submit feedback. Please try again later.');
    } finally {
      setLoading(false);
    }
  };

  return (
    <Box sx={{ maxWidth: 500, mx: 'auto', p: 2, border: '1px solid #ccc', borderRadius: 2, backgroundColor: '#fafafa' }}>
      <Typography variant="h6" gutterBottom>
        Submit Your Feedback
      </Typography>

      <TextField
        label="Intent Label"
        fullWidth
        value={intentLabel}
        onChange={(e) => setIntentLabel(e.target.value)}
        margin="normal"
        required
      />

      <Box sx={{ display: 'flex', alignItems: 'center', mt: 2 }}>
        <Typography component="legend">Rating</Typography>
        <Rating
          name="user-rating"
          value={userRating}
          onChange={(_, newValue) => setUserRating(newValue)}
          precision={1}
          max={5}
          sx={{ ml: 2 }}
        />
      </Box>

      <TextField
        label="Comments (optional)"
        fullWidth
        multiline
        minRows={3}
        value={comments}
        onChange={(e) => setComments(e.target.value)}
        margin="normal"
      />

      {error && <Alert severity="error" sx={{ mt: 2 }}>{error}</Alert>}
      {success && <Alert severity="success" sx={{ mt: 2 }}>Thank you for your feedback!</Alert>}

      <Box sx={{ mt: 3, textAlign: 'right' }}>
        <Button variant="contained" onClick={handleSubmit} disabled={loading}>
          {loading ? <CircularProgress size={24} /> : 'Submit'}
        </Button>
      </Box>
    </Box>
  );
};

export default FeedbackForm;
