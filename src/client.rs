use std::collections::HashMap;
use reqwest::blocking::{Client, Response};
use serde_json::json;
use mpesa_derive::MpesaSecurity;

use super::environment::Environment;
use crate::{CommandId, IdentifierTypes};
use super::payloads::{B2bResponse,B2cResponse,AuthResponse,C2bRegisterResponse,C2bSimulateResponse};
use crate::payloads::{B2bPayload,B2cPayload,C2bRegisterPayload,C2bSimulatePayload};
use crate::payloads::ResponseType;
use crate::payloads::{AccountBalanceResponse,AccountBalancePayload};
use crate::mpesa_security::MpesaSecurity;

/// Mpesa client that will facilitate communication with the Safaricom API
#[derive(Debug, MpesaSecurity)]
pub struct Mpesa {
    client_key: String,
    client_secret: String,
    environment: Environment,
    initiator_password: String,
}

impl Mpesa {
    /// Constructs a new `Mpesa` instance.
    pub fn new(client_key: String, client_secret: String, environment: Environment, initiator_password: String) -> Self {
        Self {
            client_key,
            client_secret,
            environment,
            initiator_password,
        }
    }

    /// Generates an access token
    /// Sends `GET` request to Safaricom oauth to acquire token for token authentication
    /// The OAuth access token expires after an hour, after which, you will need to generate another access token
    fn auth(&self) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}/oauth/v1/generate?grant_type=client_credentials", self.environment.base_url());

        let resp: AuthResponse = Client::new().get(&url)
            .basic_auth(&self.client_key, Some(&self.client_secret))
            .send()?
            .json()?;

        Ok(resp.access_token)
    }

    /// # B2C API
    /// Sends b2c payment request.
    ///
    /// This API enables Business to Customer (B2C) transactions between a company and
    /// customers who are the end-users of its products or services. Use of this API requires a
    /// valid and verified B2C M-Pesa Short code.
    /// See more at: https://developer.safaricom.co.ke/docs?shell#b2c-api
    ///
    /// # Example
    /// ```
    /// dotenv::dotenv().ok();
    ///
    /// let client = mpesa::Mpesa::new(
    ///    std::env::var("CLIENT_KEY").unwrap(),
    ///    std::env::var("CLIENT_SECRET").unwrap(),
    ///    mpesa::Environment::Sandbox,
    ///    std::env::var("INIT_PASSWORD").unwrap(),
    /// );
    ///
    /// let response = client.b2c(
    ///         "testapi496",
    ///         mpesa::CommandId::BusinessPayment,
    ///         1000,
    ///         "600496",
    ///         "254708374149",
    ///         "gg",
    ///         "https://muriuki.dev/api/a",
    ///         "https://muriuki.dev/api/b",
    ///         "Test",
    ///     ).unwrap();
    /// ```
    ///
    /// # Errors
    /// TODO
    pub fn b2c(
        &self,
        initiator_name: &str,
        command_id: CommandId,
        amount: u32,
        party_a: &str,
        party_b: &str,
        remarks: &str,
        queue_timeout_url: &str,
        result_url: &str,
        occasion: &str
    ) -> Result<B2cResponse, Box<dyn std::error::Error>> {
        let url = format!("{}/mpesa/b2c/v1/paymentrequest", self.environment.base_url());
        let credentials = self.gen_security_credentials()?;

        let payload = B2cPayload {
            initiator_name,
            security_credentials: &credentials,
            command_id,
            amount,
            party_a,
            party_b,
            remarks,
            queue_timeout_url,
            result_url,
            occasion,
        };

        let data = json!({
            "InitiatorName": payload.initiator_name,
            "SecurityCredential": payload.security_credentials,
            "CommandID": payload.command_id.to_string(),
            "Amount": payload.amount,
            "PartyA": payload.party_a,
            "PartyB": payload.party_b,
            "Remarks": payload.remarks,
            "QueueTimeOutURL": payload.queue_timeout_url,
            "ResultURL": payload.result_url,
            "Occasion": payload.occasion,
        });

        let response: B2cResponse = Client::new().post(&url)
            .bearer_auth(self.auth()?)
            .json(&data)
            .send()?
            .json()?;

        Ok(response)
    }

    /// # B2B API
    /// Sends b2b payment request.
    ///
    /// This API enables Business to Business (B2B) transactions between a business and another
    /// business. Use of this API requires a valid and verified B2B M-Pesa short code for the
    /// business initiating the transaction and the both businesses involved in the transaction
    /// See more at https://developer.safaricom.co.ke/docs?shell#b2b-api
    ///
    /// # Example
    /// ```
    /// dotenv::dotenv().ok();
    ///
    /// let client = mpesa::Mpesa::new(
    ///    std::env::var("CLIENT_KEY").unwrap(),
    ///    std::env::var("CLIENT_SECRET").unwrap(),
    ///    mpesa::Environment::Sandbox,
    ///    std::env::var("INIT_PASSWORD").unwrap(),
    /// );
    ///
    /// let b2b_response = client.b2b(
    ///         "testapi496",
    ///         mpesa::CommandId::BusinessToBusinessTransfer,
    ///         1000,
    ///         "600496",
    ///         4,
    ///         "600000",
    ///         4,
    ///         "gg",
    ///         "https://muriuki.dev/api/a",
    ///         "https://muriuki.dev/api/b",
    ///         "254708374149",
    ///     ).unwrap();
    /// ```
    /// # Errors
    /// TODO
    pub fn b2b(
        &self,
        initiator_name: &str,
        command_id: CommandId,
        amount: u32,
        party_a: &str,
        sender_id: u32,
        party_b: &str,
        receiver_id: u32,
        remarks: &str,
        queue_timeout_url: &str,
        result_url: &str,
        account_ref: &str,
    ) -> Result<B2bResponse,Box<dyn std::error::Error>> {
        let url = format!("{}/mpesa/b2b/v1/paymentrequest", self.environment.base_url());
        let credentials = self.gen_security_credentials()?;

        let payload = B2bPayload {
            initiator_name,
            security_credentials: &credentials,
            command_id,
            amount,
            party_a,
            sender_id,
            party_b,
            receiver_id,
            remarks,
            queue_timeout_url,
            result_url,
            account_ref,
        };

        let data = json!({
            "Initiator": payload.initiator_name,
            "SecurityCredential": payload.security_credentials,
            "CommandID": payload.command_id.to_string(),
            "SenderIdentifierType": payload.sender_id,
            "RecieverIdentifierType": payload.receiver_id,
            "Amount": payload.amount,
            "PartyA": payload.party_a,
            "PartyB": payload.party_b,
            "AccountReference": payload.account_ref,
            "Remarks": payload.remarks,
            "QueueTimeOutURL": payload.queue_timeout_url,
            "ResultURL": payload.result_url,
        });

        let response: B2bResponse = Client::new().post(&url)
            .bearer_auth(self.auth()?)
            .json(&data)
            .send()?
            .json()?;

        Ok(response)
    }

    /// Registers the the 3rd party’s confirmation and validation URLs to M-Pesa
    ///
    /// Registering maps these URLs to the 3rd party shortcode.
    /// Whenever M-Pesa receives a transaction on the shortcode,
    /// M-Pesa triggers a validation request against the validation URL and
    /// the 3rd party system responds to M-Pesa with a validation response (either a success or an error code).
    /// The response expected is the success code the 3rd party
    ///
    /// # Example
    /// ```
    /// dotenv::dotenv().ok();
    ///
    /// let client = mpesa::Mpesa::new(
    ///    std::env::var("CLIENT_KEY").unwrap(),
    ///    std::env::var("CLIENT_SECRET").unwrap(),
    ///    mpesa::Environment::Sandbox,
    ///    std::env::var("INIT_PASSWORD").unwrap(),
    /// );
    ///
    /// let c2b_register_response = client.c2b_register(
    ///         "https://muriuki.dev/api",
    ///         "https://muriuki.dev/verify",
    ///         mpesa::ResponseType::Complete,
    ///         "600496"
    ///     ).unwrap();
    /// ```
    ///
    /// # Errors
    /// TODO
    pub fn c2b_register(
        &self,
        validation_url: &str,
        confirmation_url: &str,
        response_type: ResponseType,
        short_code: &str,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!("{}/mpesa/c2b/v1/registerurl", self.environment.base_url());

        let payload = C2bRegisterPayload {
            validation_url,
            confirmation_url,
            response_type,
            short_code,
        };

        let data = json!({
            "ValidationURL": payload.validation_url,
            "ConfirmationURL": payload.confirmation_url,
            "ResponseType": payload.response_type.to_string(),
            "ShortCode": payload.short_code,
        });

        let response = Client::new().post(&url)
            .bearer_auth(self.auth()?)
            .json(&data)
            .send()?;

        Ok(response)
    }

    /// Make payment requests from Client to Business
    ///
    /// This enables you to receive the payment requests in real time.
    /// See more here: https://developer.safaricom.co.ke/c2b/apis/post/simulate
    ///
    /// # Example
    /// ```
    /// dotenv::dotenv().ok();
    ///
    /// let client = mpesa::Mpesa::new(
    ///    std::env::var("CLIENT_KEY").unwrap(),
    ///    std::env::var("CLIENT_SECRET").unwrap(),
    ///    mpesa::Environment::Sandbox,
    ///    std::env::var("INIT_PASSWORD").unwrap(),
    /// );
    ///
    /// let c2b_simulate_response = client.c2b_simulate(
    ///         mpesa::CommandId::CustomerPayBillOnline,
    ///         1,
    ///         "254705583540",
    ///         "123abc",
    ///         "600496"
    ///     ).unwrap();
    /// ```
    ///
    /// # Errors
    /// TODO
    pub fn c2b_simulate(
        &self,
        command_id: CommandId,
        amount: u32,
        msisdn: &str,
        bill_ref_number: &str,
        short_code: &str,
    ) -> Result<C2bSimulateResponse, Box<dyn std::error::Error>> {
        let url = format!("{}/mpesa/c2b/v1/simulate", self.environment.base_url());

        let payload = C2bSimulatePayload {
            command_id,
            amount,
            msisdn,
            bill_ref_number,
            short_code
        };

        let data = json!({
            "CommandID": payload.command_id.to_string(),
            "Amount": payload.amount,
            "Msisdn": payload.msisdn,
            "BillRefNumber": payload.bill_ref_number,
            "ShortCode": short_code,
        });

        let response: C2bSimulateResponse = Client::new().post(&url)
            .bearer_auth(self.auth()?)
            .json(&data)
            .send()?
            .json()?;

        Ok(response)
    }

    /// Enquire the balance on an M-Pesa BuyGoods (Till Number).
    ///
    /// # Example
    /// ```
    /// dotenv::dotenv().ok();
    ///
    /// let client = mpesa::Mpesa::new(
    ///    std::env::var("CLIENT_KEY").unwrap(),
    ///    std::env::var("CLIENT_SECRET").unwrap(),
    ///    mpesa::Environment::Sandbox,
    ///    std::env::var("INIT_PASSWORD").unwrap(),
    /// );
    ///
    /// let account_balance_response = client.account_balance(
    ///         "600496",
    ///         "none",
    ///         "collins",
    ///         "https://hell.world/api",
    ///         "https://hello.world/api"
    ///     ).unwrap();
    /// ```
    ///
    /// # Errors
    /// TODO
    pub fn account_balance(
        &self,
        party_a: &str,
        remarks: &str,
        initiator_name: &str,
        queue_timeout_url: &str,
        result_url: &str,
    ) -> Result<AccountBalanceResponse, Box<dyn std::error::Error>> {
        let url = format!("{}/mpesa/accountbalance/v1/query", self.environment.base_url());
        let credentials = self.gen_security_credentials()?;

        let payload = AccountBalancePayload {
            command_id: CommandId::AccountBalance,
            party_a,
            identifier_type: IdentifierTypes::Shortcode,
            remarks,
            initiator_name,
            queue_timeout_url,
            result_url,
            security_credentials: &credentials,
        };

        let data = json!({
            "CommandID": payload.command_id.to_string(),
            "PartyA": payload.party_a,
            "IdentifierType": "4", // FIXME
            "Remarks": payload.remarks,
            "Initiator": payload.initiator_name,
            "SecurityCredential": payload.security_credentials,
            "QueueTimeOutURL": payload.queue_timeout_url,
            "ResultURL": payload.result_url,
        });

        let response = Client::new().post(&url)
            .bearer_auth(self.auth()?)
            .json(&data)
            .send()?
            .json()?;

        Ok(response)
    }
}