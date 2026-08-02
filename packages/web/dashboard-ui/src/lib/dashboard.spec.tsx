import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import { Dashboard } from './dashboard.js';

describe('Dashboard', () => {
  it('server-renders the dashboard and both chart surfaces', () => {
    const markup = renderToStaticMarkup(<Dashboard />);

    expect(markup).toContain('Operations at a glance');
    expect(markup).toContain('Recent deployments');
    expect(markup).toContain(
      'aria-label="Request and cached request volume over 24 hours"',
    );
    expect(markup).toContain('aria-label="Request volume by runtime"');
  });
});
