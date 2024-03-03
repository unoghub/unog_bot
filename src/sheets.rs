use anyhow::Result;
use google_sheets4::{
    api::{SpreadsheetMethods, ValueRange},
    hyper::{client::HttpConnector, Client},
    hyper_rustls::{HttpsConnector, HttpsConnectorBuilder},
    oauth2::{read_service_account_key, ServiceAccountAuthenticator},
    Sheets as GoogleSheets,
};

use crate::model::verification::VerificationSubmission;

pub struct Sheets {
    sheet_id: String,
    sheets: GoogleSheets<HttpsConnector<HttpConnector>>,
}

impl Sheets {
    pub async fn new(sheet_id: String) -> Result<Self> {
        let hyper_client = Client::builder().build(
            HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .build(),
        );

        let key = read_service_account_key("service_account_key.json").await?;
        let auth = ServiceAccountAuthenticator::with_client(key, hyper_client.clone())
            .build()
            .await?;

        let sheets = GoogleSheets::new(hyper_client, auth);

        Ok(Self { sheet_id, sheets })
    }

    pub async fn append_verification_submission(
        &self,
        submission: VerificationSubmission,
    ) -> Result<()> {
        let value = ValueRange {
            major_dimension: None,
            range: None,
            values: Some(vec![vec![
                submission.user_id.to_string().into(),
                submission.name_surname.into(),
                submission.email.into(),
                submission.birthday.into(),
                submission.experience.into(),
                submission.organization.into(),
                "Doğrulanmadı".into(),
            ]]),
        };

        self.req()
            .values_append(value, &self.sheet_id, "A:A")
            .value_input_option("USER_ENTERED")
            .doit()
            .await?;

        Ok(())
    }

    fn req(&self) -> SpreadsheetMethods<'_, HttpsConnector<HttpConnector>> {
        self.sheets.spreadsheets()
    }
}
