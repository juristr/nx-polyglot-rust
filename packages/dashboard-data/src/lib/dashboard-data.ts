export type Trend = 'up' | 'down';

export interface DashboardMetric {
  label: string;
  value: string;
  change: string;
  trend: Trend;
  context: string;
}

export interface TrafficSample {
  period: string;
  requests: number;
  cached: number;
}

export interface RuntimeSample {
  runtime: string;
  requests: number;
  p95: number;
  availability: string;
}

export interface Deployment {
  service: string;
  runtime: 'Node.js' | 'Rust' | 'Edge';
  version: string;
  status: 'Healthy' | 'Building';
  updated: string;
}

export const dashboardMetrics: readonly DashboardMetric[] = [
  {
    label: 'Requests',
    value: '9.84M',
    change: '18.6%',
    trend: 'up',
    context: 'vs. previous 24h',
  },
  {
    label: 'P95 latency',
    value: '124ms',
    change: '12.4%',
    trend: 'down',
    context: 'faster than baseline',
  },
  {
    label: 'Cache hit rate',
    value: '68.2%',
    change: '4.3%',
    trend: 'up',
    context: 'across all regions',
  },
  {
    label: 'Build success',
    value: '98.7%',
    change: '1.8%',
    trend: 'up',
    context: 'last 72 pipelines',
  },
];

export const trafficSamples: readonly TrafficSample[] = [
  { period: '00:00', requests: 0.84, cached: 0.48 },
  { period: '03:00', requests: 0.73, cached: 0.42 },
  { period: '06:00', requests: 0.92, cached: 0.57 },
  { period: '09:00', requests: 1.32, cached: 0.86 },
  { period: '12:00', requests: 1.56, cached: 1.08 },
  { period: '15:00', requests: 1.48, cached: 1.01 },
  { period: '18:00', requests: 1.67, cached: 1.19 },
  { period: '21:00', requests: 1.32, cached: 0.98 },
];

export const runtimeSamples: readonly RuntimeSample[] = [
  { runtime: 'Node.js', requests: 4.82, p95: 148, availability: '99.97%' },
  { runtime: 'Rust', requests: 3.91, p95: 72, availability: '99.99%' },
  { runtime: 'Edge', requests: 1.11, p95: 31, availability: '99.95%' },
];

export const deployments: readonly Deployment[] = [
  {
    service: 'web-dashboard',
    runtime: 'Node.js',
    version: 'web@7f82c1',
    status: 'Healthy',
    updated: '2 min ago',
  },
  {
    service: 'events-ingest',
    runtime: 'Rust',
    version: 'ingest@4a90df',
    status: 'Healthy',
    updated: '11 min ago',
  },
  {
    service: 'usage-rollup',
    runtime: 'Rust',
    version: 'rollup@9ce21b',
    status: 'Building',
    updated: '18 min ago',
  },
  {
    service: 'session-router',
    runtime: 'Edge',
    version: 'router@c114ae',
    status: 'Healthy',
    updated: '24 min ago',
  },
];
