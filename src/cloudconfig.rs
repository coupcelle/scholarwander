
use serde::{Deserialize, Serialize};
use serde_repr::*;
use serde_xml_rs::{from_str, to_string, Error};
use std::fmt::Display;
use std::fmt;


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Subscription {
	pub purchaseDate: u64,
	pub purchaseExpireDate: u64,
	pub supportExpireDate: u64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Organization {
	pub name: String,
	pub normalizedName: String,
	pub domainName: String,
	pub UID: u32,
}



#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct LocaleString{
	pub id: u16,
	pub text: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Res {
	pub resources: Vec<Res2>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Res2 {
	pub resource: Vec<Resource>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Resource {
	pub locales: Vec<Loc>,
	pub strings: Vec<LocString>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Loc {
	pub locale: Vec<Locales>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Locales {
	#[serde(rename = "$value")]
	pub locale: String
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct LocString {
	pub string: Vec<LocaleString>,
}


// #[derive(Debug, Serialize, Deserialize, PartialEq)]
// enum HandlerType { 
// 	ONE = 1,
// 	TWO = 2,
// 	THREE = 3,
// }

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Hand {
	pub handlers: Vec<Hand2>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Hand2 {
	pub handler: Vec<Handler>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Handler {
	#[serde(rename = "type")]
	pub handlerType: u8,
	pub enable: Option<bool>,
	pub reportUserIdentity: Option<bool>,
	pub reportIP: Option<bool>,
	pub server: Option<String>,
	pub service: Option<String>,
	pub port: Option<u16>,
	pub useSSL: Option<bool>,
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Handlers {
	pub handlers: Vec<Handler>,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ActionType { 
	NONE                    = 0,
	REPORT                  = 1,
	LOCKSCREEN              = 2,
	SETNEWPASSWORD          = 3,
	ENABLEWLAN              = 4,
	ADDWLAN                 = 5,
	BACKUP                  = 6,
	RESTORE                 = 7,
	ADDCERTIFICATE          = 8,
	CAMERA                  = 9,
	WIPEDATA                = 10,
	INSTALLSOFTWARE         = 11,
	CONNECT                 = 12,
	SETPROXYCONFIGURATION   = 13,
	DISABLEWIRELESSONWIRED  = 14,
	REBOOT                  = 15,
	REMOVESSID              = 16,
	GROUPACTION             = 17,
	COPYFILE                = 18,
	RUNCOMMAND              = 19,
	REMOVESECUREW2          = 20,
	SYSCHECK                = 21,
	NAC                     = 22,
	CREDENTIALS             = 23,
	ENABLELAN               = 24,
	ADDLAN                  = 25,
	CUSTOMXML               = 26,
	TIMECHECK               = 27,
	ADDCERTIFICATEFIREFOX   = 28,
	STARTFIREFOX            = 29,
	STOPFIREFOX             = 30,
	SMARTCARD               = 31,
	ENROLL                  = 32,
	CERTIFICATEREUSE        = 33,
	OPENBROWSER             = 34,
	_LAST_                  = 35,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum CredentialsType { 
	USERNAMEPASSWORD               = 0,
	USERNAMEPASSWORD_TLSENROLLMENT = 1,
	WEBSSO_TLSENROLLMENT           = 2,
	SHAREDSECRET                   = 3,
	OSLOGONCREDS                   = 4,
	OSLOGONCREDS_TLSENROLLMENT     = 5,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Credentials {
	#[serde(rename = "type")]
	pub credentialsType: CredentialsType,
	pub UUID: String,
	pub useRegex: bool,
	#[serde(rename = "TLSEnrollment")]
	pub tlsEnrollment: Option<TLSEnrollment>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TLSEnrollment {
	pub protocol: u8,
	pub URL: String,
	#[serde(rename = "CAIdentity")]
	pub caIdentity: String,
	pub keySize: u16,
	#[serde(rename = "SANType")]
	pub sanType: u8,
	#[serde(rename = "PKIClient")]
	pub pkiClient: PKIClient,
	pub useTPM: bool,
	pub requireTPM: bool,
	pub caCertificates: Vec<Certificate>,
	pub webSSOUrl: String,
	pub webSSOConfirmType:u8,
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PKIClient{
	pub usePKIClient: bool,
	pub forcePKIClient: bool,
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct WlanProfile {
	#[serde(rename = "type")]
	pub wlanProfileType: u8,
	pub preferedNetworkLocation: u8,
	pub credentialsUUID: String,
	pub name: String,
	pub scope: u8,
	pub ssids: Vec<SSID>,
	pub eap: EAP,
	pub sso: SSO,
	pub securew2: SecureW2,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SSID {
	pub priority: u8,
	pub ssidConfig: SSIDConfig,
	pub connection: Connection,
	pub security: Security,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SSIDConfig {
	pub name: String,
	pub nonBroadcast: bool
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Connection {
	pub connectionType: u8,
	pub connectionMode: u8,
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Security {
	pub securityType: u8,
	pub encryptionType: u8,
}



#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct EAP {
	pub eapMethod: String,//TODO: make me an enum
	pub authorId: u8,
	pub enableServerValidation: bool,
	pub caCertificates: Vec<Certificate>, 
	pub serverNames: String,
	pub useRegex: bool,
	pub enableFastReconnect: bool,
	pub allowNewConnections: bool,
	pub innerEapOptional: bool,
	pub enableQuarantineChecks: bool,
	pub requireCryptoBinding: bool,
	pub windowsLogon: bool,
	pub cacheUserData: bool,
	pub authMode: u8
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Certificate {
	pub alias: Option<String>,
	pub fingerPrint: Option<String>,
	pub useSystemStore: Option<bool>,
	pub data: Option<String>,
	pub useDpiSSL: Option<bool>,
	pub useFirefoxCertStore: Option<bool>,
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SSO {
	pub useSingleSignOn: bool,
	pub ssoType: u8,
	pub ssoMaxDelay: u8,
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SecureW2 {
	pub bypassBalloonNotification: bool
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Act {
	pub action: Vec<Action>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Action {
	#[serde(rename = "type")]
	pub actionType: ActionType,
	pub failAction: Option<u8>,
	pub removeOnFailure: bool,
	pub enable: Option<bool>,
	pub minimumVersion: Option<String>,
	pub customization: Option<Res>,
	pub credentials: Option<Credentials>,
	pub removeSSID: Option<RemoveSSID>,
	pub wlanProfile: Option<WlanProfile>,
	pub deviceMatches: Option<DevMat>,
	pub checkForIP: Option<bool>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DevMat {
	pub deviceMatch: Vec<DeviceMatch>,

}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DeviceMatch {
	pub deviceMatchAttributes: Vec<DevMatAtt>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DevMatAtt {
	pub deviceMatchAttribute: Vec<DeviceMatchAttribute>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DeviceMatchAttribute {
	pub attribute: String,
	pub operator: String,
	pub value: String,
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RemoveSSID {
	pub name: String,
	pub removeType: u8
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DC {
	pub deviceConfiguration: Vec<DeviceConfiguration>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DeviceConfiguration {
	#[serde(rename = "id")]
	pub identifier: u16,
	pub name: String,
	pub profileUUID: String,
	pub customization: Res,
	pub requireAdminPrivileges: bool,
	pub enforceScreenLock: bool,
	pub enableForgetSSIDSteps: bool,
	pub enableTLSMigration: bool,
	pub mobileconfigDescriptionMacOS: Option<String>,
	pub mobileconfigDescriptionIOS: Option<String>,
	pub reporting: Hand,
	pub actions: Vec<Act>,
}




#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CloudConfig {
    pub subscription: Subscription,
	pub organization: Organization,
	// #[serde(rename = "$value")]
	pub configurations: DC,
}

impl CloudConfig {

	/// Create a CloudConfig instance from xml
	pub fn from_xml(xml: &str) -> Result<CloudConfig, Error> {//<T: std::io::Read>
		from_str(&xml)
	}
}

impl Display for CloudConfig {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CloudConfig({})", self.organization.name)
    }
}


#[cfg(test)]
mod tests {
	use crate::cloudconfig::{CloudConfig, Subscription, Organization, DC};
	use serde_xml_rs::{from_str};


	#[test]
	/// Ensure that the subscription portion of the XML can parse correctly
	/// 
 	/// All identifying data has been replaced with dummy values
	fn parsing_subscription_xml() {

		let result: Subscription = from_str(
			"<subscription>
				<purchaseDate>1689014373000</purchaseDate>
				<purchaseExpireDate>1736562706000</purchaseExpireDate>
				<supportExpireDate>1736562706000</supportExpireDate>
			</subscription>").unwrap();
	}


	#[test]
	/// Ensure that the organization portion of the XML can parse correctly
	/// 
	/// All identifying data has been replaced with dummy values
	fn parsing_organization_xml() {

		let result: Organization = from_str(
			"<organization>
				<name>The Best Test University</name>
				<normalizedName>The_Best_Test_University</normalizedName>
				<domainName>test.edu</domainName>
				<UID>12345</UID>
			</organization>").unwrap();
	}

	#[test]
	/// Ensure that the configurations portion of the XML can parse mostly correctly
	/// 
	/// All identifying data has been replaced with dummy values and some
	/// structures (i.e. items in a list) have had items removed to make them shorter
	fn parsing_configurations_xml() {

		let result: DC = from_str(
			"<configurations>
				<deviceConfiguration id=\"54321\">
					<name>eduroam</name>
					<profileUUID>151C1232-278C-4B2D-BD63-5AAA2C926738</profileUUID>
					<requireAdminPrivileges>true</requireAdminPrivileges>
					<customization>
						<resources>
							<resource>
								<locales>
									<locale>en</locale>
								</locales>
								<strings>
									<string>
										<id>4</id>
										<text>Help</text>
									</string>
								</strings>
							</resource>
						</resources>
					</customization>
					<enforceScreenLock>false</enforceScreenLock>
					<enableForgetSSIDSteps>false</enableForgetSSIDSteps>
					<enableTLSMigration>false</enableTLSMigration>
					<reporting>
						<handlers>
							<handler type=\"1\" />
						</handlers>
					</reporting>
					<actions>
						<action type=\"1\">
							<failAction>2</failAction>
							<removeOnFailure>false</removeOnFailure>
							<minimumVersion>0.5.7</minimumVersion>
						</action>
					</actions>
				</deviceConfiguration>
			</configurations>
			</paladinResponse>"
		).unwrap();
	}

}

