import {
  dashboardMetrics,
  deployments,
  runtimeSamples,
  trafficSamples,
} from '@polyglot/dashboard-data';
import { areaY, barY, defineChart, lineY } from '@tanstack/charts';
import { tooltip } from '@tanstack/charts/tooltip';
import { Chart } from '@tanstack/react-charts';
import { scaleBand, scaleLinear } from 'd3-scale';
import './dashboard.css';

const compactNumber = new Intl.NumberFormat('en-US', {
  maximumFractionDigits: 1,
});

const trafficChart = defineChart({
  marks: [
    areaY(trafficSamples, {
      id: 'traffic-area',
      x: 'period',
      y1: 0,
      y2: 'requests',
      key: 'period',
      fill: '#5b5bd6',
      fillOpacity: 0.1,
    }),
    lineY(trafficSamples, {
      id: 'all-requests',
      x: 'period',
      y: 'requests',
      key: 'period',
      stroke: '#5b5bd6',
      strokeWidth: 2.5,
      points: true,
    }),
    lineY(trafficSamples, {
      id: 'cached-requests',
      x: 'period',
      y: 'cached',
      key: 'period',
      stroke: '#16a085',
      strokeWidth: 2,
      strokeDasharray: '5 5',
    }),
  ],
  x: {
    scale: () => scaleBand().padding(0.14),
    axis: { ticks: { format: (value) => String(value) } },
  },
  y: {
    scale: scaleLinear,
    nice: true,
    grid: true,
    axis: {
      label: 'Requests (millions)',
      ticks: { format: (value) => `${compactNumber.format(value)}M` },
    },
  },
  tooltip: { use: tooltip, className: 'dashboard-tooltip' },
  theme: {
    foreground: '#24252b',
    muted: '#8a8d98',
    grid: '#e8e8ec',
    background: 'transparent',
  },
});

const runtimeChart = defineChart({
  marks: [
    barY(runtimeSamples, {
      id: 'runtime-volume',
      x: 'runtime',
      y: 'requests',
      key: 'runtime',
      fill: (row) =>
        row.runtime === 'Rust'
          ? '#ef7f4d'
          : row.runtime === 'Edge'
            ? '#16a085'
            : '#5b5bd6',
      inset: 12,
      radius: 5,
    }),
  ],
  x: {
    scale: () => scaleBand().padding(0.2),
  },
  y: {
    scale: scaleLinear,
    nice: true,
    grid: true,
    axis: { ticks: { format: (value) => `${compactNumber.format(value)}M` } },
  },
  tooltip: { use: tooltip, className: 'dashboard-tooltip' },
  theme: {
    foreground: '#24252b',
    muted: '#8a8d98',
    grid: '#e8e8ec',
    background: 'transparent',
  },
});

const navItems = ['Overview', 'Deployments', 'Pipelines', 'Runtimes'] as const;

