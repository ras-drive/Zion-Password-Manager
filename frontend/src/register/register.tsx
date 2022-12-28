import React from 'react'
import ReactDOM from 'react-dom/client'
// import nav_bar from '../App'

const nav_bar = () => {
  return (
    <nav className='navbar'>
    <div className='logo'>
    <img src='/assets/logo.svg' alt='logo' width='50px' height='50px'></img>
    </div>
    <ul className='nav-links'>
        <div className='menu'>
            <li><a href='/'>Home</a></li>
            <li><a href='/login'>Log In</a></li>
            <li><a href='https://github.com/ras-drive/zion-password-manager'>Code</a></li>
        </div>
    </ul>
</nav>
  )
}

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    { nav_bar() }
    <div style={{textAlign: 'center'}}>
      <h1>Register new account</h1>
    </div>
  </React.StrictMode>
)
