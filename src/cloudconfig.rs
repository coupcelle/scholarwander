
use serde::{Deserialize, Serialize};
use serde_xml_rs::{from_str, to_string, Error};
use std::fmt::Display;
use std::fmt;


#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Subscription {
	purchaseDate: u64,
	purchaseExpireDate: u64,
	supportExpireDate: u64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Organization {
	name: String,
	normalizedName: String,
	domainName: String,
	UID: u8,
}



#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct LocaleString{
	id: u8,
	text: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Resources {
	locales: Vec<String>,
	strings: Vec<LocaleString>,
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
struct Handler {
	#[serde(rename = "type")]
	handlerType: u8,
	enable: bool,
	reportUserIdentity: bool,
	reportIP: bool,
	server: String,
	service: String,
	port: u8,
	useSSL: bool,
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Handlers {
	handlers: Vec<Handler>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum ActionType { 
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
struct Credentials {
	#[serde(rename = "type")]
	credentialsType: u8,
	UUID: String,
	useRegex: bool,
	tlsEnrollment: TLSEnrollment,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct TLSEnrollment {
	protocol: u8,
	URL: String,
	caIdentity: String,
	keySize: u8,
	sanType: u8,
	pkiClient: PKIClient,
	useTPM: bool,
	requireTPM: bool,
	caCertificates: Vec<Certificate>,
	webSSOUrl: String,
	webSSOConfirmType:u8,
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct PKIClient{
	usePKIClient: bool,
	forcePKIClient: bool,
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Certificate{
	data: String
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Action {
	#[serde(rename = "type")]
	actionType: ActionType,
	failAction: u8,
	removeOnFailure: bool,
	minimumVersion: String,
	customization: Resources,
	credentials: Credentials,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Configuration {
	name: String,
	profileUUID: String,
	customization: Resources,
	requireAdminPrivileges: bool,
	enforceScreenLock: bool,
	enableForgetSSIDSteps: bool,
	enableTLSMigration: bool,
	mobileconfigDescriptionMacOS: String,
	mobileconfigDescriptionIOS: String,
	reporting: Handlers,
	actions: Vec<Action>,
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