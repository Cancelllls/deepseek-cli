```javascript
#!/usr/bin/env node
const { Command } = require('commander');
const { runCommand } = require('./cli/run');

const program = new Command();

program
  .name('deepseek')
  .description('Deepseek CLI - AI at your terminal')
  .version('1.0.0');

program
  .command('ask <prompt>')
  .description('Ask a question to Deepseek')
  .action(async (prompt) =>    const { success, output } = runCommand(prompt);
    console.log(output);
    process.exit(success ? 0 : 1);
  });

// Default command if no args? Might want to show help
program.parse(process.argv);
```