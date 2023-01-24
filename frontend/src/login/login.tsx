import React from 'react'
import ReactDOM from 'react-dom/client'
import Form from '../components/login/form'
import '../components/form.css'
import { NavBar } from '../components/navbar'

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <NavBar />
    <Form />
  </React.StrictMode>
)
