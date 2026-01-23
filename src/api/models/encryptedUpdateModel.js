/**
 * @file encryptedUpdateModel.js
 * @description Mongoose schema for storing encrypted model updates
 */

const mongoose = require('mongoose');

const encryptedUpdateSchema = new mongoose.Schema({
  clientId: { type: String, required: true },
  encryptedUpdate: { type: String, required: true }, // store as base64 or hex string
  timestamp: { type: Date, default: Date.now },
});

module.exports = mongoose.model('EncryptedUpdate', encryptedUpdateSchema);
