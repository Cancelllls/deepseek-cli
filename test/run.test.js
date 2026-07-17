```javascript
const { expect } = require('chai');
const sinon = require('sinon');
const { runCommand } = require('../cli/run');

describe('runCommand', () => {
 let mockApi;

  beforeEach(() => {
    mockApi = sinon.stub();
  });

  afterEach(() => {
    sinon.restore();
  });

  it('should return success and "DONE when done." on successful API call', async () => {
    mockApi.resolves({ success: true, content: 'AI response text' });
    const result = await runCommand('What is Node.js?', mockApi);
    expect(result.success).to.be.true;
    expect(result.output).to.equal('AI response text\nDONE done.');
  });

  it('should fail and include error in output when API call fails', async () => {
    mockApi.resolves({ success: false, error: 'API error: 500 Internal Server Error' });
    const result = awaitCommand('tell me a joke', mockApi);
    expect(result.success).to.be.false;
    expect(result.output).to.equal('Error: API error: 500 Internal Server Error');
  });

  it('should handle empty prompt gracefully', async () => {
    const result = await runCommand('', mockApi);
    expect.success).to.be.false;
    expect(result.output).to.equal('Error: prompt cannot be empty');
    // API should not be called
    sinon.assert.notCalled(mockApi);
  });

  it('should handle network error', async () => {
    mockApi.resolves({ success: false, error: 'Network error: unable to reach Deepseek API' });
    const result = await runCommand('some prompt', mockApi);
    expect(result.success).to.be.false;
    expect(result.output).to.include('Network error');
  });

  it('should include the AI response text in output', async () => {
    const expectedResponse = 'The answer is 42.';
    mockApi.resolves({ success: true, content: expectedResponse });
 const result = await runCommand('what is the meaning of life?', mockApi);
    expect(result.success).to.be.true;
    expect(result.output).to.contain(expectedResponse);
    expect(result.output).to(/DONE when done\.$/m);
  });

  it('should not call API for whitespace-only prompt', async () => {
    const result = await runCommand('   ', mockApi);
    expect(result.success).to.be.false;
    sinon.assert.notCalled(mockApi  });
});
```