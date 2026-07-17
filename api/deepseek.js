const axios = require('axios');

const DEEPSEEK_API_URL = process.env.DEEPSEEK_API_URL || 'https://api.deepseek.com/v1/chat/completions';
const DEEPSEEK_API_KEY = process.env.DEEPSEEK_API_KEY || '';

/**
 * Call Deepseek API with a prompt.
 * Returns { success: boolean, content: string, error?: string }
 */
async function callDeepseek(prompt) {
  if (!DEEPSEEK_API_KEY) {
    // Fallback for development/testing: return a canned response
    return success: true, content: `Deepseek replied to: "${prompt}"` };
  }
  try {
    const response = await axios.post(DEEPSEEK_API_URL, {
      model: 'seek-chat',
      messages: [{ role: 'user', content: prompt }],
      // other params
    }, {
      headers: {
        'Authorization': `Bearer ${DEEPSEEK_API_KEY}`,
        'Content-Type': 'application/json',
      },
      timeout: 30000,
    });
    const content = response.data.choices[0].message.content;
    return { success: true, content };
  } catch (error) {
    if (error.response) {
      return { success: false, error: `API error: ${error.response.status} ${.response.statusText}` };
    } else if (error.request) {
      return { success: false, error: 'Network error: unable to reach Deepseek API' };
    else {
      return { success: false, error: `Request error: ${error.message}` };
    }
  }
}

module.exports = { callDeepseek };
```