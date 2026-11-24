/**
 * The welcome tour's pages, in order.
 *
 * To add a page: add it here, add its `welcome.*` strings in `lib/i18n.ts`,
 * and draw it in `welcome-tour.tsx` from `components/ui` parts.
 */
export const welcomePages = ["intro", "shortcuts", "ready"] as const;

export type WelcomePage = (typeof welcomePages)[number];

export function pageIndex(page: WelcomePage): number {
  return welcomePages.indexOf(page);
}

export function nextPage(page: WelcomePage): WelcomePage | undefined {
  return welcomePages[pageIndex(page) + 1];
}

export function previousPage(page: WelcomePage): WelcomePage | undefined {
  const index = pageIndex(page);

  return index > 0 ? welcomePages[index - 1] : undefined;
}
