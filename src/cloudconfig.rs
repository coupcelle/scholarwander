
use serde::{Deserialize, Serialize};
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
	pub UID: u16,
}



#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct LocaleString{
	pub id: u16,
	pub text: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Resources {
	pub locales: Vec<String>,
	pub strings: Vec<LocaleString>,
}


// #[derive(Debug, Serialize, Deserialize, PartialEq)]
// struct Locale{

// }



// #[derive(Debug, Serialize, Deserialize, PartialEq)]
// enum HandlerType { 
// 	ONE = 1,
// 	TWO = 2,
// 	THREE = 3,
// }


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Handler {
	#[serde(rename = "type")]
	pub handlerType: u8,
	pub enable: bool,
	pub reportUserIdentity: bool,
	pub reportIP: bool,
	pub server: String,
	pub service: String,
	pub port: u16,
	pub useSSL: bool,
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Handlers {
	pub handlers: Vec<Handler>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

// #[derive(Debug, Serialize, Deserialize, PartialEq)]
// enum CredentialsType { 
// 	ONE = 1,
// 	TWO = 2,
// 	THREE = 3,
// }

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Credentials {
	#[serde(rename = "type")]
	pub redentialsType: u8,
	pub UID: String,
	pub seRegex: bool,
	pub lsEnrollment: TLSEnrollment,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TLSEnrollment {
	pub protocol: u8,
	pub URL: String,
	pub caIdentity: String,
	pub keySize: u16,
	pub sanType: u8,
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
pub struct Certificate{
	pub data: String
}
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Action {
	#[serde(rename = "type")]
	pub actionType: ActionType,
	pub failAction: u8,
	pub removeOnFailure: bool,
	pub enable: Option<bool>,
	pub minimumVersion: Option<String>,
	pub customization: Option<Resources>,
	pub credentials: Option<Credentials>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Configuration {
	pub name: String,
	pub profileUUID: String,
	pub customization: Resources,
	pub requireAdminPrivileges: bool,
	pub enforceScreenLock: bool,
	pub enableForgetSSIDSteps: bool,
	pub enableTLSMigration: bool,
	pub mobileconfigDescriptionMacOS: String,
	pub mobileconfigDescriptionIOS: String,
	pub reporting: Handlers,
	pub actions: Vec<Action>,
}




#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CloudConfig {
    pub subscription: Subscription,
	pub organization: Organization,
	pub configurations: Vec<Configuration>,
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