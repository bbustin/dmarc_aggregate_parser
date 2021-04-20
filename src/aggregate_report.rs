// Based upon appendix C of the DMARC RFC
// https://tools.ietf.org/html/rfc7489#appendix-C

// Tested against a large number of reports I have from various places and
// had to make certain fields optional even though the spec says they are not.
// Also had to make the version field a String.
// Guess the spec is not being followed to a T.
use std::net::IpAddr;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DateRangeType {
    pub begin:  u32,
    pub end:    u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportMetadataType {
    pub org_name:           String,
    pub email:              String,
    pub extra_contact_info: Option<String>,
    pub report_id:          String,
    pub date_range:         DateRangeType,
    pub error:              Option<Vec<String>>
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub enum AlignmentType {
    r,
    s
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub enum DispositionType {
    none,
    quarantine,
    reject
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolicyPublishedType {
    pub domain:     String,
    pub adkim:      Option<AlignmentType>,
    pub aspf:       Option<AlignmentType>,
    pub p:          DispositionType,
    pub sp:         Option<DispositionType>,
    pub pct:        u8,
    pub fo:         Option<String>
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub enum DMARCResultType {
    pass,
    fail
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub enum PolicyOverrideType {
    forwarded,
    sampled_out,
    trusted_forwarder,
    mailing_list,
    local_policy,
    other
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolicyOverrideReason {
    pub r#type:     PolicyOverrideType,
    pub comment:    Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolicyEvaluatedType {
    pub disposition:    DispositionType,
    pub dkim:           Option<DMARCResultType>,
    pub spf:            Option<DMARCResultType>,
    pub reason:         Option<Vec<PolicyOverrideReason>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RowType {
    pub source_ip:          IpAddr,
    pub count:              u32,
    pub policy_evaluated:   PolicyEvaluatedType
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierType {
    pub envelope_to:    Option<String>,
    pub envelope_from:  Option<String>,
    pub header_from:    String
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub enum DKIMResultType {
    none,
    pass,
    fail,
    policy,
    neutral,
    temperror,
    permerror
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DKIMAuthResultType {
    pub domain:         String,
    pub selector:       Option<String>,
    pub result:         DKIMResultType,
    pub human_result:   Option<String>
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub enum SPFDomainScope {
    helo,
    mfrom
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub enum SPFResultType {
    none,
    neutral,
    pass,
    fail,
    softfail,
    temperror,
    permerror
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SPFAuthResultType {
    pub domain: String,
    pub scope:  Option<SPFDomainScope>,
    pub result: SPFResultType
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResultType {
    pub dkim:   Option<Vec<DKIMAuthResultType>>,
    pub spf:    Vec<SPFAuthResultType>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordType {
    pub row:            RowType,
    pub identifiers:    IdentifierType,
    pub auth_results:   AuthResultType
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub struct feedback {
    pub version:            Option<String>,
    pub report_metadata:    ReportMetadataType,
    pub policy_published:   PolicyPublishedType,
    pub record:             Vec<RecordType>
}
