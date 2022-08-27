use std::borrow::Cow;
use std::env::var;

/// Returns the current AWS profile name.
pub fn aws_profile<'a>() -> Cow<'a, str> {
    get_env("AWS_PROFILE", "default")
}

/// Retrieve the value of an environment variable.
pub fn get_env<'a>(env_var_name: &'a str, default: &'a str) -> Cow<'a, str> {
    if let Ok(value) = var(env_var_name) {
        Cow::Owned(value)
    } else {
        Cow::Borrowed(default)
    }
}
