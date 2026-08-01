import { describe, expect, it } from 'vitest';
import {
  dashboardMetrics,
  deployments,
  runtimeSamples,
  trafficSamples,
} from './dashboard-data.js';

describe('dashboard data', () => {
  it('keeps the displayed request total aligned with runtime volume', () => {
    const totalRequests = runtimeSamples.reduce(
      (total, runtime) => total + runtime.requests,
      0,
    );

    expect(totalRequests).toBeCloseTo(9.84);
    expect(dashboardMetrics[0]).toMatchObject({
      label: 'Requests',
      value: '9.84M',
    });
  });

  it('provides stable chart and deployment identities', () => {
    expect(new Set(trafficSamples.map(({ period }) => period)).size).toBe(
      trafficSamples.length,
    );
    expect(new Set(deployments.map(({ service }) => service)).size).toBe(
      deployments.length,
    );
  });
});
