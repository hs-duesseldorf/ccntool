//! This library is a collection of functions to interact with a MySQL
//! used for openDCIM.
//!
//! The `ccntool_core` crate provides a few functions to interact with a
//! MySQL database that is being used by an instance of the
//! openDCIM data center infrastructure management tool.
//!
//! # Example usage
//!
//! ```rust
//! use ccntool_core::*;
//!
//! fn get_all_ports() {
//!     let pool = connectdb().expect("Can't connect to database");
//!     let all_ports = queryall(pool);
//!
//!     println!("All ports: {:?}", all_ports);
//! }
//! ```

use sqlx::{
    mysql::{MySqlPoolOptions, MySqlRow},
    MySql, Pool, Row,
};
use std::time::Duration;

/// Establishes a connection to a MySQL database using provided credentials
/// and base URL, either passed along the function or via dotenvy.
///
/// # Arguments
///
/// * `un` - An optional `String` representing the username to use for the
/// database connection. If not provided, the function will attempt to
/// retrieve the username from the `DCIMUSER` environment variable.
/// * `pw` - An optional `String` representing the password to use for the
/// database connection. If not provided, the function will attempt to
/// retrieve the password from the `DCIMPASSWORD` environment variable.
/// * `burl` - An optional `String` representing the base URL of the database
/// server. If not provided, the function will attempt to retrieve the
/// base URL from the `DCIMHOST` environment variable.
///
/// # Returns
///
/// Returns a `Result` containing a `Pool<MySql>` if the connection was
/// successful, or an `sqlx::Error` if an error occurred.
///
/// # Examples
///
/// ```
/// use ccntool_core::*;
///
/// #[tokio::main]
/// async fn main() {
///     // Username, Password and Hostname are passed via dotenvy
///     let conn = match connectdb(None, None, None) {
///         Ok(pool) => pool,
///         Err(error) => {
///             let error_message = format!("Connection timeout: {error}");
///             self.error = error_message;
///             eprintln!("Error: {}", error);
///             return;
///         }
///     };
///     ..
/// }
/// ```
#[tokio::main]
pub async fn connectdb(
    un: Option<String>,
    pw: Option<String>,
    burl: Option<String>,
) -> Result<Pool<MySql>, sqlx::Error> {
    /*
    TODO:
    - LDAP: get username from environemnt, then have use type in their password?
    https://jstaf.github.io/posts/mariadb-ldap/
    - TLS: enable crate to use openssl or gnutls? Probably openssl..?
    */

    let username: String = match un {
        Some(un) => un,
        _ => {
            dotenvy::dotenv().ok();
            dotenvy::var("DCIMUSER")
                .expect("No Username via env")
                .to_owned()
        }
    };

    let password: String = match pw {
        Some(pw) => pw,
        _ => {
            dotenvy::dotenv().ok();
            dotenvy::var("DCIMPASSWORD")
                .expect("No Password via env")
                .to_owned()
        }
    };

    let baseurl: String = match burl {
        Some(burl) => burl,
        _ => {
            dotenvy::dotenv().ok();
            dotenvy::var("DCIMHOST")
                .expect("No BASEURL via env")
                .to_owned()
        }
    };

    let url: String = format!("mysql://{}:{}@{}:3306/dcim", username, password, baseurl);

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&url)
        .await?;
    Ok(pool)
}

/// This function will query all available, valid wallsocket descriptions.
///
/// # Arguments
///
/// * `conn` - A `Pool<MySql>` representing a connection to the
/// `dcim` database.
///
/// # Returns
///
/// Returns a `Result` containing a `Vec<String>` if the connection was
/// successful, or an `sqlx::Error` if an error occurred.
///
/// # Errors
///
/// Returns an error of type `sqlx::Error` if no matching row is found in
/// the `fac_Ports` table.
///
/// # Example
///
/// ```rust
/// fn main() {
///     let ports: Vec<String> = vec![];
///     let all_ports = queryall(connectdb(None, None, None)
///         .expect("Can't connect to database!"))
///         .unwrap();
///
///     println!("All ports: {:?}", all_ports);
/// }
/// ```

