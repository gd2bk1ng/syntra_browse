/* ================================================================================================
   SYNTRA KERNEL — FRONTEND VISUALIZATION
   ------------------------------------------------------------------------------------------------
        .\s/.
       :: S ::
        '/s\'

   File:        /frontend/src/App.tsx
   Module:      Syntra Kernel — Visualization Root Component
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Entry point for the Syntra Kernel visualization frontend. This component mounts
                the Dashboard, which serves as the primary perceptual surface for observing and
                interacting with the cognitive runtime.

   Notes:
     - This file intentionally remains minimal to keep the root clean.
     - The Dashboard component is responsible for rendering cognitive state, events, and UI panels.
     - Future expansions may introduce routing, multi‑panel layouts, or renderer integration.
   ================================================================================================ */

import React from 'react';
import Dashboard from './Dashboard';

const App: React.FC = () => {
  return <Dashboard />;
};

export default App;
