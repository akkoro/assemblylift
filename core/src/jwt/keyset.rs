use std::time::{Duration, SystemTime};

//use base64::{decode_config, URL_SAFE_NO_PAD};
use base64::{Engine as _, engine::general_purpose};
use regex::Regex;
use reqwest;
use ring::signature::{RsaPublicKeyComponents, RSA_PKCS1_2048_8192_SHA256};
use serde::{
    de::DeserializeOwned,
    {Deserialize, Serialize},
};
use serde_json::Value;

use crate::jwt::error::*;
use crate::jwt::jwt::*;

type HeaderBody = String;
pub type Signature = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtKey {
    #[serde(default)] // https://github.com/jfbilodeau/jwks-client/issues/1
    pub e: String,
    pub kty: String,
    pub alg: Option<String>,
    #[serde(default)] // https://github.com/jfbilodeau/jwks-client/issues/1
    pub n: String,
    pub kid: String,
}

impl JwtKey {
    pub fn new(kid: &str, n: &str, e: &str) -> JwtKey {
        JwtKey {
            e: e.to_owned(),
            kty: "JTW".to_string(),
            alg: Some("RS256".to_string()),
            n: n.to_owned(),
            kid: kid.to_owned(),
        }
    }
}

impl Clone for JwtKey {
    fn clone(&self) -> Self {
        JwtKey {
            e: self.e.clone(),
            kty: self.kty.clone(),
            alg: self.alg.clone(),
            n: self.n.clone(),
            kid: self.kid.clone(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct KeyStore {
    key_url: String,
    keys: Vec<JwtKey>,
    refresh_interval: f64,
    load_time: Option<SystemTime>,
    expire_time: Option<SystemTime>,
    refresh_time: Option<SystemTime>,
}

impl KeyStore {
    pub fn new() -> KeyStore {
        let key_store = KeyStore {
            key_url: "".to_owned(),
            keys: vec![],
            refresh_interval: 0.5,
            load_time: None,
            expire_time: None,
            refresh_time: None,
        };

        key_store
    }

    pub async fn new_from(jkws_url: String) -> Result<KeyStore, Error> {
        let mut key_store = KeyStore::new();

        key_store.key_url = jkws_url;

        key_store.load_keys().await?;

        Ok(key_store)
    }
    
    pub fn new_from_blocking(jkws_url: String) -> Result<KeyStore, Error> {
        let mut key_store = KeyStore::new();

        key_store.key_url = jkws_url;

        key_store.load_keys_blocking()?;

        Ok(key_store)
    }

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    pub fn key_set_url(&self) -> &str {
        &self.key_url
    }

    pub async fn load_keys_from(&mut self, url: String) -> Result<(), Error> {
        self.key_url = url;

        self.load_keys().await?;

        Ok(())
    }

    pub async fn load_keys(&mut self) -> Result<(), Error> {
        #[derive(Deserialize)]
        pub struct JwtKeys {
            pub keys: Vec<JwtKey>,
        }

        let mut response = reqwest::get(&self.key_url).await.map_err(|_| err_con("Could not download JWKS"))?;

        let load_time = SystemTime::now();
        self.load_time = Some(load_time);

        let result = KeyStore::cache_max_age(&mut response);

        if let Ok(value) = result {
            let expire = load_time + Duration::new(value, 0);
            self.expire_time = Some(expire);
            let refresh_time = (value as f64 * self.refresh_interval) as u64;
            let refresh = load_time + Duration::new(refresh_time, 0);
            self.refresh_time = Some(refresh);
        }

        let jwks = response.json::<JwtKeys>().await.map_err(|_| err_int("Failed to parse keys"))?;

        jwks.keys.iter().for_each(|k| self.add_key(k));

        Ok(())
    }

    pub fn load_keys_blocking(&mut self) -> Result<(), Error> {
        #[derive(Deserialize)]
        pub struct JwtKeys {
            pub keys: Vec<JwtKey>,
        }

        let mut response = reqwest::blocking::get(&self.key_url).map_err(|_| err_con("Could not download JWKS"))?;
        let load_time = SystemTime::now();
        self.load_time = Some(load_time);

        let result = KeyStore::cache_max_age_blocking(&mut response);

        if let Ok(value) = result {
            let expire = load_time + Duration::new(value, 0);
            self.expire_time = Some(expire);
            let refresh_time = (value as f64 * self.refresh_interval) as u64;
            let refresh = load_time + Duration::new(refresh_time, 0);
            self.refresh_time = Some(refresh);
        }

        let jwks = response.json::<JwtKeys>().map_err(|_| err_int("Failed to parse keys"))?;
        jwks.keys.iter().for_each(|k| self.add_key(k));

        Ok(())
    }

    fn cache_max_age(response: &mut reqwest::Response) -> Result<u64, ()> {
        let header = response.headers().get("cache-control").ok_or(())?;

        let header_text = header.to_str().map_err(|_| ())?;

        let re = Regex::new("max-age\\s*=\\s*(\\d+)").map_err(|_| ())?;

        let captures = re.captures(header_text).ok_or(())?;

        let capture = captures.get(1).ok_or(())?;

        let text = capture.as_str();

        let value = text.parse::<u64>().map_err(|_| ())?;

        Ok(value)
    }
    
    fn cache_max_age_blocking(response: &mut reqwest::blocking::Response) -> Result<u64, ()> {
        let header = response.headers().get("cache-control").ok_or(())?;

        let header_text = header.to_str().map_err(|_| ())?;

        let re = Regex::new("max-age\\s*=\\s*(\\d+)").map_err(|_| ())?;

        let captures = re.captures(header_text).ok_or(())?;

        let capture = captures.get(1).ok_or(())?;

        let text = capture.as_str();

        let value = text.parse::<u64>().map_err(|_| ())?;

        Ok(value)
    }

    /// Fetch a key by key id (KID)
    pub fn key_by_id(&self, kid: &str) -> Option<&JwtKey> {
        self.keys.iter().find(|k| k.kid == kid)
    }

    /// Number of keys in keystore
    pub fn keys_len(&self) -> usize {
        self.keys.len()
    }

    /// Manually add a key to the keystore
    pub fn add_key(&mut self, key: &JwtKey) {
        self.keys.push(key.clone());
    }

    fn decode_segments(&self, token: &str) -> Result<(Header, Payload, Signature, HeaderBody), Error> {
        let raw_segments: Vec<&str> = token.split(".").collect();
        if raw_segments.len() != 3 {
            return Err(err_inv("JWT does not have 3 segments"));
        }

        let header_segment = raw_segments[0];
        let payload_segment = raw_segments[1];
        let signature_segment = raw_segments[2].to_string();

        let header = Header::new(decode_segment::<Value>(header_segment).or(Err(err_hea("Failed to decode header")))?);
        let payload = Payload::new(decode_segment::<Value>(payload_segment).or(Err(err_pay("Failed to decode payload")))?);

        let body = format!("{}.{}", header_segment, payload_segment);

        Ok((header, payload, signature_segment, body))
    }

    pub fn decode(&self, token: &str) -> Result<Jwt, Error> {
        let (header, payload, signature, _) = self.decode_segments(token)?;

        Ok(Jwt::new(header, payload, signature))
    }

    pub fn verify_time(&self, token: &str, time: SystemTime) -> Result<Jwt, Error> {
        let (header, payload, signature, body) = self.decode_segments(token)?;

        if header.alg() != Some("RS256") {
            return Err(err_inv("Unsupported algorithm"));
        }

        let kid = header.kid().ok_or(err_key("No key id"))?;

        let key = self.key_by_id(kid).ok_or(err_key("JWT key does not exists"))?;

        //let e = decode_config(&key.e, URL_SAFE_NO_PAD).or(Err(err_cer("Failed to decode exponent")))?;
        let e = general_purpose::URL_SAFE_NO_PAD.decode(&key.e).or(Err(err_cer("Failed to decode exponent")))?;
        //let n = decode_config(&key.n, URL_SAFE_NO_PAD).or(Err(err_cer("Failed to decode modulus")))?;
        let n = general_purpose::URL_SAFE_NO_PAD.decode(&key.n).or(Err(err_cer("Failed to decode modulus")))?;

        verify_signature(&e, &n, &body, &signature)?;

        let jwt = Jwt::new(header, payload, signature);

        if jwt.expired_time(time).unwrap_or(false) {
            return Err(err_exp("Token expired"));
        }
        if jwt.early_time(time).unwrap_or(false) {
            return Err(err_nbf("Too early to use token (nbf)"));
        }

        Ok(jwt)
    }

    /// Verify a JWT token.
    /// If the token is valid, it is returned.
    ///
    /// A token is considered valid if:
    /// * Is well formed
    /// * Has a `kid` field that matches a public signature `kid
    /// * Signature matches public key
    /// * It is not expired
    /// * The `nbf` is not set to before now
    pub fn verify(&self, token: &str) -> Result<Jwt, Error> {
        self.verify_time(token, SystemTime::now())
    }

    /// Time at which the keys were last refreshed
    pub fn last_load_time(&self) -> Option<SystemTime> {
        self.load_time
    }

    /// True if the keys are expired and should be refreshed
    ///
    /// None if keys do not have an expiration time
    pub fn keys_expired(&self) -> Option<bool> {
        match self.expire_time {
            Some(expire) => Some(expire <= SystemTime::now()),
            None => None,
        }
    }

    /// Specifies the interval (as a fraction) when the key store should refresh it's key.
    ///
    /// The default is 0.5, meaning that keys should be refreshed when we are halfway through the expiration time (similar to DHCP).
    ///
    /// This method does _not_ update the refresh time. Call `load_keys` to force an update on the refresh time property.
    pub fn set_refresh_interval(&mut self, interval: f64) {
        self.refresh_interval = interval;
    }

    /// Get the current fraction time to check for token refresh time.
    pub fn refresh_interval(&self) -> f64 {
        self.refresh_interval
    }

    /// The time at which the keys were loaded
    /// None if the keys were never loaded via `load_keys` or `load_keys_from`.
    pub fn load_time(&self) -> Option<SystemTime> {
        self.load_time
    }

    /// Get the time at which the keys are considered expired
    pub fn expire_time(&self) -> Option<SystemTime> {
        self.expire_time
    }

    /// time at which keys should be refreshed.
    pub fn refresh_time(&self) -> Option<SystemTime> {
        self.refresh_time
    }

    /// Returns `Option<true>` if keys should be refreshed based on the given `current_time`.
    ///
    /// None is returned if the key store does not have a refresh time available. For example, the
    /// `load_keys` function was not called or the HTTP server did not provide a  
    pub fn should_refresh_time(&self, current_time: SystemTime) -> Option<bool> {
        if let Some(refresh_time) = self.refresh_time {
            return Some(refresh_time <= current_time);
        }

        None
    }

    /// Returns `Option<true>` if keys should be refreshed based on the system time.
    ///
    /// None is returned if the key store does not have a refresh time available. For example, the
    /// `load_keys` function was not called or the HTTP server did not provide a  
    pub fn should_refresh(&self) -> Option<bool> {
        self.should_refresh_time(SystemTime::now())
    }
}

fn verify_signature(e: &Vec<u8>, n: &Vec<u8>, message: &str, signature: &str) -> Result<(), Error> {
    let pkc = RsaPublicKeyComponents { e, n };

    let message_bytes = &message.as_bytes().to_vec();
    let signature_bytes = general_purpose::URL_SAFE_NO_PAD.decode(&signature).or(Err(err_sig("Could not base64 decode signature")))?;

    let result = pkc.verify(&RSA_PKCS1_2048_8192_SHA256, &message_bytes, &signature_bytes);

    result.or(Err(err_cer("Signature does not match certificate")))
}

fn decode_segment<T: DeserializeOwned>(segment: &str) -> Result<T, Error> {
    let raw = general_purpose::URL_SAFE_NO_PAD.decode(segment).or(Err(err_inv("Failed to decode segment")))?;
    let slice = String::from_utf8_lossy(&raw);
    let decoded: T = serde_json::from_str(&slice).or(Err(err_inv("Failed to decode segment")))?;

    Ok(decoded)
}
