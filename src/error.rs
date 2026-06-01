use serde_json::Value;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("ERROR - Message: {0}")]
  Message(String),
  #[error("ERROR - IO: {0}")]
  IoError(#[from] std::io::Error),
  #[error("ERROR - Serde: {0}")]
  SerdeError(#[from] serde_json::Error),
  #[error("ERROR - http client: {0}")]
  NetError(#[from] reqwest::Error),
  #[error("ERROR - http request:\nresponse: {response}\npayload: {payload}")]
  RequestError {
    response: reqwest::Error,
    payload: String, // NOTE: CSM/OCHAMI Apis either returns plain text or a json therefore, we
                     // will just return a String
  },
  /// Structured error payload returned by CSM/HSM endpoints when an
  /// HTTP request fails. `status` is the HTTP status code, `detail` is
  /// the human-readable message extracted from the RFC 7807
  /// `Problem7807` body (`detail` field, falling back to `title`), and
  /// `body` retains the raw JSON so callers needing extension fields
  /// can still reach them without string-parsing the Display output.
  #[error("CSM API: status={status} {detail}")]
  CsmError {
    status: u16,
    detail: String,
    body: Option<Value>,
  },
  #[error("ERROR - Console: {0}")]
  ConsoleError(String),
  #[error("ERROR - CFS Configuration already exists: {0}")]
  ConfigurationAlreadyExistsError(String),
  #[error("Configuration not found")]
  ConfigurationNotFound, // NOTE: I would like to add the session name as a parameter but the error
  // from CSM does not provide it
  #[error("Session could not be found")]
  SessionNotFound, // NOTE: I would like to add the session name as a parameter but the error
  // from CSM does not provide it
  #[error("Authentication token not found in {0}")]
  AuthenticationTokenNotFound(String),
  #[error("Not found: {0}")]
  NotFound(String),
  #[error("Bad request: {0}")]
  BadRequest(String),
  #[error("Conflict: {0}")]
  Conflict(String),
  #[error("ERROR - TOML parse: {0}")]
  TomlEditError(#[from] toml_edit::TomlError),
  #[error("ERROR - TOML serialize: {0}")]
  TomlSerError(#[from] toml::ser::Error),
  #[error("ERROR - Config: {0}")]
  ConfigError(#[from] config::ConfigError),
  #[error("ERROR - Dialoguer: {0}")]
  DialoguerError(#[from] dialoguer::Error),
  #[error("ERROR - K8s: {0}")]
  K8sError(String),
  #[error("YAML error: {0}")]
  YamlError(#[from] serde_yaml::Error),
  #[error("Template render error: {0}")]
  TemplateError(String),
  #[error("JWT malformed: {0}")]
  JwtMalformed(String),
  #[error("Invalid hardware pattern: {0}")]
  InvalidPattern(String),
  #[error("Insufficient resources: {0}")]
  InsufficientResources(String),
  #[error("Missing field: {0}")]
  MissingField(String),
  #[error("Unsupported backend: {0}")]
  UnsupportedBackend(String),
  #[error("Invalid node ID: {0}")]
  InvalidNodeId(String),
  #[error("Kafka error: {0}")]
  KafkaError(String),
  #[error("Hook error: {0}")]
  HookError(String),
  #[error("Local git error: {0}")]
  LocalGitError(String),
}
