/**
 * @file federatedUpdate.js
 * @description Controller for federated update collection and triggering retraining
 */

const EncryptedUpdate = require('../models/encryptedUpdateModel');
const { aggregateAndRetrain } = require('../../ml/retrain');

async function receiveUpdate(req, res) {
  try {
    const { encryptedModelUpdate } = req.body;
    if (!encryptedModelUpdate) {
      return res.status(400).json({ error: 'Missing encryptedModelUpdate in request body' });
    }

    // Store encrypted update securely with metadata
    await EncryptedUpdate.create({
      clientId: req.clientId,
      encryptedUpdate: encryptedModelUpdate,
      timestamp: new Date(),
    });

    // Aggregate and trigger retraining asynchronously
    aggregateAndRetrain();

    res.status(200).json({ message: 'Update received and retraining triggered' });
  } catch (err) {
    console.error('Error in receiveUpdate:', err);
    res.status(500).json({ error: 'Internal server error' });
  }
}

module.exports = { receiveUpdate };
