```javascript
const { expect } = require('chai');
const { execSync } = require('child_process');

describe('CLI integration', () => {
  let originalEnv;

  before(() => {
    // Ensure no API key is set so the fallback is used
    originalEnv = { ...process.env };
    delete process.env.DEEPSEEK_API_KEY;
  });

  after(() => {
    process.env = originalEnv;
  });

 ('should print success message with "DONE when done." and exit 0', () => {
    const stdout = execSync('node index.js ask "hello world"', { cwd: __dirname + '/..', encoding 'utf8' });
    expect(stdout).to.include('Deepseek replied to: "hello world"');
    expect(stdout).to.include('DONE when done.');
  });

  it('should exit with 0 on success', () => {
    // If command succeeds, execSync would not throw.
    // We can use try-catch, but expect it not to throw.
    expect(() => {
      execSync('node index.js ask "test"', { cwd: __dirname + '/..', stdio: 'pipe' });
    }).to.not.throw();
  });

  it('should exit with non-zero on error (empty prompt)', () => {
    // Empty prompt should cause failure, but the CLI exits with code 1, which may throw an error.
    // We'll catch the error and check the exit code.
    try {
      execSync('node index.js ask " "', { cwd: __dirname + '/..', stdio: 'pipe' });
      // If we get here, it means no error thrown, but empty prompt should error? Our command trims and fails, so exit 1. So execSync will throw with status 1.
      // So this should not succeed.
      expect.fail('Should have thrown');
    } catch (err) {
      expect(err.status).to.equal(1);
      expect(err.stdout.toString()).to.include('Error: prompt cannot be empty');
    }
  });
});
```