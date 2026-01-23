/**
 * @file userModel.js
 * @description Mongoose schema for user management
 */

const mongoose = require('mongoose');
const bcrypt = require('bcrypt');

const userSchema = new mongoose.Schema({
  email: { type: String, unique: true, required: true },
  passwordHash: { type: String, required: true },
  roles: { type: [String], default: ['user'] },
  createdAt: { type: Date, default: Date.now },
});

// Password hashing middleware
userSchema.pre('save', async function(next) {
  if (!this.isModified('passwordHash')) return next();
  this.passwordHash = await bcrypt.hash(this.passwordHash, 12);
  next();
});

module.exports = mongoose.model('User', userSchema);
