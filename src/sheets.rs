use std::num::ParseIntError;

use anyhow::{anyhow, Result};
use google_sheets4::{
    api::{SpreadsheetMethods, ValueRange},
    hyper::{client::HttpConnector, Client},
    hyper_rustls::{HttpsConnector, HttpsConnectorBuilder},
    oauth2::{read_service_account_key, ServiceAccountAuthenticator},
    Sheets as GoogleSheets,
};
use twilight_model::id::{marker::UserMarker, Id};

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

    pub async fn set_verification_submission_approved(
        &self,
        user_id: Id<UserMarker>,
    ) -> Result<()> {
        let (_, user_id_column) = self.req().values_get(&self.sheet_id, "A:A").doit().await?;
        let user_id_row_idx = user_id_column
            .values
            .ok_or_else(|| anyhow!("user ids column has no value"))?
            .into_iter()
            .skip(1)
            .map(|values| {
                values
                    .first()
                    .and_then(|value| value.as_str())
                    .ok_or_else(|| anyhow!("value in user id column isnt string"))
                    .and_then(|value| value.parse().map_err(|err: ParseIntError| err.into()))
            })
            .collect::<Result<Vec<Id<UserMarker>>>>()?
            .into_iter()
            .position(|id| id == user_id)
            .ok_or_else(|| anyhow!("user id to approve not found in sheet"))?
            .checked_add(2)
            .ok_or_else(|| anyhow!("user id row idx doesnt fit in usize"))?;

        let value = ValueRange {
            major_dimension: None,
            range: None,
            values: Some(vec![vec!["Doğrulandı".into()]]),
        };

        self.req()
            .values_update(value, &self.sheet_id, &format!("G{user_id_row_idx}"))
            .value_input_option("USER_ENTERED")
            .doit()
            .await?;

        Ok(())
    }

    fn req(&self) -> SpreadsheetMethods<'_, HttpsConnector<HttpConnector>> {
        self.sheets.spreadsheets()
    }
}
