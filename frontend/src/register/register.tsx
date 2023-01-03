import React from 'react'
import ReactDOM from 'react-dom/client'
import nav_bar from '../App'
import Form from '../components/register/form';
import '../components/form.css'

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    { nav_bar.nav_bar() }
    <Form></Form>
  </React.StrictMode>
)
