```
const { callDeepseek } = require('../api/deepseek');

/**
 * Execute the run command.
 * @param {string} prompt
 * @param {function} apiCaller - optional, defaults to real API caller
 * @returns {Promise<{ success: boolean, output: string }>}
 */
async function runCommand(prompt, apiCaller = callDeepseek) {
  if (!prompt || prompt.trim() === '') {
    return { success: false, output: 'Error: prompt cannot be empty' };
  }
  const result = await apiCaller(prompt);
  if (!result.success) {
    return { success: false, output: `Error: ${result.error}` };
  }
  const output = `${result.content}\nDONE when done.`;
  return { success: true, output };
}

module.exports = { runCommand };
```