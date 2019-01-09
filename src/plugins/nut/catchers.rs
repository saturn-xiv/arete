use rocket::{http::Status, Catcher};

pub fn catchers() -> Vec<Catcher> {
    catchers![
        non_authoritative_information,
        bad_request,
        unauthorized,
        forbidden,
        not_found,
        unprocessable_entity,
        internal_server_error,
        service_unavailable,
    ]
}

#[catch(203)]
fn non_authoritative_information() -> &'static str {
    Status::NonAuthoritativeInformation.reason
}

#[catch(400)]
fn bad_request() -> &'static str {
    Status::BadRequest.reason
}

#[catch(401)]
fn unauthorized() -> &'static str {
    Status::Unauthorized.reason
}

#[catch(403)]
fn forbidden() -> &'static str {
    Status::Forbidden.reason
}

#[catch(404)]
fn not_found() -> &'static str {
    Status::NotFound.reason
}

#[catch(422)]
fn unprocessable_entity() -> &'static str {
    Status::UnprocessableEntity.reason
}

#[catch(500)]
fn internal_server_error() -> &'static str {
    Status::InternalServerError.reason
}

#[catch(503)]
fn service_unavailable() -> &'static str {
    Status::ServiceUnavailable.reason
}
