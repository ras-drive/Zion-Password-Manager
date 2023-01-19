import React from 'react'
import ReactDOM from 'react-dom/client'
import './App.css'
import { Home } from './components/home'

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <Home />
  </React.StrictMode>
)
