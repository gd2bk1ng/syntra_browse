/**
 * @file feedbackModel.js
 * @description Mongoose schema for user feedback
 */

const mongoose = require('mongoose');

const feedbackSchema = new mongoose.Schema({
  userId: { type: String, required: true },
  feedbackText: { type: String, required: true },
  timestamp: { type: Date, default: Date.now },
  type: { type: String, default: 'general' }, // optional categorization
});

module.exports = mongoose.model('Feedback', feedbackSchema);
