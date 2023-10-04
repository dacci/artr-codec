'use client';

import  { createContext, ReactNode, useEffect, useState } from 'react';

interface WasmContextProps {
  artr?: typeof import('artr-codec');
}

const initialState: WasmContextProps = {};

export const WasmContext = createContext(initialState);

interface WasmProviderProps {
  readonly children?: ReactNode | ReactNode[];
}

export function WasmProvider({ children }: WasmProviderProps){
  const [state, setState] = useState(initialState);

  useEffect(()=> {
    (async () => {
      const artr = await import('artr-codec');
      await artr.default();
      setState({ artr });
    })();
  }, []);

  return (
    <WasmContext.Provider value={state}>
      {children}
    </WasmContext.Provider>
  );
}
