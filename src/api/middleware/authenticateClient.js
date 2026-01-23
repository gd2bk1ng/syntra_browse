/**
 * @file authenticateClient.js
 * @description Basic API key authentication middleware for federated clients
 */

const validApiKeys = new Set([
  process.env.CLIENT_API_KEY_1,
  process.env.CLIENT_API_KEY_2,
  // Add more keys as needed
]);

function authenticateClient(req, res, next) {
  const apiKey = req.headers['x-api-key'];
  if (!apiKey || !validApiKeys.has(apiKey)) {
    return res.status(403).json({ error: 'Forbidden: Invalid client API key' });
  }
  req.clientId = apiKey; // or map to client ID
  next();
}

module.exports = { authenticateClient };
