import { createFileRoute } from '@tanstack/react-router';
import { createServerFn, useServerFn } from '@tanstack/react-start';
import { Dashboard } from '@polyglot/dashboard-ui';
import type { SecuritySnapshot } from '@polyglot/security-contract';
import { fetchSecuritySnapshot } from '@polyglot/topcoat-client';

const loadSecuritySnapshot = createServerFn({ method: 'GET' }).handler(
  async (): Promise<SecuritySnapshot | null> => {
    try {
      return await fetchSecuritySnapshot();
    } catch {
      return null;
    }
  },
);

export const Route = createFileRoute('/')({
  loader: () => loadSecuritySnapshot(),
  component: Home,
});

function Home() {
  const initialSecuritySnapshot = Route.useLoaderData();
  const refreshSecuritySnapshot = useServerFn(loadSecuritySnapshot);

  return (
    <Dashboard
      initialSecuritySnapshot={initialSecuritySnapshot}
      loadSecuritySnapshot={refreshSecuritySnapshot}
    />
  );
}
