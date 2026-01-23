/**
 * @file feedback.js
 * @description Controller for feedback history with pagination and filtering
 */

const Feedback = require('../models/feedbackModel');

async function getFeedbackHistory(req, res) {
  try {
    const userId = req.user.id;
    const page = parseInt(req.query.page) || 1;
    const limit = parseInt(req.query.limit) || 20;
    const filter = req.query.filter || '';

    // Example filter parsing: filter=date:2026-01-01
    let query = { userId };
    if (filter.startsWith('date:')) {
      const date = filter.split(':')[1];
      query.timestamp = { $gte: new Date(date) };
    }

    const total = await Feedback.countDocuments(query);
    const feedbacks = await Feedback.find(query)
      .sort({ timestamp: -1 })
      .skip((page - 1) * limit)
      .limit(limit)
      .lean();

    res.json({
      data: feedbacks,
      meta: { total, page, limit },
    });
  } catch (err) {
    console.error('Error in getFeedbackHistory:', err);
    res.status(500).json({ error: 'Internal server error' });
  }
}

module.exports = { getFeedbackHistory };
