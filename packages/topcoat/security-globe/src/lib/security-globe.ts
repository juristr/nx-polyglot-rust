import type {
  SecurityIncident,
  SecuritySnapshot,
  SecurityStreamEvent,
  Severity,
} from '@polyglot/security-contract';
import Globe, { type GlobeInstance } from 'globe.gl';

const MAX_ARCS = 34;

export function severityColor(severity: Severity): string {
  switch (severity) {
    case 'critical':
      return '#ff615b';
    case 'high':
      return '#ffae57';
    case 'medium':
      return '#4dbbf5';
    case 'low':
      return '#7189a5';
  }
}

function incident(value: object): SecurityIncident {
  return value as SecurityIncident;
}

export class SecurityGlobeElement extends HTMLElement {
  private globe?: GlobeInstance;
  private eventSource?: EventSource;
  private resizeObserver?: ResizeObserver;

  connectedCallback() {
    if (this.globe) return;

    const globe = new Globe(this, { animateIn: true, waitForGlobeReady: true })
      .backgroundColor('#03070b')
      .showGraticules(true)
      .showAtmosphere(true)
      .atmosphereColor('#36b9e8')
      .atmosphereAltitude(0.16)
      .arcStartLat((item: object) => incident(item).source.latitude)
      .arcStartLng((item: object) => incident(item).source.longitude)
      .arcEndLat((item: object) => incident(item).target.latitude)
      .arcEndLng((item: object) => incident(item).target.longitude)
      .arcColor((item: object) => severityColor(incident(item).severity))
      .arcStroke((item: object) =>
        incident(item).severity === 'critical' ? 1.2 : 0.55,
      )
      .arcAltitudeAutoScale(0.35)
      .arcDashLength(0.48)
      .arcDashGap(0.18)
      .arcDashAnimateTime(1_700)
      .arcLabel((item: object) => {
        const current = incident(item);
        return `${current.id}<br>${current.source.city} → ${current.target.city}`;
      })
      .pointLat((item: object) => incident(item).target.latitude)
      .pointLng((item: object) => incident(item).target.longitude)
      .pointColor((item: object) => severityColor(incident(item).severity))
      .pointAltitude(0.012)
      .pointRadius((item: object) =>
        incident(item).severity === 'critical' ? 0.42 : 0.24,
      )
      .ringLat((item: object) => incident(item).target.latitude)
      .ringLng((item: object) => incident(item).target.longitude)
      .ringColor((item: object) => severityColor(incident(item).severity))
      .ringMaxRadius(5)
      .ringPropagationSpeed(2.5)
      .ringRepeatPeriod(900)
      .pointOfView({ lat: 18, lng: 12, altitude: 2.15 });

    const material = globe.globeMaterial();
    material.color.set('#07101c');
    material.emissive.set('#06111e');
    material.emissiveIntensity = 0.65;
    material.shininess = 3;

    globe.controls().autoRotate = true;
    globe.controls().autoRotateSpeed = 0.28;
    globe.controls().enableDamping = true;

    this.globe = globe;
    this.resizeObserver = new ResizeObserver(() => this.resize());
    this.resizeObserver.observe(this);
    this.resize();

    void this.loadSnapshot();
    this.eventSource = new EventSource('/api/security/stream');
    this.eventSource.addEventListener('incident', (event) => {
      const update = JSON.parse(
        (event as MessageEvent<string>).data,
      ) as SecurityStreamEvent;
      this.render(update.snapshot);
    });
  }

  disconnectedCallback() {
    this.eventSource?.close();
    this.resizeObserver?.disconnect();
    this.globe?._destructor();
    this.globe = undefined;
  }

  private async loadSnapshot() {
    const response = await fetch('/api/security/snapshot');
    if (!response.ok) return;
    this.render((await response.json()) as SecuritySnapshot);
  }

  private resize() {
    const width = Math.max(this.clientWidth, 320);
    const height = Math.max(this.clientHeight, 320);
    this.globe?.width(width).height(height);
  }

  private render(snapshot: SecuritySnapshot) {
    const incidents = snapshot.incidents.slice(0, MAX_ARCS);
    this.globe
      ?.arcsData(incidents)
      .pointsData(incidents.slice(0, 22))
      .ringsData(incidents.slice(0, 6));
  }
}

export function registerSecurityGlobe() {
  if (!customElements.get('security-globe')) {
    customElements.define('security-globe', SecurityGlobeElement);
  }
}

registerSecurityGlobe();
