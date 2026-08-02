import type { SecuritySnapshot } from '@polyglot/security-contract';

export interface TopcoatClientOptions {
  baseUrl?: string;
  fetch?: typeof globalThis.fetch;
}

export class TopcoatUnavailableError extends Error {
  constructor(status?: number) {
    super(
      status
        ? `Topcoat security API returned ${status}`
        : 'Topcoat security API unavailable',
    );
    this.name = 'TopcoatUnavailableError';
  }
}

export async function fetchSecuritySnapshot(
  options: TopcoatClientOptions = {},
): Promise<SecuritySnapshot> {
  const baseUrl = (
    options.baseUrl ??
    process.env['TOPCOAT_SECURITY_URL'] ??
    'http://127.0.0.1:3000'
  ).replace(/\/$/, '');
  const request = options.fetch ?? globalThis.fetch;

  let response: Response;
  try {
    response = await request(`${baseUrl}/api/security/snapshot`, {
      headers: { accept: 'application/json' },
      signal: AbortSignal.timeout(2_000),
    });
  } catch {
    throw new TopcoatUnavailableError();
  }

  if (!response.ok) {
    throw new TopcoatUnavailableError(response.status);
  }

  return (await response.json()) as SecuritySnapshot;
}
