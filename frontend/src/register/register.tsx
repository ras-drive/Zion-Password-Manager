import React from 'react'
import ReactDOM from 'react-dom/client'
import { NavBar } from '../components/navbar'
import Form from '../components/register/form';
import '../components/form.css'

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <NavBar />
    <Form />
  </React.StrictMode>
)
