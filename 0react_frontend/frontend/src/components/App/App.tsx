import { AppShell, Navbar, Header } from '@mantine/core';
import HomePage from '../HomePage/HomePage';
import axios from 'axios'

axios.defaults.headers.common['Content-Type'] = 'application/json';


export default function App() {
  return (
    <AppShell
      padding="md"
      navbar={<Navbar width={{ base: 300 }} height={500} p="xs">{/* Navbar content */}</Navbar>}
      header={<Header height={60} p="xs">My Website</Header>}
      styles={(theme) => ({
        main: { backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[8] : theme.colors.gray[0] },
      })}
    >
      <HomePage />
    </AppShell>
  );
}