#[tokio::main]
pub async fn queryall(conn: Pool<MySql>) -> Result<Vec<String>, sqlx::Error> {
    let mut allports: Vec<String> = Vec::new();

    // TODO/CHECK: is this regex really fetching _all_ valid Notes?
    let allvalidnotes = sqlx::query(
        r#"
SELECT Notes
FROM fac_Ports
WHERE Notes REGEXP '^[0-9]+.[EU0-9]+.[0-9]+-[0-9a-z/,]+?$'
OR Notes REGEXP '^MT-'
        "#,
    )
    .fetch_all(&mut conn.acquire().await?)
    .await?;

    for row in allvalidnotes {
        let notes: String = row.get("Notes");
        allports.push(notes);
    }

    Ok(allports)
}

/// Executes a SQL query against the `dcim` database and returns a vector
/// of strings containing the results. The query looks for the first port
/// in the `fac_Ports` table with a matching `Notes` field to the `notes`
/// argument, and returns various details about the associated switch and
/// port. If no matching row is found, an error is returned.
///
/// # Arguments
///
/// * `conn` - A `Pool<MySql>` representing a connection to the
/// `dcim` database.
/// * `notes` - A `&str` containing the `Notes` field value to match against
/// in the `fac_Ports` table.
///
/// # Returns
///
/// A `Result` containing a vector of strings, where each string
/// represents a different detail about the switch and port.
/// The vector contains the following fields, in order:
///
/// * `results[0]` - Switch hostname (`Label` field from `fac_Device` table)
/// * `results[1]` - Port description (`Notes` field from `fac_Ports` table)
/// * `results[2]` - Switch port (`Label` field from `fac_Ports` table)
/// * `results[3]` - Switch IP address (`PrimaryIP` field from `fac_Device`
/// table)
/// * `results[4]` - Switch device ID (`DeviceID` field from `fac_Device`
/// table)
///
/// # Errors
///
/// Returns an error of type `sqlx::Error` if no matching row is found in
/// the `fac_Ports` table.
///
/// # Examples
///
/// ```rust
/// fn main() {
///     let results = match myquery(connectdb(None, None, None), "01.1.001-1") {
///         Ok(rows) => {
///            self.error.clear();
///            rows
///         }
///         Err(error) => {
///            let error_message = format!("Received garbage: {error}");
///            self.error = error_message;
///            eprintln!("Error: {error}");
///            return;
///         }
///     };
///     ..
/// }
/// ```
///
#[tokio::main]
pub async fn myquery(conn: Pool<MySql>, notes: &str) -> Result<Vec<String>, sqlx::Error> {
    let mut results: Vec<String> = Vec::new();

    let selectedrows = sqlx::query(
        r#"
SELECT p1.PortNumber AS '@PortNumber', p1.DeviceID AS '@DeviceID',
  p2.ConnectedDeviceID AS '@ConnectedDeviceID',
  p2.ConnectedPort AS '@ConnectedPort', d1.DeviceID AS '@SwitchDeviceID',
  d1.Label AS '@SwitchLabel', p3.Notes AS '@PortNotes',
  p3.Label AS '@SwitchPort', d1.PrimaryIP AS '@SwitchIP'
FROM fac_Ports p1
  JOIN fac_Ports p2 ON p1.DeviceID = p2.DeviceID AND p1.PortNumber = -p2.PortNumber
  JOIN fac_Device d1 ON d1.DeviceID = p2.ConnectedDeviceID
  JOIN fac_Ports p3 ON p3.DeviceID = p2.ConnectedDeviceID AND p3.PortNumber = p2.ConnectedPort
WHERE p1.Notes = ? LIMIT 1
        "#,
    )
    .bind(notes.trim())
    .map(|row: MySqlRow| {
        (
            row.get::<i32, _>("@PortNumber"),
            row.get::<i32, _>("@DeviceID"),
            row.get::<i32, _>("@ConnectedDeviceID"),
            row.get::<i32, _>("@ConnectedPort"),
            row.get::<i32, _>("@SwitchDeviceID"),
            row.get::<String, _>("@SwitchLabel"),
            row.get::<String, _>("@PortNotes"),
            row.get::<String, _>("@SwitchPort"),
            row.get::<String, _>("@SwitchIP"),
        )
    })
    .fetch_all(&mut conn.acquire().await?)
    .await?;

    for i in selectedrows {
        results.push(i.5); // results[0] -> Switch Hostname
        results.push(i.6); // results[1] -> Port Description
        results.push(i.7); // results[2] -> Switch Port
        results.push(i.8); // results[3] -> Switch IP
        results.push(i.4.to_string()); // results[4] -> Switch DeviceID
    }

    if results.is_empty() {
        return Err(sqlx::Error::RowNotFound);
    }

    Ok(results)
}
