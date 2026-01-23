// ================================================================================================
// SYNTRA FEEDBACK HISTORY — Visualization of past feedback with charts
// ================================================================================================

import React, { useEffect, useState } from 'react';
import { Typography, Box, CircularProgress, Paper } from '@mui/material';
import { UserFeedback } from '../types';
import { LineChart, Line, XAxis, YAxis, Tooltip, CartesianGrid, ResponsiveContainer } from 'recharts';
import axios from 'axios';

const FEEDBACK_HISTORY_API = 'http://localhost:3030/feedback_history'; // Adjust as needed

const FeedbackHistory: React.FC = () => {
  const [feedbacks, setFeedbacks] = useState<UserFeedback[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchFeedback() {
      try {
        const response = await axios.get<UserFeedback[]>(FEEDBACK_HISTORY_API);
        setFeedbacks(response.data);
      } catch {
        setFeedbacks([]);
      } finally {
        setLoading(false);
      }
    }
    fetchFeedback();
  }, []);

  // Aggregate average rating per day
  const data = feedbacks.reduce<Record<string, { date: string; total: number; count: number }>>((acc, fb) => {
    const day = fb.timestamp.slice(0, 10);
    if (!acc[day]) acc[day] = { date: day, total: 0, count: 0 };
    acc[day].total += fb.userRating;
    acc[day].count += 1;
    return acc;
  }, {});

  const chartData = Object.values(data).map(({ date, total, count }) => ({
    date,
    avgRating: count > 0 ? total / count : 0,
  }));

  return (
    <Paper sx={{ p: 3, mt: 4 }}>
      <Typography variant="h6" gutterBottom>
        Feedback History (Average Rating by Day)
      </Typography>

      {loading ? (
        <Box sx={{ display: 'flex', justifyContent: 'center', p: 4 }}>
          <CircularProgress />
        </Box>
      ) : (
        <ResponsiveContainer width="100%" height={300}>
          <LineChart data={chartData}>
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis dataKey="date" />
            <YAxis domain={[0, 5]} />
            <Tooltip />
            <Line type="monotone" dataKey="avgRating" stroke="#1976d2" strokeWidth={2} />
          </LineChart>
        </ResponsiveContainer>
      )}
    </Paper>
  );
};

export default FeedbackHistory;
