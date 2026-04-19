import React from 'react'
import { ThemeProvider } from './ThemeProvider'
import { StoreProvider } from './StoreProvider'

interface AppProvidersProps {
  children: React.ReactNode
}

export function AppProviders({ children }: AppProvidersProps) {
  return (
    <StoreProvider>
      <ThemeProvider>
        {children}
      </ThemeProvider>
    </StoreProvider>
  )
}
