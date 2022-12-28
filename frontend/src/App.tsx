import './App.css'


function App() {

  return (
    <div className='App'>
      { nav_bar() }
      { about_div() }
      { footer() }
    </div>
  )
}

const about_div = () => {
  return (
    <div id='about'>
    <h1>What is Zion?</h1>
    <p>Zion is an open source password manager built using Rust, and it's suite of blazingly fast libraries!</p>
    <p>Such as...</p>
    <br></br>
    <div>
        <ul>
            <li><p><a href='https://actix.rs/'>Actix</a>:
                Actix is a powerful and extremely fast web framework for Rust that has very much support behind it</p><br></br></li>
            <li><p><a href='https://vitejs.dev/'>Vite</a>:
                A very easy to setup frontend framework that we are using with React in order to build our web pages</p><br></br></li>
            <li><p><a href='https://www.diesel.rs/'>Diesel</a>:
                Diesel is a high level Rust ORM that helps us to run and load PostgreSQL queries over abstractions,
                allowing us to write effective queries against a relational database in Rust easily and safely</p><br></br></li>
        </ul>
        <br></br>
    </div>
    <h1>Why is Zion being developed?</h1>
    <p>Zion is being developed in order to bring a secure password manager to the masses that is built
        with modern technology. Our top priority is making sure that your logins are accessible to you, no
        matter the platform, and to make sure that they are secure from prying eyes.</p>
    <br></br>
    <h1>How can I contribute?</h1>
    <p>If anyone is thinking about even learning more about the inner working of Zion they are more
        than welcome to at out Github <a href='https://github.com/ras-drive/zion-password-manager'>repo</a>.
        Any issues opened about Zion will be responded to quickly and anyone willing to help with them
        would be heavily appreciated!</p>
</div>
    )
}

const nav_bar = () => {
  return (
    <nav className='navbar'>
    <div className='logo'>
    <img src='/assets/logo.svg' alt='logo' width='50px' height='50px'></img>
    </div>
    <ul className='nav-links'>
        <div className='menu'>
            <li><a href='/'>Home</a></li>
            <li><a href='/login/login.html'>Log In</a></li>
            <li><a href='#about'>About</a></li>
            <li><a href='#contact'>Contact</a></li>
            <li><a href='https://github.com/ras-drive/zion-password-manager'>Code</a></li>
        </div>
    </ul>
</nav>
  )
}

const footer = () => {
  return (
    <div>
      <footer id='contact'>
        <h1>Contacts</h1>
        <p>If for any reason you need to contact a member of the project you can do so at any of the following links</p>
        <br></br>
        <ol>
            <li>
                <p>Project Lead: Sarah Petkovic
                    <a href='mailto::rasdrive4165@protonmail.com'> Email </a>
                    <a href='https://github.com/ras-drive/'>Github </a>
                    <a href='https://www.linkedin.com/in/sarah-petkovic-a9272720b/'>LinkedIn</a>
                </p>
            </li>
        </ol>
      </footer>
    </div>
  )
}


export default App
