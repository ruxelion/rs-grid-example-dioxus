import { test, expect, Page } from '@playwright/test';

// Canvas layout constants (match example_common::build_model): the grid is
// rendered on a <canvas>, so interaction tests use viewport coordinates.
const GUTTER = 55; // row-number gutter width
const HEADER = 60; // column-header height
const LS_KEY = 'rs-grid-basic-layout';

/** Wait long enough for the rAF loop to paint at least one frame. */
async function waitForPaint(page: Page, ms = 350) {
  await page.waitForTimeout(ms);
}

// ── smoke ────────────────────────────────────────────────────────────────────

test.describe('smoke', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await waitForPaint(page);
  });

  test('title is visible', async ({ page }) => {
    await expect(page.getByText('rs-grid basic example')).toBeVisible();
  });

  test('canvas is visible and sized', async ({ page }) => {
    const canvas = page.locator('canvas');
    await expect(canvas).toBeVisible();
    const box = await canvas.boundingBox();
    expect(box!.width).toBeGreaterThan(200);
    expect(box!.height).toBeGreaterThan(200);
  });

  test('defaults to 1 000 rows / 20 columns', async ({ page }) => {
    await expect(
      page.locator('strong', { hasText: '1 000 rows' }),
    ).toBeVisible();
    await expect(
      page.locator('strong', { hasText: '20 columns' }),
    ).toBeVisible();
  });

  test('loads without uncaught JS errors', async ({ page }) => {
    const errors: string[] = [];
    page.on('pageerror', e => errors.push(e.message));
    await page.reload();
    await waitForPaint(page);
    expect(errors).toEqual([]);
  });
});

// ── controls: dataset / column selectors ─────────────────────────────────────

test.describe('controls', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await waitForPaint(page);
  });

  test('switching to 100 000 rows updates the label', async ({ page }) => {
    await page.locator('select').first().selectOption('100000');
    await waitForPaint(page);
    await expect(
      page.locator('strong', { hasText: '100 000 rows' }),
    ).toBeVisible();
    await expect(page.locator('canvas')).toBeVisible();
  });

  test('switching to 100 columns updates the label', async ({ page }) => {
    await page.locator('select').nth(1).selectOption('100');
    await waitForPaint(page);
    await expect(
      page.locator('strong', { hasText: '100 columns' }),
    ).toBeVisible();
    await expect(page.locator('canvas')).toBeVisible();
  });

  test('combined rows + columns change', async ({ page }) => {
    await page.locator('select').first().selectOption('1000000');
    await page.locator('select').nth(1).selectOption('1000');
    await waitForPaint(page);
    await expect(
      page.locator('strong', { hasText: '1 million rows' }),
    ).toBeVisible();
    await expect(
      page.locator('strong', { hasText: '1 000 columns' }),
    ).toBeVisible();
    await expect(page.locator('canvas')).toBeVisible();
  });
});

// ── theme selector ───────────────────────────────────────────────────────────

test.describe('theme', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await waitForPaint(page);
  });

  test('Dark applies the root class', async ({ page }) => {
    await page.locator('select').nth(2).selectOption('dark');
    await waitForPaint(page);
    await expect
      .poll(() => page.evaluate(() => document.documentElement.className))
      .toBe('dark');
    await expect(page.locator('canvas')).toBeVisible();
  });

  test('Dimmed then back to Light', async ({ page }) => {
    const theme = page.locator('select').nth(2);
    await theme.selectOption('dimmed');
    await waitForPaint(page);
    await expect
      .poll(() => page.evaluate(() => document.documentElement.className))
      .toBe('dimmed');
    await theme.selectOption('');
    await waitForPaint(page);
    await expect
      .poll(() => page.evaluate(() => document.documentElement.className))
      .toBe('');
    await expect(page.locator('canvas')).toBeVisible();
  });
});

// ── language selector ────────────────────────────────────────────────────────

test.describe('language', () => {
  test('switching to Japanese does not crash', async ({ page }) => {
    await page.goto('/');
    await waitForPaint(page);
    const errors: string[] = [];
    page.on('pageerror', e => errors.push(e.message));
    await page.locator('select').nth(3).selectOption('ja');
    await waitForPaint(page);
    await expect(page.locator('canvas')).toBeVisible();
    expect(errors).toEqual([]);
  });
});

// ── editable / selectable / column-reorder toggles ───────────────────────────

