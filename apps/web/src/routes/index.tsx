import { createFileRoute } from '@tanstack/react-router';
import { Dashboard } from '@polyglot/dashboard-ui';

export const Route = createFileRoute('/')({ component: Home });

function Home() {
  return <Dashboard />;
}
