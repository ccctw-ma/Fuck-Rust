import { expect, test, type Page } from '@playwright/test';

const pages = [
  { path: '/', title: /Rust 阶梯学习站|Rust Ladder/ },
  { path: '/learn', title: /课程路径|Learning Path/ },
  { path: '/cards', title: /知识卡片|Knowledge Cards/ },
  { path: '/stats', title: /学习统计|Stats/ },
  { path: '/exercise/syntax-let-mut', title: /让变量真的可变|Make a variable mutable/ },
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