test.describe('toggles', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await waitForPaint(page);
  });

  test('editable on: double-click the Name cell opens a text input', async ({
    page,
  }) => {
    const canvas = page.locator('canvas');
    await canvas.dblclick({ position: { x: GUTTER + 100, y: HEADER + 20 } });
    await waitForPaint(page, 200);
    await expect(page.locator('input[type="text"]')).toBeVisible();
  });

  test('editable off: double-click does not open an input', async ({ page }) => {
    // The Editable toggle is the first switch; the <input> is visually hidden
    // behind the track, so click the label (native label toggles the input).
    await page.locator('.app-switch').first().click();
    await waitForPaint(page, 150);
    const canvas = page.locator('canvas');
    await canvas.dblclick({ position: { x: GUTTER + 100, y: HEADER + 20 } });
    await waitForPaint(page, 200);
    await expect(page.locator('input[type="text"]')).toHaveCount(0);
  });

  test('selectable and column-reorder toggles do not crash', async ({
    page,
  }) => {
    const switches = page.locator('.app-switch');
    await switches.nth(1).click(); // selectable
    await switches.nth(2).click(); // column reorder
    await waitForPaint(page, 150);
    await expect(page.locator('canvas')).toBeVisible();
  });
});

// ── layout persistence + reset ───────────────────────────────────────────────

test.describe('layout persistence', () => {
  test('a seeded layout is applied on load without crashing', async ({
    page,
  }) => {
    await page.addInitScript(
      ([key]) => {
        window.localStorage.setItem(
          key,
          JSON.stringify([[['name', 300.0]], ['email', 'name'], 1]),
        );
      },
      [LS_KEY],
    );
    const errors: string[] = [];
    page.on('pageerror', e => errors.push(e.message));
    await page.goto('/');
    await waitForPaint(page);
    await expect(page.locator('canvas')).toBeVisible();
    expect(errors).toEqual([]);
  });

  test('Reset clears the persisted layout', async ({ page }) => {
    await page.goto('/');
    await waitForPaint(page);
    await page.evaluate(key => window.localStorage.setItem(key, '[[],[],0]'), LS_KEY);
    // The Reset button clears storage then reloads the page.
    await page.locator('.app-control-button').click();
    await waitForPaint(page);
    const stored = await page.evaluate(
      key => window.localStorage.getItem(key),
      LS_KEY,
    );
    expect(stored).toBeNull();
  });
});

// ── canvas interaction ───────────────────────────────────────────────────────

test.describe('canvas interaction', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await waitForPaint(page);
  });

  test('clicking a data cell does not crash', async ({ page }) => {
    const canvas = page.locator('canvas');
    await canvas.click({ position: { x: 80, y: 80 } });
    await waitForPaint(page, 100);
    await expect(canvas).toBeVisible();
  });

  test('wheel scroll', async ({ page }) => {
    const canvas = page.locator('canvas');
    await canvas.hover();
    await page.mouse.wheel(0, 300);
    await waitForPaint(page, 100);
    await expect(canvas).toBeVisible();
  });

  test('shift+click extends the selection', async ({ page }) => {
    const canvas = page.locator('canvas');
    await canvas.click({ position: { x: 80, y: 80 } });
    await canvas.click({
      position: { x: 200, y: 120 },
      modifiers: ['Shift'],
    });
    await waitForPaint(page, 100);
    await expect(canvas).toBeVisible();
  });
});

// ── visual regression ────────────────────────────────────────────────────────
//
// Generate baselines with `npm run update-snapshots`. Tolerance: 2 % of pixels.

test.describe('visual regression', () => {
  test('initial render', async ({ page }) => {
    await page.goto('/');
    await waitForPaint(page);
    await expect(page).toHaveScreenshot('initial.png', {
      maxDiffPixelRatio: 0.02,
    });
  });

  test('dark theme', async ({ page }) => {
    await page.goto('/');
    await waitForPaint(page);
    await page.locator('select').nth(2).selectOption('dark');
    await waitForPaint(page);
    await expect(page).toHaveScreenshot('dark.png', {
      maxDiffPixelRatio: 0.02,
    });
  });

  test('after vertical scroll', async ({ page }) => {
    await page.goto('/');
    await waitForPaint(page);
    const canvas = page.locator('canvas');
    await canvas.hover();
    await page.mouse.wheel(0, 500);
    await waitForPaint(page);
    await expect(canvas).toHaveScreenshot('scrolled-down.png', {
      maxDiffPixelRatio: 0.02,
    });
  });
});
