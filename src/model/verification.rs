use twilight_model::id::{marker::UserMarker, Id};

#[derive(Debug, Clone)]
pub struct VerificationSubmission {
    pub birthday: String,
    pub email: String,
    pub experience: String,
    pub name_surname: String,
    pub organization: String,
    pub user_id: Id<UserMarker>,
}
