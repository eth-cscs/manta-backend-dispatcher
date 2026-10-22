// The backend traits mirror the CSM/OCHAMI APIs: many query parameters,
// tuple results. Reshaping them would change every backend, so accept it.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

pub mod error;
pub mod interfaces;
pub mod types;
