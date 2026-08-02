import { fetchSecuritySnapshot } from './topcoat-client.js';

describe('Topcoat client', () => {
  it('loads the shared Rust contract', async () => {
    const snapshot = {
      sequence: 1,
      generated_at: 1,
      incidents: [],
      trend: [],
      vectors: [],
      summary: {
        active_incidents: 2,
        attacks_per_minute: 84,
        blocked_percent: 92,
        critical_incidents: 1,
        top_target: 'Frankfurt',
      },
    };
    const request = vi.fn(async () =>
      Promise.resolve(new Response(JSON.stringify(snapshot))),
    );

    await expect(
      fetchSecuritySnapshot({
        baseUrl: 'http://topcoat.test/',
        fetch: request,
      }),
    ).resolves.toEqual(snapshot);
    expect(request).toHaveBeenCalledWith(
      'http://topcoat.test/api/security/snapshot',
      expect.any(Object),
    );
  });
});
