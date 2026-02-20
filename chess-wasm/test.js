const { execSync } = require('child_process');
const fs = require('fs');
try {
    execSync('cargo check --color never', { encoding: 'utf8', stdio: 'pipe' });
    console.log("Success");
} catch (e) {
    fs.writeFileSync('build_errors.txt', (e.stdout || '') + '\n' + (e.stderr || ''));
    console.log("Found errors");
}
