import { expect, test, type Page } from '@playwright/test';

const pages = [
  { path: '/', title: /Rust 阶梯学习站|Rust Ladder/ },
  { path: '/learn', title: /课程路径|Learning Path/ },
  { path: '/cards', title: /知识卡片|Knowledge Cards/ },
  { path: '/stats', title: /学习统计|Stats/ },
  { path: '/exercise/syntax-let-mut', title: /让变量真的可变|Make a variable mutable/ },
];

const exerciseCases = [
  { id: 'syntax-let-mut', kind: 'fill blank input' },
  { id: 'syntax-output', kind: 'code output textarea' },
  { id: 'array-type', kind: 'single choice buttons' },
  { id: 'borrowing-mut-ref', kind: 'order step buttons' },
];

test.describe('Rust Ladder pages', () => {
  for (const item of pages) {
    test(`${item.path} renders without visual regressions`, async ({ page }) => {
      await page.goto(item.path);
      await expect(page.getByRole('link', { name: /Rust 阶梯学习站|Rust Ladder/ })).toBeVisible();
      await expect(page.getByText(item.title).first()).toBeVisible();
      await assertNoHorizontalOverflow(page);
      await assertNoVisibleOverlap(page);
    });
  }

  test('defaults to readable light theme', async ({ page }) => {
    await page.goto('/exercise/syntax-let-mut');

    await expect(page.locator('html')).toHaveAttribute('data-theme', 'light');
    await expect(page.locator('.code-block').first()).toBeVisible();

    const colors = await page.locator('.code-block').first().evaluate((element) => {
      const style = getComputedStyle(element);
      return {
        foreground: style.color,
        background: style.backgroundColor,
      };
    });
    const contrast = contrastRatio(colors.foreground, colors.background);

    expect(contrast).toBeGreaterThanOrEqual(7);
  });

  test('lesson exercise includes concise Rust Book guide before answering', async ({ page }) => {
    await page.goto('/exercise/syntax-let-mut');

    await expect(page.getByText('先学这个')).toBeVisible();
    await expect(page.getByText(/Rust 默认让绑定不可变/)).toBeVisible();
    await expect(page.getByText('掌握目标')).toBeVisible();
    await expect(page.getByRole('link', { name: 'Rust Book' })).toBeVisible();
  });

  test('brand icon is rendered as a square mark', async ({ page }) => {
    await page.goto('/');
    const box = await page.locator('.brand-mark').boundingBox();

    expect(box).not.toBeNull();
    expect(Math.abs((box?.width ?? 0) - (box?.height ?? 0))).toBeLessThanOrEqual(1);
  });

  test('desktop learning category rail can collapse without hiding the toggle', async ({ page }) => {
    await page.setViewportSize({ width: 1280, height: 900 });
    await page.goto('/learn');

    const rail = page.locator('.side-rail');
    const content = page.locator('.rail-content');
    const collapseButton = page.getByRole('button', { name: /收起分类|Collapse categories/ });
    await expect(collapseButton).toBeVisible();

    const openBox = await rail.boundingBox();
    await collapseButton.click();

    await expect(rail).toHaveClass(/is-collapsed/);
    await expect(content).toHaveAttribute('aria-hidden', 'true');
    await expect(page.getByRole('button', { name: /展开分类|Open categories/ })).toBeVisible();

    const collapsedBox = await rail.boundingBox();
    expect(openBox).not.toBeNull();
    expect(collapsedBox).not.toBeNull();
    expect(collapsedBox?.width ?? 0).toBeLessThan((openBox?.width ?? 0) * 0.5);
    await assertNoHorizontalOverflow(page);
  });

  test('mobile learning category drawer opens from the left and exposes lesson links', async ({ page }) => {
    await page.setViewportSize({ width: 390, height: 844 });
    await page.goto('/learn');

    const rail = page.locator('.side-rail');
    const railLink = rail.locator('.rail-item').first();
    await expect(page.getByRole('button', { name: /收起分类|Collapse categories/ })).toBeVisible();
    await expect(railLink).toBeVisible();

    const openBox = await rail.boundingBox();
    expect(openBox).not.toBeNull();
    expect(openBox?.x ?? 99).toBeLessThanOrEqual(12);
    expect(openBox?.width ?? 999).toBeLessThanOrEqual(360);

    await page.getByRole('button', { name: /收起分类|Collapse categories/ }).click();
    await expect(rail).toHaveClass(/is-collapsed/);
    await expect(railLink).not.toBeVisible();
    await expect(page.getByRole('button', { name: /展开分类|Open categories/ })).toBeVisible();

    await page.getByRole('button', { name: /展开分类|Open categories/ }).click();
    await expect(rail).toHaveClass(/is-open/);
    await expect(railLink).toBeVisible();
    await assertNoHorizontalOverflow(page);
  });

  test('lesson card action buttons keep breathing room from content', async ({ page }) => {
    await page.goto('/learn');

    const gap = await verticalGapBetweenSiblings(page, '.lesson-card', '.tiny-button');
    expect(gap).toBeGreaterThanOrEqual(10);
  });

  for (const item of exerciseCases) {
    test(`${item.kind} keeps input and submit controls separated`, async ({ page }) => {
      await page.goto(`/exercise/${item.id}`);

      await assertExerciseControlsHaveSpacing(page);
    });
  }
});

