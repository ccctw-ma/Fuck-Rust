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
    await expect(page.getByRole('link', { name: /查看 ripgrep 源码/ })).toHaveAttribute('href', /github\.com\/BurntSushi\/ripgrep/);
  });

  test('right drawer exposes a VS Code-like Rust playground', async ({ page }) => {
    await page.goto('/learn');

    await page.getByRole('button', { name: /打开 Playground|Open Playground/ }).click();

    const drawer = page.locator('.playground-drawer');
    await expect(drawer).toHaveClass(/is-open/);
    await expect(drawer.getByText('main.rs')).toBeVisible();
    await expect(drawer.getByText(/IntelliSense/)).toBeVisible();
    await expect(drawer.getByText(/面板宽度|Panel width/)).toBeVisible();
    await expect(drawer.locator('.token.keyword').first()).toBeVisible();
    await expect(drawer.locator('.playground-editor')).toHaveValue(/fn main\(\)/);
    await expect(drawer.getByRole('button', { name: /运行代码|Run code/ })).toBeVisible();
    await expect(drawer.getByText(/运行输出|Output/)).toBeVisible();
  });

  test('lesson exercise folds mental model into the richer learn-first module', async ({ page }) => {
    await page.goto('/exercise/enum-if-let-method');

    const demo = page.locator('.demo-panel');
    await expect(page.getByText('答题前知识模块')).not.toBeVisible();
    await expect(demo.getByText('先学这个')).toBeVisible();
    await expect(demo.getByText(/Message::Quit/).first()).toBeVisible();
    await expect(demo.getByText(/答题提示/)).toBeVisible();
    await expect(page.getByText(/enum Message \{ Quit/)).toBeVisible();
  });

  test('brand icon is rendered as a square mark', async ({ page }) => {
    await page.goto('/');
    const box = await page.locator('.brand-mark').boundingBox();

    expect(box).not.toBeNull();
    expect(Math.abs((box?.width ?? 0) - (box?.height ?? 0))).toBeLessThanOrEqual(1);
  });

  test('desktop wide layout keeps learning categories expanded', async ({ page }) => {
    await page.setViewportSize({ width: 1280, height: 900 });
    await page.goto('/learn');

    const rail = page.locator('.side-rail');
    const content = page.locator('.rail-content');
    await expect(rail).toHaveClass(/is-open/);
    await expect(content).toHaveAttribute('aria-hidden', 'false');
    await expect(rail.locator('.rail-item').first()).toBeVisible();
    await expect(page.getByRole('button', { name: /收起分类|Collapse categories/ })).not.toBeVisible();

    const railBox = await rail.boundingBox();
    const panelBox = await page.locator('.panel.full').boundingBox();
    expect(railBox?.width ?? 0).toBeGreaterThanOrEqual(280);
    expect((panelBox?.x ?? 0) - ((railBox?.x ?? 0) + (railBox?.width ?? 0))).toBeGreaterThanOrEqual(20);
    await assertNoHorizontalOverflow(page);
  });

  test('tablet layout folds learning categories below the responsive breakpoint', async ({ page }) => {
    await page.setViewportSize({ width: 900, height: 760 });
    await page.goto('/learn');

    const rail = page.locator('.side-rail');
    await expect(page.getByRole('button', { name: /展开分类|Open categories/ })).toBeVisible();
    await expect(rail).toHaveClass(/is-collapsed/);
    const compactBox = await rail.boundingBox();
    expect(compactBox?.width ?? 999).toBeLessThanOrEqual(56);
    expect(compactBox?.height ?? 999).toBeLessThanOrEqual(56);
    await assertNoHorizontalOverflow(page);
  });

  test('mobile learning category trigger is only a small floating button', async ({ page }) => {
    await page.setViewportSize({ width: 390, height: 844 });
    await page.goto('/learn');

    const rail = page.locator('.side-rail');
    const railLink = rail.locator('.rail-item').first();
    await expect(page.getByRole('button', { name: /展开分类|Open categories/ })).toBeVisible();
    await expect(railLink).not.toBeVisible();

    const compactBox = await rail.boundingBox();
    const panelBox = await page.locator('.panel.full').boundingBox();
    expect(compactBox).not.toBeNull();
    expect(compactBox?.width ?? 999).toBeLessThanOrEqual(50);
    expect(compactBox?.height ?? 999).toBeLessThanOrEqual(50);
    expect(panelBox?.x ?? 0).toBeGreaterThanOrEqual(10);

    await page.getByRole('button', { name: /展开分类|Open categories/ }).click();
    await expect(rail).toHaveClass(/is-open/);
    await expect(railLink).toBeVisible();
    const openBox = await rail.boundingBox();
    expect(openBox?.width ?? 0).toBeLessThanOrEqual(310);
    expect(openBox?.x ?? 99).toBeLessThanOrEqual(1);
    expect(openBox?.y ?? 99).toBeLessThanOrEqual(1);
    expect(openBox?.height ?? 0).toBeGreaterThanOrEqual(840);

    await page.getByRole('button', { name: /收起分类|Collapse categories/ }).click();
    await expect(rail).toHaveClass(/is-collapsed/);
    await expect(railLink).not.toBeVisible();
    await assertNoHorizontalOverflow(page);
  });

  test('exercise teaches syntax before showing the question', async ({ page }) => {
    await page.goto('/exercise/syntax-let-mut');

    const guideBox = await page.locator('.demo-panel').boundingBox();
    const questionBox = await page.locator('.exercise-panel > article').boundingBox();

    expect(guideBox).not.toBeNull();
    expect(questionBox).not.toBeNull();
    expect(guideBox?.y ?? 999).toBeLessThan(questionBox?.y ?? 0);
  });

  test('exercise can navigate to the previous curriculum question', async ({ page }) => {
    await page.goto('/exercise/tuple-index');

    await page.getByRole('link', { name: /上一题|Previous/ }).click();

    await expect(page).toHaveURL(/\/exercise\/function-param-type$/);
    await expect(page.getByText(/第 5\/|5\//).first()).toBeVisible();
  });

  test('exercise route highlights the active lesson in the learning rail', async ({ page }) => {
    await page.goto('/exercise/tuple-index');

    const activeRailItem = page.locator('.rail-item.active');
    await expect(activeRailItem).toBeVisible();
    await expect(activeRailItem).toContainText(/从 pattern 读取读函数签名与返回值|Data Types, Functions, and Returns/);
    await expect(activeRailItem.locator('.status-dot.active')).toBeVisible();
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
  const overlapCount = await page.locator('.topbar, .side-rail.is-open, .hero-card, .panel, .exercise-panel, .stat-card').evaluateAll((elements) => {
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
