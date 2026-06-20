import { defineConfig, devices } from '@playwright/test';

// Serves the pre-built Trunk output (`dist/`). Run `trunk build` at the repo
// root before the tests.
export default defineConfig({
  testDir: './tests',
  snapshotDir: './tests/snapshots',
  fullyParallel: false,
  retries: process.env.CI ? 1 : 0,
  reporter: process.env.CI ? 'github' : 'list',

  use: {
    baseURL: 'http://localhost:4181',
    trace: 'on-first-retry',
    viewport: { width: 1280, height: 800 },
  },

  projects: [{ name: 'chromium', use: { ...devices['Desktop Chrome'] } }],

  webServer: {
    command: 'npx serve ../dist -p 4181 --no-clipboard',
    url: 'http://localhost:4181',
    reuseExistingServer: !process.env.CI,
    timeout: 20_000,
  },
});