async function assertNoHorizontalOverflow(page: Page) {
  const overflow = await page.evaluate(() => document.documentElement.scrollWidth - document.documentElement.clientWidth);
  expect(overflow).toBeLessThanOrEqual(1);
}

async function assertNoVisibleOverlap(page: Page) {
  const overlapCount = await page.locator('.topbar, .side-rail, .hero-card, .panel, .exercise-panel, .stat-card').evaluateAll((elements) => {
    const boxes = elements
      .map((element) => element.getBoundingClientRect())
      .filter((box) => box.width > 0 && box.height > 0);

    let overlaps = 0;
    for (let index = 0; index < boxes.length; index += 1) {
      for (let next = index + 1; next < boxes.length; next += 1) {
        const a = boxes[index];
        const b = boxes[next];
        const separated = a.right <= b.left || b.right <= a.left || a.bottom <= b.top || b.bottom <= a.top;
        if (!separated) {
          overlaps += 1;
        }
      }
    }

    return overlaps;
  });

  expect(overlapCount).toBe(0);
}

function contrastRatio(foreground: string, background: string) {
  const fg = parseRgb(foreground);
  const bg = parseRgb(background);
  const lighter = Math.max(relativeLuminance(fg), relativeLuminance(bg));
  const darker = Math.min(relativeLuminance(fg), relativeLuminance(bg));
  return (lighter + 0.05) / (darker + 0.05);
}

function parseRgb(color: string): [number, number, number] {
  const match = color.match(/rgba?\((\d+),\s*(\d+),\s*(\d+)/);
  if (!match) {
    throw new Error(`Unsupported color: ${color}`);
  }
  return [Number(match[1]), Number(match[2]), Number(match[3])];
}

function relativeLuminance([red, green, blue]: [number, number, number]) {
  const [r, g, b] = [red, green, blue].map((value) => {
    const channel = value / 255;
    return channel <= 0.03928 ? channel / 12.92 : ((channel + 0.055) / 1.055) ** 2.4;
  });
  return 0.2126 * r + 0.7152 * g + 0.0722 * b;
}

async function assertExerciseControlsHaveSpacing(page: Page) {
  const answerStack = page.locator('.answer-stack').first();
  const buttonRow = page.locator('.button-row').first();
  await expect(answerStack).toBeVisible();
  await expect(buttonRow).toBeVisible();

  const answerBox = await answerStack.boundingBox();
  const buttonBox = await buttonRow.boundingBox();
  expect(answerBox).not.toBeNull();
  expect(buttonBox).not.toBeNull();

  const verticalGap = (buttonBox?.y ?? 0) - ((answerBox?.y ?? 0) + (answerBox?.height ?? 0));
  expect(verticalGap).toBeGreaterThanOrEqual(12);

  const innerGaps = await answerStack.evaluate((element) => {
    const directControls = Array.from(element.children).filter((child) =>
      child.matches('button, input, textarea, .feedback'),
    );
    const boxes = directControls
      .map((element) => element.getBoundingClientRect())
      .filter((box) => box.width > 0 && box.height > 0)
      .sort((a, b) => a.top - b.top || a.left - b.left);

    return boxes.slice(1).map((box, index) => box.top - boxes[index].bottom);
  });

  for (const gap of innerGaps) {
    expect(gap).toBeGreaterThanOrEqual(8);
  }
}

async function verticalGapBetweenSiblings(page: Page, parentSelector: string, childSelector: string) {
  return page.locator(parentSelector).first().evaluate((parent, selector) => {
    const target = parent.querySelector(selector as string);
    if (!target) {
      throw new Error(`Missing child ${selector}`);
    }

    const targetBox = target.getBoundingClientRect();
    const previousBottom = Array.from(parent.children)
      .filter((child) => child !== target)
      .map((child) => child.getBoundingClientRect())
      .filter((box) => box.width > 0 && box.height > 0 && box.bottom <= targetBox.top)
      .reduce((bottom, box) => Math.max(bottom, box.bottom), 0);

    return targetBox.top - previousBottom;
  }, childSelector);
}
