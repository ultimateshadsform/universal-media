import { spawnSync } from 'bun';
import { mkdir, rename, existsSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

// Get current directory
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// Configuration
const PLATFORM = 'win32-x64-msvc';
const FILENAME = `universal-media.${PLATFORM}.node`;
const TARGET_DIR = join(__dirname, '..', 'npm', PLATFORM);

// Ensure target directory exists
if (!existsSync(TARGET_DIR)) {
  mkdir(TARGET_DIR, { recursive: true }, (err) => {
    if (err) throw err;
  });
}

// Run the build
console.log('Building native module...');
const buildResult = spawnSync(['napi', 'build', '--platform', '--release'], {
  stdio: ['ignore', 'inherit', 'inherit'] as const,
});

if (buildResult.success) {
  // Move the file to the correct location
  const sourceFile = join(__dirname, '..', FILENAME);
  const targetFile = join(TARGET_DIR, FILENAME);

  if (existsSync(sourceFile)) {
    rename(sourceFile, targetFile, (err) => {
      if (err) throw err;
      console.log(`Successfully moved ${FILENAME} to ${TARGET_DIR}`);
    });
  } else {
    console.error(`Build failed: ${FILENAME} not found`);
    process.exit(1);
  }
} else {
  console.error('Build failed');
  process.exit(1);
}
