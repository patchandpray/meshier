extern crate hyper;
extern crate hyper_rustls;
extern crate yup_oauth2 as oauth2;
extern crate google_bigquery2 as bigquery2;
use bigquery2::{self as bigquery, Table, Result, Error};
use std::default::Default;
use std::fs::{File, read_to_string};
use bigquery2::Bigquery;
use std::path::Path;

use hyper::net::HttpsConnector;
use hyper_rustls::TlsClient;

const CLIENT_SECRET_FILE: &'static str = "";
const GCP_PROJECT: &'static str = "";
const DATASET_ID: &'static str = "";
const TABLE_ID: &'static str = "";


fn main() {
    // Get authentication token from Google Service account key file
    let client_secret = oauth2::service_account_key_from_file(&CLIENT_SECRET_FILE.to_string()).unwrap();
    let client = hyper::Client::with_connector(HttpsConnector::new(TlsClient::new()));
    let mut access = oauth2::ServiceAccountAccess::new(client_secret, client);

    // Show use the token!
    use yup_oauth2::GetToken;
    println!("{:?}", access.token(vec![&"https://www.googleapis.com/auth/bigquery"]).unwrap());    

    // Get Bigquery Methods available by creating a hub
    let client = hyper::Client::with_connector(HttpsConnector::new(TlsClient::new()));
    let mut hub = bigquery::Bigquery::new(client, access);

    // // As the method needs a request, you would usually fill it with the desired information
    // // into the respective structure. Some of the parts shown here might not be applicable !
    // // Values shown here are possibly random and not representative !
    let mut req = Table::default();

    // // You can configure optional parameters by calling the respective setters at will, and
    // // execute the final call using `doit()`.
    // // Values shown here are possibly random and not representative !
    let result = hub.tables().update(req, GCP_PROJECT, DATASET_ID, TABLE_ID).doit();

    match result {
        Err(e) => match e {
            // The Error enum provides details about what exactly happened.
            // You can also just use its `Debug`, `Display` or `Error` traits
             Error::HttpError(_)
            |Error::MissingAPIKey
            |Error::MissingToken(_)
            |Error::Cancelled
            |Error::UploadSizeLimitExceeded(_, _)
            |Error::Failure(_)
            |Error::BadRequest(_)
            |Error::FieldClash(_)
            |Error::JsonDecodeError(_, _) => println!("{}", e),
        },
        Ok(res) => println!("Success: {:?}", res),
    }
}
