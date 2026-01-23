/**
 * @file server.js
 * @description Main Express server with federated update and feedback history APIs
 * @author YourName
 */

const express = require('express');
const bodyParser = require('body-parser');
const jwt = require('jsonwebtoken');
const { authenticateUser } = require('./middleware/authenticateUser');
const { authenticateClient } = require('./middleware/authenticateClient');
const federatedUpdateController = require('./controllers/federatedUpdate');
const feedbackController = require('./controllers/feedback');

const app = express();
app.use(bodyParser.json());

// Federated update endpoint (clients send encrypted model updates)
app.post('/federated_update', authenticateClient, federatedUpdateController.receiveUpdate);

// Feedback history API with pagination and filtering
app.get('/feedback_history', authenticateUser, feedbackController.getFeedbackHistory);

const PORT = process.env.PORT || 3000;
app.listen(PORT, () => {
  console.log(`Federated backend server listening on port ${PORT}`);
});
