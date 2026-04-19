import React from 'react'
import { AppShell } from './shell/AppShell'
import { AppProviders } from './providers/AppProviders'

export function App() {
  return (
    <AppProviders>
      <AppShell />
    </AppProviders>
  )
}
