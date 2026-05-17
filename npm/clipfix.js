#!/usr/bin/env node
'use strict';
const { execFileSync } = require('child_process');
const path = require('path');
const binary = path.join(__dirname, 'bin', 'clipfix');
const args = process.argv.slice(2);
try {
  execFileSync(binary, args, { stdio: 'inherit' });
} catch (e) {
  process.exit(e.status || 1);
}
