use crate::schema::session_cookies::user_email;
use crate::{database::routes::errors::ServiceError, schema::session_cookies};
use actix_identity::Identity;
use diesel::delete;
use diesel::{insert_into, prelude::*};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Insertable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = session_cookies)]
pub struct SessionCookie {
    pub cookie_id: Uuid,
    pub user_email: String,
}

impl SessionCookie {
    pub fn new(email: String) -> Self {
        Self {
            cookie_id: Uuid::new_v4(),
            user_email: email,
        }
    }

    pub fn insert(&self, conn: &mut PgConnection) -> Result<(), ServiceError> {
        match session_cookies::table
            .select(user_email)
            .filter(user_email.eq(self.user_email.clone()))
            .execute(conn)
        {
            Ok(rows) => {
                if rows > 0 {
                    match delete(
                        session_cookies::table.filter(user_email.eq(self.user_email.clone())),
                    )
                    .execute(conn)
                    {
                        Ok(delete_rows) => {
                            if delete_rows > 1 {
                                log::error!("Multiple sessions deleted");
                                panic!("multiple sessions deleted")
                            }
                        }
                        Err(e) => log::error!("{}", e),
                    };
                }
            }
            Err(e) => log::error!("{}", e),
        };

        match insert_into(session_cookies::table)
            .values(self)
            .execute(conn)
        {
            Ok(num) => {
                return {
                    match num {
                        1 => Ok(()),
                        _ => {
                            log::error!("More than one row is being updated at once in sessions cookies table");
                            Err(ServiceError::InternalServerError)
                        }
                    }
                }
            }
            Err(e) => {
                return {
                    log::error!("{}", e);
                    Err(ServiceError::InternalServerError)
                }
            }
        }
    }

    pub fn verify(&self, conn: &mut PgConnection) -> Result<(), ServiceError> {
        let db_cookie = session_cookies::table
            .filter(user_email.eq(self.user_email.clone()))
            .load::<SessionCookie>(conn)
            .expect("session cookie")
            .first()
            .unwrap()
            .clone();

        if self.cookie_id == db_cookie.cookie_id {
            Ok(())
        } else {
            Err(ServiceError::Unauthorized)
        }
    }
}

impl From<Identity> for SessionCookie {
    fn from(value: Identity) -> Self {
        serde_json::from_str(&value.id().expect("session cookie")).unwrap()
    }
}
