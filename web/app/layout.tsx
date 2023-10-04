import React from 'react';
import type { Metadata } from 'next';
import { ThemeRegistry } from '@/component';
import { WasmProvider } from '@/context';

export const metadata: Metadata = {
  title: 'ARTR Encoder / Decoder',
  description: 'ARTR Encoder / Decoder',
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="ja" style={{ height: '100%' }}>
      <body style={{ height: '100%' }}>
        <WasmProvider>
          <ThemeRegistry options={{ key: 'mui', prepend: true }}>
            {children}
          </ThemeRegistry>
        </WasmProvider>
      </body>
    </html>
  );
}
