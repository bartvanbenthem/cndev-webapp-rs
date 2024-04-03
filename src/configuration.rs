//! src/configuration.rs
use handlebars::Handlebars;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct AppState {
    pub settings: Settings,
    pub template: Handlebars<'static>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Settings {
    pub application_port: u16,
    pub mail: MailSettings,
    pub repo: String,
    pub content_path: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MailSettings {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_password: String,
    pub mail_address: String,
    pub tls: bool,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialise our configuration reader
    let settings = config::Config::builder()
        // Add configuration values from a file named `configuration.yaml`.
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;
    // Try to convert the configuration values it read into
    // our Settings type
    settings.try_deserialize::<Settings>()
}

pub fn build_app_state(config: Settings, template: Handlebars<'static>) -> AppState {
    return AppState {
        settings: config,
        template: template,
    };
}