export function Dashboard() {
  return (
    <div className="ops-shell">
      <aside className="ops-sidebar">
        <div className="ops-brand">
          <span className="ops-brand-mark">PX</span>
          <span>Polygraph</span>
        </div>

        <nav className="ops-nav" aria-label="Dashboard navigation">
          <p className="ops-nav-label">Workspace</p>
          {navItems.map((item) => (
            <a
              className={
                item === 'Overview' ? 'ops-nav-item is-active' : 'ops-nav-item'
              }
              href={item === 'Overview' ? '/' : `#${item.toLowerCase()}`}
              key={item}
            >
              <span className="ops-nav-glyph" aria-hidden="true" />
              {item}
            </a>
          ))}
        </nav>

        <div className="ops-repo-card">
          <div className="ops-repo-heading">
            <span className="status-dot" />
            Repository healthy
          </div>
          <code>nx-polyglot/main</code>
          <span>3 projects connected</span>
        </div>
      </aside>

      <main className="ops-main">
        <header className="ops-header">
          <div>
            <p className="ops-eyebrow">Live system overview</p>
            <h1>Operations at a glance</h1>
            <p className="ops-subtitle">
              One workspace. JavaScript at the edge, Rust in the hot path.
            </p>
          </div>
          <div className="ops-header-actions">
            <button className="ops-period" type="button">
              Last 24 hours
              <span aria-hidden="true">⌄</span>
            </button>
            <span className="ops-live">
              <span className="status-dot" />
              Live
            </span>
          </div>
        </header>

        <section className="metric-grid" aria-label="Key metrics">
          {dashboardMetrics.map((metric) => (
            <article className="metric-card" key={metric.label}>
              <p>{metric.label}</p>
              <div className="metric-value-row">
                <strong>{metric.value}</strong>
                <span className={`metric-change is-${metric.trend}`}>
                  {metric.trend === 'up' ? '↑' : '↓'} {metric.change}
                </span>
              </div>
              <span>{metric.context}</span>
            </article>
          ))}
        </section>

        <section className="chart-grid" aria-label="Traffic charts">
          <article className="panel traffic-panel">
            <div className="panel-heading">
              <div>
                <p className="panel-kicker">Throughput</p>
                <h2>Request volume</h2>
              </div>
              <div className="chart-legend" aria-label="Chart legend">
                <span>
                  <i className="legend-swatch is-indigo" />
                  All requests
                </span>
                <span>
                  <i className="legend-swatch is-green" />
                  Cached
                </span>
              </div>
            </div>
            <Chart
              definition={trafficChart}
              height={292}
              initialWidth={760}
              ariaLabel="Request and cached request volume over 24 hours"
            />
          </article>

          <article className="panel runtime-panel">
            <div className="panel-heading">
              <div>
                <p className="panel-kicker">Workload mix</p>
                <h2>Requests by runtime</h2>
              </div>
              <span className="panel-total">9.84M total</span>
            </div>
            <Chart
              definition={runtimeChart}
              height={292}
              initialWidth={420}
              ariaLabel="Request volume by runtime"
            />
          </article>
        </section>

        <section className="bottom-grid">
          <article className="panel deployment-panel" id="deployments">
            <div className="panel-heading">
              <div>
                <p className="panel-kicker">Delivery</p>
                <h2>Recent deployments</h2>
              </div>
              <a href="#pipelines">View pipelines</a>
            </div>
            <div
              className="deployment-table"
              role="table"
              aria-label="Recent deployments"
            >
              <div className="deployment-row deployment-head" role="row">
                <span role="columnheader">Service</span>
                <span role="columnheader">Runtime</span>
                <span role="columnheader">Version</span>
                <span role="columnheader">Status</span>
                <span role="columnheader">Updated</span>
              </div>
              {deployments.map((deployment) => (
                <div
                  className="deployment-row"
                  role="row"
                  key={deployment.service}
                >
                  <strong role="cell">{deployment.service}</strong>
                  <span role="cell" className="runtime-pill">
                    {deployment.runtime}
                  </span>
                  <code role="cell">{deployment.version}</code>
                  <span
                    role="cell"
                    className={`deploy-status is-${deployment.status.toLowerCase()}`}
                  >
                    <span className="status-dot" />
                    {deployment.status}
                  </span>
                  <span role="cell" className="deployment-time">
                    {deployment.updated}
                  </span>
                </div>
              ))}
            </div>
          </article>

          <article className="panel health-panel" id="runtimes">
            <div className="panel-heading">
              <div>
                <p className="panel-kicker">Runtime health</p>
                <h2>Fast where it matters</h2>
              </div>
            </div>
            <div className="runtime-list">
              {runtimeSamples.map((runtime) => (
                <div className="runtime-row" key={runtime.runtime}>
                  <div>
                    <strong>{runtime.runtime}</strong>
                    <span>{runtime.availability} availability</span>
                  </div>
                  <div>
                    <strong>{runtime.p95}ms</strong>
                    <span>P95 latency</span>
                  </div>
                </div>
              ))}
            </div>
            <div className="health-note">
              <span>Rust services</span>
              <strong>51% lower latency</strong>
              <p>than the Node.js workload baseline.</p>
            </div>
          </article>
        </section>
      </main>
    </div>
  );
}
