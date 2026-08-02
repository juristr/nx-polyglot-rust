import { severityColor } from './security-globe.js';

describe('security globe', () => {
  it('maps incident severity to stable visual signals', () => {
    expect(severityColor('critical')).toBe('#ff615b');
    expect(severityColor('low')).toBe('#7189a5');
  });
});
