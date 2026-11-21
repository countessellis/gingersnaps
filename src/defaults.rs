#![allow(dead_code)]

use env_logger::TimestampPrecision;
use log::LevelFilter;

use crate::logger::LogTarget;

// Build Constants:

pub(crate) const APP_NAME:    &str = "Gingersnaps";
pub(crate) const BUILD_NAME:  &str = env!("BUILD_NAME");
pub(crate) const VERSION_ID:  &str = env!("CARGO_PKG_VERSION");
pub(crate) const BUILD_TIME:  &str = env!("BUILD_TIME");
pub(crate) const BUILD_ID:    &str = env!("BUILD_ID");
pub(crate) const AUTHORS:     &str = env!("CARGO_PKG_AUTHORS");
pub(crate) const COPYRIGHT:   &str = "2026";
pub(crate) const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub(crate) const LICENSE:     &str = env!("CARGO_PKG_LICENSE");


// Logging Constants:
pub(crate) const DEFAULT_LOG_LEVEL:            LevelFilter                                 = LevelFilter::Debug; // LevelFilter::Info;
pub(crate) const DEFAULT_LOG_FORMAT_TIMESTAMP: Option<env_logger::fmt::TimestampPrecision> = Some(TimestampPrecision::Millis); // None;
pub(crate) const DEFAULT_LOG_FORMAT_LEVEL:     bool                                        = true; // false;
pub(crate) const DEFAULT_LOG_FORMAT_TARGET:    bool                                        = true; // false;
pub(crate) const DEFAULT_LOG_DIR:              &'static str                                = "logs";
pub(crate) const DEFAULT_LOG_FILE:             &'static str                                = "gingersnaps.log";
pub(crate) const DEFAULT_LOG_TARGET:           LogTarget                                   = LogTarget::Stdout;
//pub(crate) const DEFAULT_LOG_TARGET:           LogTarget                                   = LogTarget::File(DEFAULT_LOG_DIR, DEFAULT_LOG_FILE);
pub(crate) const DEFAULT_LOG_NO_DEBUG_MODULES: &[&str]                                     = &[];
pub(crate) const REDACT_LOG_LIST:              &str                                        = "config/redact_list.txt";

// Path Constants:

pub(crate) const DEFAULT_CONFIG_DIR:  &str = "config";
pub(crate) const DEFAULT_CONFIG_FILE: &str = "gingersnaps.cfg";
pub(crate) const DEFAULT_CACHE_DIR:   &str = "cache";

// Gingersnaps Defaults:
pub(crate) const MODE_LIST:           &[&str] = &[ "server","help"];
pub(crate) const DEFAULT_MODE:        &str    = "server";
pub(crate) const DEFAULT_WAIT:        u64     = 3;
pub(crate) const DEFAULT_LISTEN_ADDR: &str    = "0.0.0.0:17187";

// Apron Defaults:
pub(crate) const APRON_VERSION:        u64     = 10;
pub(crate) const APRON_SUPPORTED_AUTH: &[&str] = &["guest"];

// Includes:
pub(crate) const SPLASH_COLOUR:          u8    = 88;
pub(crate) const SPLASH_PNG:             &[u8] = include_bytes!("../resources/gingersnaps.png");
pub(crate) const SPLASH_TXT:             &str  = include_str!("../resources/gingersnaps.txt");
pub(crate) const HELP_TEMPLATE:          &str  = include_str!("../resources/help.tpl");
