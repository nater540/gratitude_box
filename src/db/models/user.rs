use chrono::NaiveDateTime;
use diesel::prelude::*;
use std::borrow::Cow;
use anyhow::Result;
use uuid::Uuid;

use crate::db::schema::*;

#[derive(Debug, Queryable, Identifiable, AsChangeset)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
  pub id: Uuid,
  pub slack_id: String,
  pub slack_team_id: String,
  pub updated_at: NaiveDateTime,
  pub points: i32
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
struct NewUser<'a> {
  slack_id: Cow<'a, str>,
  slack_team_id: Cow<'a, str>
}

/*
 * The following section contains accessor functions for finding & creating users.
 * Please only put globally accessible functions here, see below for impl on the user struct.
 */

/// Find or create a new user record.
///
/// # Parameters
/// - `conn`: Mutable reference to the PG connection.
/// - `slack_id`: The Slack ID for the user.
/// - `slack_team_id`: The Slack Team ID for the user.
///
/// # Returns
/// This returns a `Result<User>` that contains either a new user record,
/// or a new user record that was just created based on the provided params.
///
/// # Errors
/// This function should only error in extreme cases due to PG constraint failures, network errors, etc.
pub fn find_or_create<'a, T>(conn: &mut PgConnection, slack_id: T, slack_team_id: T) -> Result<User>
  where T: Into<Cow<'a, str>> {

  let slack_id = slack_id.into();
  let slack_team_id = slack_team_id.into();

  let user = users::table
    .filter(users::slack_id.eq(&*slack_id))
    .filter(users::slack_team_id.eq(&*slack_team_id))
    .first::<User>(conn)
    .optional()?;

  match user {
    Some(usr) => Ok(usr),
    None => {
      let new_user = NewUser { slack_id, slack_team_id };
      Ok(diesel::insert_into(users::table).values(&new_user).get_result(conn)?)
    }
  }
}

/// Finds an existing user record by their UUID.
///
/// # Parameters
/// - `conn`: Mutable reference to the PG connection.
/// - `id`: UUID value for the user you're looking for.
///
/// # Returns
/// This returns a `Result<User>` that contains a user record matching the passed uuid if one was found.
pub fn find_by_id(conn: &mut PgConnection, id: Uuid) -> Result<User> {
  Ok(users::table.filter(users::id.eq(id)).first::<User>(conn)?)
}

/// Finds or creates a new user record, then sets their points to the provided value.
///
/// # Parameters
/// - `conn`: Mutable reference to the PG connection.
/// - `slack_id`: The Slack ID for the user.
/// - `slack_team_id`: The Slack Team ID for the user.
/// - `points`: The number of points to set for the user.
///
/// # Returns
/// This returns a `Result<User>` that contains the updated user record, if the save was successful.
pub fn set_points<'a, T>(conn: &mut PgConnection, slack_id: T, slack_team_id: T, points: i32) -> Result<User>
  where T: Into<Cow<'a, str>> {
  let mut user = find_or_create(conn, slack_id, slack_team_id)?;
  user.points = points;
  Ok(user.save(conn)?)
}

/// Finds or creates a new user record, then adds the provided number of points to their existing total.
///
/// # Parameters
/// - `conn`: Mutable reference to the PG connection.
/// - `slack_id`: The Slack ID for the user.
/// - `slack_team_id`: The Slack Team ID for the user.
/// - `points`: The number of points to add to the user's existing total.
///
/// # Returns
/// This returns a `Result<User>` that contains the updated user record, if the save was successful.
pub fn add_points<'a, T>(conn: &mut PgConnection, slack_id: T, slack_team_id: T, points: i32) -> Result<User>
  where T: Into<Cow<'a, str>> {
  let mut user = find_or_create(conn, slack_id, slack_team_id)?;
  user.points += points;
  Ok(user.save(conn)?)
}

/// Contains helper functions for modifying existing user records.
impl User {
  /// Saves any changes made to an existing user record.
  ///
  /// # Parameters
  /// - `conn`: Mutable reference to the PG connection.
  ///
  /// # Returns
  /// This returns a `Result<User>` that contains the updated user record, if the save was successful.
  pub fn save(&self, conn: &mut PgConnection) -> Result<User> {
    Ok(diesel::update(users::table).set(self).get_result::<User>(conn)?)
  }
}


/*
 * Tests below this point.
 */

#[cfg(test)]
mod tests {
  use super::*;

  const TEST_SLACK_ID: &str = "U123ABC456";
  const TEST_SLACK_TEAM_ID: &str = "TAABBCCDDEE";

  fn create_test_user(conn: &mut PgConnection, slack_id: Option<String>, slack_team_id: Option<String>) -> User {
    let slack_id = slack_id.as_deref().unwrap_or(TEST_SLACK_ID);
    let slack_team_id = slack_team_id.as_deref().unwrap_or(TEST_SLACK_TEAM_ID);

    let new_user = NewUser {
      slack_id: Cow::Borrowed(slack_id),
      slack_team_id: Cow::Borrowed(slack_team_id)
    };

    diesel::insert_into(users::table).values(&new_user).get_result(conn).unwrap()
  }

  #[test]
  fn test_find_or_create_new() {
    let mut conn = crate::db::test_connection();

    let user = find_or_create(&mut conn, TEST_SLACK_ID, TEST_SLACK_TEAM_ID).unwrap();
    assert_eq!(user.slack_id, TEST_SLACK_ID);
    assert_eq!(user.slack_team_id, TEST_SLACK_TEAM_ID);
  }

  #[test]
  fn test_find_or_create_existing() {
    let mut conn = crate::db::test_connection();

    let existing_user = create_test_user(&mut conn, None, None);
    let user = find_or_create(&mut conn, &existing_user.slack_id, &existing_user.slack_team_id).unwrap();
    assert_eq!(user.id, existing_user.id);
    assert_eq!(user.slack_id, existing_user.slack_id);
    assert_eq!(user.slack_team_id, existing_user.slack_team_id);
  }

  #[test]
  fn test_find_by_id() {
    let mut conn = crate::db::test_connection();

    let existing_user = create_test_user(&mut conn, None, None);
    let found_user = find_by_id(&mut conn, existing_user.id).unwrap();
    assert_eq!(found_user.id, existing_user.id);
  }

  #[test]
  fn test_set_points() {
    let mut conn = crate::db::test_connection();

    let existing_user = create_test_user(&mut conn, None, None);
    let updated_user = set_points(&mut conn, existing_user.slack_id, existing_user.slack_team_id, 1000).unwrap();

    assert_eq!(updated_user.id, existing_user.id);
    assert_eq!(updated_user.points, 1000);
  }

  #[test]
  fn test_add_points() {
    let mut conn = crate::db::test_connection();

    let mut existing_user = create_test_user(&mut conn, None, None);
    existing_user.points = 900;
    let existing_user = existing_user.save(&mut conn).unwrap();

    let updated_user = add_points(&mut conn, existing_user.slack_id, existing_user.slack_team_id, 100).unwrap();
    assert_eq!(updated_user.id, existing_user.id);
    assert_eq!(updated_user.points, 1000);
  }

  #[test]
  fn test_user_save_changeset() {
    let mut conn = crate::db::test_connection();

    let mut existing_user = create_test_user(&mut conn, None, None);
    existing_user.points = 1000;

    let updated_user = existing_user.save(&mut conn).unwrap();
    assert_eq!(updated_user.id, existing_user.id);
    assert_eq!(updated_user.points, existing_user.points);
  }
}
