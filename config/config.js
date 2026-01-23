/**
 * @file config.js
 * @description Configuration variables for the backend
 */

module.exports = {
  JWT_SECRET: process.env.JWT_SECRET || 'your_jwt_secret_here',
  MONGO_URI: process.env.MONGO_URI || 'mongodb://localhost:27017/federated_db',
};
