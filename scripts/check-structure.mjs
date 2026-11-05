import { existsSync } from 'node:fs';

const required = [
  'apps',
  'packages',
  'contracts',
  'services',
  'docs',
  'infra',
  '.github/workflows',
  'README.md',
  'LICENSE',
  'SECURITY.md',
  'CONTRIBUTING.md',
];

const missing = required.filter((path) => !existsSync(path));

if (missing.length > 0) {
  console.error('Repository foundation check failed. Missing:', missing.join(', '));
  process.exit(1);
}

console.log('HerAgent repository foundation check passed.');
