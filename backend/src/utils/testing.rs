#[macro_export]
macro_rules! test_app (
    () => ({
        use actix_web::{App, web};
        use actix_files::Files;
        use actix_identity::IdentityMiddleware;
        use actix_session::{SessionMiddleware, storage::CookieSessionStore};
        use cookie::Key;
        use $crate::database;

        let secret_key = Key::generate();

        test::init_service(App::new()
        .app_data(web::Data::new(establish_connection().clone()))
        .wrap(IdentityMiddleware::default())
        .wrap(
            SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                .cookie_name("zion-login".to_string())
                .cookie_secure(false)
                .build(),
        )
        .configure(database::routes::user::configure)
        .service(
            Files::new("/", "../frontend/dist")
                .prefer_utf8(true)
                .index_file("index.html"),
        )
        ).await
    })
);
