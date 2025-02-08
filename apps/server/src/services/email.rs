use anyhow::{anyhow, Context, Result};
use lettre::{
	message::{header::ContentType, MessageBuilder},
	transport::smtp::authentication::Credentials,
	Message, SmtpTransport, Transport,
};

use crate::settings::SETTINGS;

#[derive(Clone)]
pub struct EmailService {
	sender_email: String,
	mailer: SmtpTransport,
}

const SMTP_RELAY: &str = "smtp.gmail.com";

#[allow(dead_code)]
impl EmailService {
	pub fn new() -> Self {
		let credentials = Credentials::new(SETTINGS.smtp.username.to_string(), SETTINGS.smtp.password.to_string());
		let transport = SmtpTransport::relay(SMTP_RELAY)
			.context("Failed to create SMTP relay")
			.unwrap();

		Self {
			sender_email: SETTINGS.smtp.sender_email.to_string(),
			mailer: transport.credentials(credentials).build(),
		}
	}

	pub fn send_verification_email(&self, recipient_email: &str, code: &str) -> Result<()> {
		let email = self
			.prepare_email(recipient_email)
			.subject("Account Verification")
			.body(verification_email_html(code))?;

		self.mailer
			.send(&email)
			.map_err(|error| anyhow!("Failed to send verification email: {}", error))?;

		Ok(())
	}

	pub fn send_password_reset_email(&self, recipient_email: &str, code: &str) -> Result<()> {
		let email = self
			.prepare_email(recipient_email)
			.subject("Password Reset")
			.body(password_reset_html(code))?;

		self.mailer
			.send(&email)
			.map_err(|error| anyhow!("Failed to send password reset email: {}", error))?;

		Ok(())
	}

	pub fn send_welcome_email(&self, recipient_email: &str) -> Result<()> {
		let email = self
			.prepare_email(recipient_email)
			.subject("Welcome to Moner!")
			.body(welcome_email_html())?;

		self.mailer
			.send(&email)
			.map_err(|error| anyhow!("Failed to send welcome email: {}", error))?;

		Ok(())
	}

	pub fn send_password_changed_email(&self, recipient_email: &str) -> Result<()> {
		let email = self
			.prepare_email(recipient_email)
			.subject("Password Changed")
			.body(password_changed_email_html())?;

		self.mailer
			.send(&email)
			.map_err(|error| anyhow!("Failed to send password changed email: {}", error))?;

		Ok(())
	}

	fn prepare_email(&self, recipient_email: &str) -> MessageBuilder {
		Message::builder()
			.from(self.sender_email.parse().unwrap())
			.to(recipient_email.parse().unwrap())
			.header(ContentType::TEXT_HTML)
	}
}

fn verification_email_html(code: &str) -> String {
	format!(
		r#"
		<!doctype html>
		<html lang="en">
		<head>
			<meta charset="UTF-8">
			<meta name="viewport"
			content="width=device-width, user-scalable=no, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0">
			<meta http-equiv="X-UA-Compatible" content="ie=edge">
			<title>Moner - Verify your email</title>
		</head>
		<body>
			<h1>Moner - Verify your email</h1>
			<p>Your verification code is: {code}</p>
		</body>
		</html>
		"#,
	)
}

fn welcome_email_html() -> String {
	r#"
		<!doctype html>
		<html lang="en">
		<head>
			<meta charset="UTF-8">
			<meta name="viewport"
			content="width=device-width, user-scalable=no, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0">
			<meta http-equiv="X-UA-Compatible" content="ie=edge">
			<title>Moner - Welcome</title>
		</head>
		<body>
			<h1>Moner - Welcome</h1>
		</body>
		</html>
		"#
	.to_string()
}

fn password_reset_html(code: &str) -> String {
	format!(
		r#"
		<!doctype html>
		<html lang="en">
		<head>
			<meta charset="UTF-8">
			<meta name="viewport"
			content="width=device-width, user-scalable=no, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0">
			<meta http-equiv="X-UA-Compatible" content="ie=edge">
			<title>Moner - Password Reset</title>
		</head>
		<body>
			<h1>Moner - Reset your password</h1>
			<p>Your code for password reset is: {code}</p>
		</body>
		</html>
		"#,
	)
}

fn password_changed_email_html() -> String {
	r#"
		<!doctype html>
		<html lang="en">
		<head>
			<meta charset="UTF-8">
			<meta name="viewport"
			content="width=device-width, user-scalable=no, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0">
			<meta http-equiv="X-UA-Compatible" content="ie=edge">
			<title>Moner - Password Changed</title>
		</head>
		<body>
			<h1>Moner - Password Changed</h1>
			<p>Your password has been changed.</p>
			<p>You can now log in with your new password.</p>
			<p>If you did not change your password, please contact support.</p>
		</body>
		</html>
		"#
	.to_string()
}
