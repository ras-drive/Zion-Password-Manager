import React from 'react';

export class NavBar extends React.Component {
    constructor(props: any) {
        super(props)
    }

    render() {
        return (
            <nav className='navbar'>
              <div className='logo'>
                <img src='/assets/logo.svg' alt='logo' width='50px' height='50px'></img>
              </div>
              <ul className='nav-links'>
                  <div className='menu'>
                      <li><a href='/'>Home</a></li>
                      <li><a href='/login/login.html'>Log In</a></li>
                      <li><a href='/register/register.html'>Register</a></li>
                      <li><a href='https://github.com/ras-drive/zion-password-manager'>Code</a></li>
                  </div>
              </ul>
          </nav>
          )
    }
}
