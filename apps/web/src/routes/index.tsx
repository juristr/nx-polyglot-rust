import { createFileRoute } from '@tanstack/react-router';
import { createServerFn, useServerFn } from '@tanstack/react-start';
import { Dashboard } from '@polyglot/dashboard-ui';
import { loadSecuritySnapshot as loadSecuritySnapshotFromTopcoat } from '@polyglot/topcoat-client';

const loadSecuritySnapshot = createServerFn({
  method: 'GET',
}).handler(() => loadSecuritySnapshotFromTopcoat());

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
