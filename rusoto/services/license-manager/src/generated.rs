// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================
#![allow(warnings)]

use futures::future;
use futures::Future;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};
use std::error::Error;
use std::fmt;

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
/// <p>Describes automated discovery.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AutomatedDiscoveryInformation {
    /// <p>Time that automated discovery last ran.</p>
    #[serde(rename = "LastRunTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_time: Option<f64>,
}

/// <p>Details about license consumption.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConsumedLicenseSummary {
    /// <p>Number of licenses consumed by the resource.</p>
    #[serde(rename = "ConsumedLicenses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_licenses: Option<i64>,
    /// <p>Resource type of the resource consuming a license.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateLicenseConfigurationRequest {
    /// <p>Description of the license configuration.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Number of licenses managed by the license configuration.</p>
    #[serde(rename = "LicenseCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_count: Option<i64>,
    /// <p>Indicates whether hard or soft license enforcement is used. Exceeding a hard limit blocks the launch of new instances.</p>
    #[serde(rename = "LicenseCountHardLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_count_hard_limit: Option<bool>,
    /// <p>Dimension used to track the license inventory.</p>
    #[serde(rename = "LicenseCountingType")]
    pub license_counting_type: String,
    /// <p><p>License rules. The syntax is #name=value (for example, #allowedTenancy=EC2-DedicatedHost). Available rules vary by dimension.</p> <ul> <li> <p> <code>Cores</code> dimension: <code>allowedTenancy</code> | <code>maximumCores</code> | <code>minimumCores</code> </p> </li> <li> <p> <code>Instances</code> dimension: <code>allowedTenancy</code> | <code>maximumCores</code> | <code>minimumCores</code> | <code>maximumSockets</code> | <code>minimumSockets</code> | <code>maximumVcpus</code> | <code>minimumVcpus</code> </p> </li> <li> <p> <code>Sockets</code> dimension: <code>allowedTenancy</code> | <code>maximumSockets</code> | <code>minimumSockets</code> </p> </li> <li> <p> <code>vCPUs</code> dimension: <code>allowedTenancy</code> | <code>honorVcpuOptimization</code> | <code>maximumVcpus</code> | <code>minimumVcpus</code> </p> </li> </ul></p>
    #[serde(rename = "LicenseRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_rules: Option<Vec<String>>,
    /// <p>Name of the license configuration.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Product information.</p>
    #[serde(rename = "ProductInformationList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_information_list: Option<Vec<ProductInformation>>,
    /// <p>Tags to add to the license configuration.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateLicenseConfigurationResponse {
    /// <p>Amazon Resource Name (ARN) of the license configuration.</p>
    #[serde(rename = "LicenseConfigurationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_configuration_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteLicenseConfigurationRequest {
    /// <p>ID of the license configuration.</p>
    #[serde(rename = "LicenseConfigurationArn")]
    pub license_configuration_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteLicenseConfigurationResponse {}

/// <p>A filter name and value pair that is used to return more specific results from a describe operation. Filters can be used to match a set of resources by specific criteria, such as tags, attributes, or IDs.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Filter {
    /// <p>Name of the filter. Filter names are case-sensitive.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Filter values. Filter values are case-sensitive.</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetLicenseConfigurationRequest {
    /// <p>Amazon Resource Name (ARN) of the license configuration.</p>
    #[serde(rename = "LicenseConfigurationArn")]
    pub license_configuration_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetLicenseConfigurationResponse {
    /// <p>Automated discovery information.</p>
    #[serde(rename = "AutomatedDiscoveryInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_discovery_information: Option<AutomatedDiscoveryInformation>,
    /// <p>Summaries of the licenses consumed by resources.</p>
    #[serde(rename = "ConsumedLicenseSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_license_summary_list: Option<Vec<ConsumedLicenseSummary>>,
    /// <p>Number of licenses assigned to resources.</p>
    #[serde(rename = "ConsumedLicenses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_licenses: Option<i64>,
    /// <p>Description of the license configuration.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Amazon Resource Name (ARN) of the license configuration.</p>
    #[serde(rename = "LicenseConfigurationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_configuration_arn: Option<String>,
    /// <p>Unique ID for the license configuration.</p>
    #[serde(rename = "LicenseConfigurationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_configuration_id: Option<String>,
    /// <p>Number of available licenses.</p>
    #[serde(rename = "LicenseCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_count: Option<i64>,
    /// <p>Sets the number of available licenses as a hard limit.</p>
    #[serde(rename = "LicenseCountHardLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_count_hard_limit: Option<bool>,
    /// <p>Dimension on which the licenses are counted.</p>
    #[serde(rename = "LicenseCountingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_counting_type: Option<String>,
    /// <p>License rules.</p>
    #[serde(rename = "LicenseRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_rules: Option<Vec<String>>,
    /// <p>Summaries of the managed resources.</p>
    #[serde(rename = "ManagedResourceSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_resource_summary_list: Option<Vec<ManagedResourceSummary>>,
    /// <p>Name of the license configuration.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Account ID of the owner of the license configuration.</p>
    #[serde(rename = "OwnerAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account_id: Option<String>,
    /// <p>Product information.</p>
    #[serde(rename = "ProductInformationList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_information_list: Option<Vec<ProductInformation>>,
    /// <p>License configuration status.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Tags for the license configuration.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetServiceSettingsRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetServiceSettingsResponse {
    /// <p>Indicates whether cross-account discovery has been enabled.</p>
    #[serde(rename = "EnableCrossAccountsDiscovery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_cross_accounts_discovery: Option<bool>,
    /// <p>Amazon Resource Name (ARN) of the AWS resource share. The License Manager master account will provide member accounts with access to this share.</p>
    #[serde(rename = "LicenseManagerResourceShareArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_manager_resource_share_arn: Option<String>,
    /// <p>Indicates whether AWS Organizations has been integrated with License Manager for cross-account discovery.</p>
    #[serde(rename = "OrganizationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_configuration: Option<OrganizationConfiguration>,
    /// <p>Regional S3 bucket path for storing reports, license trail event data, discovery data, and so on.</p>
    #[serde(rename = "S3BucketArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_arn: Option<String>,
    /// <p>SNS topic configured to receive notifications from License Manager.</p>
    #[serde(rename = "SnsTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
}

/// <p>An inventory filter.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InventoryFilter {
    /// <p>Condition of the filter.</p>
    #[serde(rename = "Condition")]
    pub condition: String,
    /// <p>Name of the filter.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Value of the filter.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>A license configuration is an abstraction of a customer license agreement that can be consumed and enforced by License Manager. Components include specifications for the license type (licensing by instance, socket, CPU, or vCPU), allowed tenancy (shared tenancy, Dedicated Instance, Dedicated Host, or all of these), host affinity (how long a VM must be associated with a host), and the number of licenses purchased and used.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LicenseConfiguration {
    /// <p>Automated discovery information.</p>
    #[serde(rename = "AutomatedDiscoveryInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_discovery_information: Option<AutomatedDiscoveryInformation>,
    /// <p>Summaries for licenses consumed by various resources.</p>
    #[serde(rename = "ConsumedLicenseSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_license_summary_list: Option<Vec<ConsumedLicenseSummary>>,
    /// <p>Number of licenses consumed. </p>
    #[serde(rename = "ConsumedLicenses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_licenses: Option<i64>,
    /// <p>Description of the license configuration.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Amazon Resource Name (ARN) of the license configuration.</p>
    #[serde(rename = "LicenseConfigurationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_configuration_arn: Option<String>,
    /// <p>Unique ID of the license configuration.</p>
    #[serde(rename = "LicenseConfigurationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_configuration_id: Option<String>,
    /// <p>Number of licenses managed by the license configuration.</p>
    #[serde(rename = "LicenseCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_count: Option<i64>,
    /// <p>Number of available licenses as a hard limit.</p>
    #[serde(rename = "LicenseCountHardLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_count_hard_limit: Option<bool>,
    /// <p>Dimension to use to track the license inventory.</p>
    #[serde(rename = "LicenseCountingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_counting_type: Option<String>,
    /// <p>License rules.</p>
    #[serde(rename = "LicenseRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_rules: Option<Vec<String>>,
    /// <p>Summaries for managed resources.</p>
    #[serde(rename = "ManagedResourceSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_resource_summary_list: Option<Vec<ManagedResourceSummary>>,
    /// <p>Name of the license configuration.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Account ID of the license configuration's owner.</p>
    #[serde(rename = "OwnerAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account_id: Option<String>,
    /// <p>Product information.</p>
    #[serde(rename = "ProductInformationList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_information_list: Option<Vec<ProductInformation>>,
    /// <p>Status of the license configuration.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Describes an association with a license configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LicenseConfigurationAssociation {
    /// <p>Time when the license configuration was associated with the resource.</p>
    #[serde(rename = "AssociationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_time: Option<f64>,
    /// <p>Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "ResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>ID of the AWS account that owns the resource consuming licenses.</p>
    #[serde(rename = "ResourceOwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner_id: Option<String>,
    /// <p>Type of server resource.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>Details about the usage of a resource associated with a license configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LicenseConfigurationUsage {
    /// <p>Time when the license configuration was initially associated with the resource.</p>
    #[serde(rename = "AssociationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_time: Option<f64>,
    /// <p>Number of licenses consumed by the resource.</p>
    #[serde(rename = "ConsumedLicenses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_licenses: Option<i64>,
    /// <p>Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "ResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>ID of the account that owns the resource.</p>
    #[serde(rename = "ResourceOwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner_id: Option<String>,
    /// <p>Status of the resource.</p>
    #[serde(rename = "ResourceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_status: Option<String>,
    /// <p>Type of resource.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>Describes the failure of a license operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LicenseOperationFailure {
    /// <p>Error message.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>Failure time.</p>
    #[serde(rename = "FailureTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_time: Option<f64>,
    /// <p>Reserved.</p>
    #[serde(rename = "MetadataList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_list: Option<Vec<Metadata>>,
    /// <p>Name of the operation.</p>
    #[serde(rename = "OperationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    /// <p>The requester is "License Manager Automated Discovery".</p>
    #[serde(rename = "OperationRequestedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_requested_by: Option<String>,
    /// <p>Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "ResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>ID of the AWS account that owns the resource.</p>
    #[serde(rename = "ResourceOwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner_id: Option<String>,
    /// <p>Resource type.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>Details for associating a license configuration with a resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LicenseSpecification {
    /// <p>Amazon Resource Name (ARN) of the license configuration.</p>
    #[serde(rename = "LicenseConfigurationArn")]
    pub license_configuration_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAssociationsForLicenseConfigurationRequest {
    /// <p>Amazon Resource Name (ARN) of a license configuration.</p>
    #[serde(rename = "LicenseConfigurationArn")]
    pub license_configuration_arn: String,
    /// <p>Maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAssociationsForLicenseConfigurationResponse {
    /// <p>Information about the associations for the license configuration.</p>
    #[serde(rename = "LicenseConfigurationAssociations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_configuration_associations: Option<Vec<LicenseConfigurationAssociation>>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListFailuresForLicenseConfigurationOperationsRequest {
    /// <p>Amazon Resource Name of the license configuration.</p>
    #[serde(rename = "LicenseConfigurationArn")]
    pub license_configuration_arn: String,
    /// <p>Maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListFailuresForLicenseConfigurationOperationsResponse {
    /// <p>License configuration operations that failed.</p>
    #[serde(rename = "LicenseOperationFailureList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_operation_failure_list: Option<Vec<LicenseOperationFailure>>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListLicenseConfigurationsRequest {
    /// <p><p>Filters to scope the results. The following filters and logical operators are supported:</p> <ul> <li> <p> <code>licenseCountingType</code> - The dimension on which licenses are counted (vCPU). Logical operators are <code>EQUALS</code> | <code>NOT<em>EQUALS</code>.</p> </li> <li> <p> <code>enforceLicenseCount</code> - A Boolean value that indicates whether hard license enforcement is used. Logical operators are <code>EQUALS</code> | <code>NOT</em>EQUALS</code>.</p> </li> <li> <p> <code>usagelimitExceeded</code> - A Boolean value that indicates whether the available licenses have been exceeded. Logical operators are <code>EQUALS</code> | <code>NOT_EQUALS</code>.</p> </li> </ul></p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>Amazon Resource Names (ARN) of the license configurations.</p>
    #[serde(rename = "LicenseConfigurationArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_configuration_arns: Option<Vec<String>>,
    /// <p>Maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListLicenseConfigurationsResponse {
    /// <p>Information about the license configurations.</p>
    #[serde(rename = "LicenseConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_configurations: Option<Vec<LicenseConfiguration>>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListLicenseSpecificationsForResourceRequest {
    /// <p>Maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Amazon Resource Name (ARN) of a resource that has an associated license configuration.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListLicenseSpecificationsForResourceResponse {
    /// <p>License configurations associated with a resource.</p>
    #[serde(rename = "LicenseSpecifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_specifications: Option<Vec<LicenseSpecification>>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListResourceInventoryRequest {
    /// <p><p>Filters to scope the results. The following filters and logical operators are supported:</p> <ul> <li> <p> <code>account<em>id</code> - The ID of the AWS account that owns the resource. Logical operators are <code>EQUALS</code> | <code>NOT</em>EQUALS</code>.</p> </li> <li> <p> <code>application<em>name</code> - The name of the application. Logical operators are <code>EQUALS</code> | <code>BEGINS</em>WITH</code>.</p> </li> <li> <p> <code>license<em>included</code> - The type of license included. Logical operators are <code>EQUALS</code> | <code>NOT</em>EQUALS</code>. Possible values are <code>sql-server-enterprise</code> | <code>sql-server-standard</code> | <code>sql-server-web</code> | <code>windows-server-datacenter</code>.</p> </li> <li> <p> <code>platform</code> - The platform of the resource. Logical operators are <code>EQUALS</code> | <code>BEGINS<em>WITH</code>.</p> </li> <li> <p> <code>resource</em>id</code> - The ID of the resource. Logical operators are <code>EQUALS</code> | <code>NOT_EQUALS</code>.</p> </li> </ul></p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<InventoryFilter>>,
    /// <p>Maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListResourceInventoryResponse {
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the resources.</p>
    #[serde(rename = "ResourceInventoryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_inventory_list: Option<Vec<ResourceInventory>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsForResourceRequest {
    /// <p>Amazon Resource Name (ARN) of the license configuration.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>Information about the tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListUsageForLicenseConfigurationRequest {
    /// <p><p>Filters to scope the results. The following filters and logical operators are supported:</p> <ul> <li> <p> <code>resourceArn</code> - The ARN of the license configuration resource. Logical operators are <code>EQUALS</code> | <code>NOT<em>EQUALS</code>.</p> </li> <li> <p> <code>resourceType</code> - The resource type (EC2</em>INSTANCE | EC2<em>HOST | EC2</em>AMI | SYSTEMS<em>MANAGER</em>MANAGED<em>INSTANCE). Logical operators are <code>EQUALS</code> | <code>NOT</em>EQUALS</code>.</p> </li> <li> <p> <code>resourceAccount</code> - The ID of the account that owns the resource. Logical operators are <code>EQUALS</code> | <code>NOT_EQUALS</code>.</p> </li> </ul></p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>Amazon Resource Name (ARN) of the license configuration.</p>
    #[serde(rename = "LicenseConfigurationArn")]
    pub license_configuration_arn: String,
    /// <p>Maximum number of results to return in a single call.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListUsageForLicenseConfigurationResponse {
    /// <p>Information about the license configurations.</p>
    #[serde(rename = "LicenseConfigurationUsageList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_configuration_usage_list: Option<Vec<LicenseConfigurationUsage>>,
    /// <p>Token for the next set of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Summary information about a managed resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ManagedResourceSummary {
    /// <p>Number of resources associated with licenses.</p>
    #[serde(rename = "AssociationCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_count: Option<i64>,
    /// <p>Type of resource associated with a license.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>Reserved.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Metadata {
    /// <p>Reserved.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Reserved.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Configuration information for AWS Organizations.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrganizationConfiguration {
    /// <p>Enables AWS Organization integration.</p>
    #[serde(rename = "EnableIntegration")]
    pub enable_integration: bool,
}

/// <p>Describes product information for a license configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductInformation {
    /// <p><p>Product information filters. The following filters and logical operators are supported:</p> <ul> <li> <p> <code>Application Name</code> - The name of the application. Logical operator is <code>EQUALS</code>.</p> </li> <li> <p> <code>Application Publisher</code> - The publisher of the application. Logical operator is <code>EQUALS</code>.</p> </li> <li> <p> <code>Application Version</code> - The version of the application. Logical operator is <code>EQUALS</code>.</p> </li> <li> <p> <code>Platform Name</code> - The name of the platform. Logical operator is <code>EQUALS</code>.</p> </li> <li> <p> <code>Platform Type</code> - The platform type. Logical operator is <code>EQUALS</code>.</p> </li> <li> <p> <code>License Included</code> - The type of license included. Logical operators are <code>EQUALS</code> and <code>NOT_EQUALS</code>. Possible values are <code>sql-server-enterprise</code> | <code>sql-server-standard</code> | <code>sql-server-web</code> | <code>windows-server-datacenter</code>.</p> </li> </ul></p>
    #[serde(rename = "ProductInformationFilterList")]
    pub product_information_filter_list: Vec<ProductInformationFilter>,
    /// <p>Resource type. The value is <code>SSM_MANAGED</code>.</p>
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
}

/// <p>Describes product information filters.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductInformationFilter {
    /// <p>Logical operator.</p>
    #[serde(rename = "ProductInformationFilterComparator")]
    pub product_information_filter_comparator: String,
    /// <p>Filter name.</p>
    #[serde(rename = "ProductInformationFilterName")]
    pub product_information_filter_name: String,
    /// <p>Filter value.</p>
    #[serde(rename = "ProductInformationFilterValue")]
    pub product_information_filter_value: Vec<String>,
}

/// <p>Details about a resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResourceInventory {
    /// <p>Platform of the resource.</p>
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>Platform version of the resource in the inventory.</p>
    #[serde(rename = "PlatformVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    /// <p>Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "ResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>ID of the resource.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>ID of the account that owns the resource.</p>
    #[serde(rename = "ResourceOwningAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owning_account_id: Option<String>,
    /// <p>Type of resource.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>Details about a tag for a license configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>Tag key.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>Tag value.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagResourceRequest {
    /// <p>Amazon Resource Name (ARN) of the license configuration.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>One or more tags.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourceRequest {
    /// <p>Amazon Resource Name (ARN) of the license configuration.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>Keys identifying the tags to remove.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateLicenseConfigurationRequest {
    /// <p>New description of the license configuration.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Amazon Resource Name (ARN) of the license configuration.</p>
    #[serde(rename = "LicenseConfigurationArn")]
    pub license_configuration_arn: String,
    /// <p>New status of the license configuration.</p>
    #[serde(rename = "LicenseConfigurationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_configuration_status: Option<String>,
    /// <p>New number of licenses managed by the license configuration.</p>
    #[serde(rename = "LicenseCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_count: Option<i64>,
    /// <p>New hard limit of the number of available licenses.</p>
    #[serde(rename = "LicenseCountHardLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_count_hard_limit: Option<bool>,
    /// <p>New license rules.</p>
    #[serde(rename = "LicenseRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_rules: Option<Vec<String>>,
    /// <p>New name of the license configuration.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>New product information.</p>
    #[serde(rename = "ProductInformationList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_information_list: Option<Vec<ProductInformation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateLicenseConfigurationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateLicenseSpecificationsForResourceRequest {
    /// <p>ARNs of the license configurations to add.</p>
    #[serde(rename = "AddLicenseSpecifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_license_specifications: Option<Vec<LicenseSpecification>>,
    /// <p>ARNs of the license configurations to remove.</p>
    #[serde(rename = "RemoveLicenseSpecifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_license_specifications: Option<Vec<LicenseSpecification>>,
    /// <p>Amazon Resource Name (ARN) of the AWS resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateLicenseSpecificationsForResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateServiceSettingsRequest {
    /// <p>Activates cross-account discovery.</p>
    #[serde(rename = "EnableCrossAccountsDiscovery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_cross_accounts_discovery: Option<bool>,
    /// <p>Enables integration with AWS Organizations for cross-account discovery.</p>
    #[serde(rename = "OrganizationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_configuration: Option<OrganizationConfiguration>,
    /// <p>Amazon Resource Name (ARN) of the Amazon S3 bucket where the License Manager information is stored.</p>
    #[serde(rename = "S3BucketArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_arn: Option<String>,
    /// <p>Amazon Resource Name (ARN) of the Amazon SNS topic used for License Manager alerts.</p>
    #[serde(rename = "SnsTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateServiceSettingsResponse {}

/// Errors returned by CreateLicenseConfiguration
#[derive(Debug, PartialEq)]
pub enum CreateLicenseConfigurationError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValue(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>Your resource limits have been exceeded.</p>
    ResourceLimitExceeded(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl CreateLicenseConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateLicenseConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateLicenseConfigurationError::AccessDenied(
                        err.msg,
                    ))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(CreateLicenseConfigurationError::Authorization(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        CreateLicenseConfigurationError::InvalidParameterValue(err.msg),
                    )
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(
                        CreateLicenseConfigurationError::RateLimitExceeded(err.msg),
                    )
                }
                "ResourceLimitExceededException" => {
                    return RusotoError::Service(
                        CreateLicenseConfigurationError::ResourceLimitExceeded(err.msg),
                    )
                }
                "ServerInternalException" => {
                    return RusotoError::Service(CreateLicenseConfigurationError::ServerInternal(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateLicenseConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateLicenseConfigurationError {
    fn description(&self) -> &str {
        match *self {
            CreateLicenseConfigurationError::AccessDenied(ref cause) => cause,
            CreateLicenseConfigurationError::Authorization(ref cause) => cause,
            CreateLicenseConfigurationError::InvalidParameterValue(ref cause) => cause,
            CreateLicenseConfigurationError::RateLimitExceeded(ref cause) => cause,
            CreateLicenseConfigurationError::ResourceLimitExceeded(ref cause) => cause,
            CreateLicenseConfigurationError::ServerInternal(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteLicenseConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteLicenseConfigurationError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValue(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl DeleteLicenseConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteLicenseConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteLicenseConfigurationError::AccessDenied(
                        err.msg,
                    ))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(DeleteLicenseConfigurationError::Authorization(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DeleteLicenseConfigurationError::InvalidParameterValue(err.msg),
                    )
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(
                        DeleteLicenseConfigurationError::RateLimitExceeded(err.msg),
                    )
                }
                "ServerInternalException" => {
                    return RusotoError::Service(DeleteLicenseConfigurationError::ServerInternal(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteLicenseConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteLicenseConfigurationError {
    fn description(&self) -> &str {
        match *self {
            DeleteLicenseConfigurationError::AccessDenied(ref cause) => cause,
            DeleteLicenseConfigurationError::Authorization(ref cause) => cause,
            DeleteLicenseConfigurationError::InvalidParameterValue(ref cause) => cause,
            DeleteLicenseConfigurationError::RateLimitExceeded(ref cause) => cause,
            DeleteLicenseConfigurationError::ServerInternal(ref cause) => cause,
        }
    }
}
/// Errors returned by GetLicenseConfiguration
#[derive(Debug, PartialEq)]
pub enum GetLicenseConfigurationError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValue(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl GetLicenseConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetLicenseConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetLicenseConfigurationError::AccessDenied(
                        err.msg,
                    ))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(GetLicenseConfigurationError::Authorization(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        GetLicenseConfigurationError::InvalidParameterValue(err.msg),
                    )
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(GetLicenseConfigurationError::RateLimitExceeded(
                        err.msg,
                    ))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(GetLicenseConfigurationError::ServerInternal(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetLicenseConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetLicenseConfigurationError {
    fn description(&self) -> &str {
        match *self {
            GetLicenseConfigurationError::AccessDenied(ref cause) => cause,
            GetLicenseConfigurationError::Authorization(ref cause) => cause,
            GetLicenseConfigurationError::InvalidParameterValue(ref cause) => cause,
            GetLicenseConfigurationError::RateLimitExceeded(ref cause) => cause,
            GetLicenseConfigurationError::ServerInternal(ref cause) => cause,
        }
    }
}
/// Errors returned by GetServiceSettings
#[derive(Debug, PartialEq)]
pub enum GetServiceSettingsError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl GetServiceSettingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetServiceSettingsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetServiceSettingsError::AccessDenied(err.msg))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(GetServiceSettingsError::Authorization(err.msg))
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(GetServiceSettingsError::RateLimitExceeded(
                        err.msg,
                    ))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(GetServiceSettingsError::ServerInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetServiceSettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetServiceSettingsError {
    fn description(&self) -> &str {
        match *self {
            GetServiceSettingsError::AccessDenied(ref cause) => cause,
            GetServiceSettingsError::Authorization(ref cause) => cause,
            GetServiceSettingsError::RateLimitExceeded(ref cause) => cause,
            GetServiceSettingsError::ServerInternal(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAssociationsForLicenseConfiguration
#[derive(Debug, PartialEq)]
pub enum ListAssociationsForLicenseConfigurationError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>The request uses too many filters or too many filter values.</p>
    FilterLimitExceeded(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValue(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl ListAssociationsForLicenseConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListAssociationsForLicenseConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        ListAssociationsForLicenseConfigurationError::AccessDenied(err.msg),
                    )
                }
                "AuthorizationException" => {
                    return RusotoError::Service(
                        ListAssociationsForLicenseConfigurationError::Authorization(err.msg),
                    )
                }
                "FilterLimitExceededException" => {
                    return RusotoError::Service(
                        ListAssociationsForLicenseConfigurationError::FilterLimitExceeded(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        ListAssociationsForLicenseConfigurationError::InvalidParameterValue(
                            err.msg,
                        ),
                    )
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(
                        ListAssociationsForLicenseConfigurationError::RateLimitExceeded(err.msg),
                    )
                }
                "ServerInternalException" => {
                    return RusotoError::Service(
                        ListAssociationsForLicenseConfigurationError::ServerInternal(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListAssociationsForLicenseConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAssociationsForLicenseConfigurationError {
    fn description(&self) -> &str {
        match *self {
            ListAssociationsForLicenseConfigurationError::AccessDenied(ref cause) => cause,
            ListAssociationsForLicenseConfigurationError::Authorization(ref cause) => cause,
            ListAssociationsForLicenseConfigurationError::FilterLimitExceeded(ref cause) => cause,
            ListAssociationsForLicenseConfigurationError::InvalidParameterValue(ref cause) => cause,
            ListAssociationsForLicenseConfigurationError::RateLimitExceeded(ref cause) => cause,
            ListAssociationsForLicenseConfigurationError::ServerInternal(ref cause) => cause,
        }
    }
}
/// Errors returned by ListFailuresForLicenseConfigurationOperations
#[derive(Debug, PartialEq)]
pub enum ListFailuresForLicenseConfigurationOperationsError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValue(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl ListFailuresForLicenseConfigurationOperationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListFailuresForLicenseConfigurationOperationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        ListFailuresForLicenseConfigurationOperationsError::AccessDenied(err.msg),
                    )
                }
                "AuthorizationException" => {
                    return RusotoError::Service(
                        ListFailuresForLicenseConfigurationOperationsError::Authorization(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        ListFailuresForLicenseConfigurationOperationsError::InvalidParameterValue(
                            err.msg,
                        ),
                    )
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(
                        ListFailuresForLicenseConfigurationOperationsError::RateLimitExceeded(
                            err.msg,
                        ),
                    )
                }
                "ServerInternalException" => {
                    return RusotoError::Service(
                        ListFailuresForLicenseConfigurationOperationsError::ServerInternal(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListFailuresForLicenseConfigurationOperationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListFailuresForLicenseConfigurationOperationsError {
    fn description(&self) -> &str {
        match *self {
            ListFailuresForLicenseConfigurationOperationsError::AccessDenied(ref cause) => cause,
            ListFailuresForLicenseConfigurationOperationsError::Authorization(ref cause) => cause,
            ListFailuresForLicenseConfigurationOperationsError::InvalidParameterValue(
                ref cause,
            ) => cause,
            ListFailuresForLicenseConfigurationOperationsError::RateLimitExceeded(ref cause) => {
                cause
            }
            ListFailuresForLicenseConfigurationOperationsError::ServerInternal(ref cause) => cause,
        }
    }
}
/// Errors returned by ListLicenseConfigurations
#[derive(Debug, PartialEq)]
pub enum ListLicenseConfigurationsError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>The request uses too many filters or too many filter values.</p>
    FilterLimitExceeded(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValue(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl ListLicenseConfigurationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListLicenseConfigurationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListLicenseConfigurationsError::AccessDenied(
                        err.msg,
                    ))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(ListLicenseConfigurationsError::Authorization(
                        err.msg,
                    ))
                }
                "FilterLimitExceededException" => {
                    return RusotoError::Service(
                        ListLicenseConfigurationsError::FilterLimitExceeded(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        ListLicenseConfigurationsError::InvalidParameterValue(err.msg),
                    )
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(ListLicenseConfigurationsError::RateLimitExceeded(
                        err.msg,
                    ))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(ListLicenseConfigurationsError::ServerInternal(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListLicenseConfigurationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListLicenseConfigurationsError {
    fn description(&self) -> &str {
        match *self {
            ListLicenseConfigurationsError::AccessDenied(ref cause) => cause,
            ListLicenseConfigurationsError::Authorization(ref cause) => cause,
            ListLicenseConfigurationsError::FilterLimitExceeded(ref cause) => cause,
            ListLicenseConfigurationsError::InvalidParameterValue(ref cause) => cause,
            ListLicenseConfigurationsError::RateLimitExceeded(ref cause) => cause,
            ListLicenseConfigurationsError::ServerInternal(ref cause) => cause,
        }
    }
}
/// Errors returned by ListLicenseSpecificationsForResource
#[derive(Debug, PartialEq)]
pub enum ListLicenseSpecificationsForResourceError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValue(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl ListLicenseSpecificationsForResourceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListLicenseSpecificationsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        ListLicenseSpecificationsForResourceError::AccessDenied(err.msg),
                    )
                }
                "AuthorizationException" => {
                    return RusotoError::Service(
                        ListLicenseSpecificationsForResourceError::Authorization(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        ListLicenseSpecificationsForResourceError::InvalidParameterValue(err.msg),
                    )
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(
                        ListLicenseSpecificationsForResourceError::RateLimitExceeded(err.msg),
                    )
                }
                "ServerInternalException" => {
                    return RusotoError::Service(
                        ListLicenseSpecificationsForResourceError::ServerInternal(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListLicenseSpecificationsForResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListLicenseSpecificationsForResourceError {
    fn description(&self) -> &str {
        match *self {
            ListLicenseSpecificationsForResourceError::AccessDenied(ref cause) => cause,
            ListLicenseSpecificationsForResourceError::Authorization(ref cause) => cause,
            ListLicenseSpecificationsForResourceError::InvalidParameterValue(ref cause) => cause,
            ListLicenseSpecificationsForResourceError::RateLimitExceeded(ref cause) => cause,
            ListLicenseSpecificationsForResourceError::ServerInternal(ref cause) => cause,
        }
    }
}
/// Errors returned by ListResourceInventory
#[derive(Debug, PartialEq)]
pub enum ListResourceInventoryError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>A dependency required to run the API is missing.</p>
    FailedDependency(String),
    /// <p>The request uses too many filters or too many filter values.</p>
    FilterLimitExceeded(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValue(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl ListResourceInventoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListResourceInventoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListResourceInventoryError::AccessDenied(err.msg))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(ListResourceInventoryError::Authorization(err.msg))
                }
                "FailedDependencyException" => {
                    return RusotoError::Service(ListResourceInventoryError::FailedDependency(
                        err.msg,
                    ))
                }
                "FilterLimitExceededException" => {
                    return RusotoError::Service(ListResourceInventoryError::FilterLimitExceeded(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListResourceInventoryError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(ListResourceInventoryError::RateLimitExceeded(
                        err.msg,
                    ))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(ListResourceInventoryError::ServerInternal(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListResourceInventoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListResourceInventoryError {
    fn description(&self) -> &str {
        match *self {
            ListResourceInventoryError::AccessDenied(ref cause) => cause,
            ListResourceInventoryError::Authorization(ref cause) => cause,
            ListResourceInventoryError::FailedDependency(ref cause) => cause,
            ListResourceInventoryError::FilterLimitExceeded(ref cause) => cause,
            ListResourceInventoryError::InvalidParameterValue(ref cause) => cause,
            ListResourceInventoryError::RateLimitExceeded(ref cause) => cause,
            ListResourceInventoryError::ServerInternal(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValue(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListTagsForResourceError::AccessDenied(err.msg))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(ListTagsForResourceError::Authorization(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(ListTagsForResourceError::RateLimitExceeded(
                        err.msg,
                    ))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(ListTagsForResourceError::ServerInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListTagsForResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsForResourceError {
    fn description(&self) -> &str {
        match *self {
            ListTagsForResourceError::AccessDenied(ref cause) => cause,
            ListTagsForResourceError::Authorization(ref cause) => cause,
            ListTagsForResourceError::InvalidParameterValue(ref cause) => cause,
            ListTagsForResourceError::RateLimitExceeded(ref cause) => cause,
            ListTagsForResourceError::ServerInternal(ref cause) => cause,
        }
    }
}
/// Errors returned by ListUsageForLicenseConfiguration
#[derive(Debug, PartialEq)]
pub enum ListUsageForLicenseConfigurationError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>The request uses too many filters or too many filter values.</p>
    FilterLimitExceeded(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValue(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl ListUsageForLicenseConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListUsageForLicenseConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        ListUsageForLicenseConfigurationError::AccessDenied(err.msg),
                    )
                }
                "AuthorizationException" => {
                    return RusotoError::Service(
                        ListUsageForLicenseConfigurationError::Authorization(err.msg),
                    )
                }
                "FilterLimitExceededException" => {
                    return RusotoError::Service(
                        ListUsageForLicenseConfigurationError::FilterLimitExceeded(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        ListUsageForLicenseConfigurationError::InvalidParameterValue(err.msg),
                    )
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(
                        ListUsageForLicenseConfigurationError::RateLimitExceeded(err.msg),
                    )
                }
                "ServerInternalException" => {
                    return RusotoError::Service(
                        ListUsageForLicenseConfigurationError::ServerInternal(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListUsageForLicenseConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListUsageForLicenseConfigurationError {
    fn description(&self) -> &str {
        match *self {
            ListUsageForLicenseConfigurationError::AccessDenied(ref cause) => cause,
            ListUsageForLicenseConfigurationError::Authorization(ref cause) => cause,
            ListUsageForLicenseConfigurationError::FilterLimitExceeded(ref cause) => cause,
            ListUsageForLicenseConfigurationError::InvalidParameterValue(ref cause) => cause,
            ListUsageForLicenseConfigurationError::RateLimitExceeded(ref cause) => cause,
            ListUsageForLicenseConfigurationError::ServerInternal(ref cause) => cause,
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValue(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(TagResourceError::AccessDenied(err.msg))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(TagResourceError::Authorization(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(TagResourceError::InvalidParameterValue(err.msg))
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(TagResourceError::RateLimitExceeded(err.msg))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(TagResourceError::ServerInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for TagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TagResourceError {
    fn description(&self) -> &str {
        match *self {
            TagResourceError::AccessDenied(ref cause) => cause,
            TagResourceError::Authorization(ref cause) => cause,
            TagResourceError::InvalidParameterValue(ref cause) => cause,
            TagResourceError::RateLimitExceeded(ref cause) => cause,
            TagResourceError::ServerInternal(ref cause) => cause,
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValue(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UntagResourceError::AccessDenied(err.msg))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(UntagResourceError::Authorization(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(UntagResourceError::InvalidParameterValue(err.msg))
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(UntagResourceError::RateLimitExceeded(err.msg))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(UntagResourceError::ServerInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UntagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UntagResourceError {
    fn description(&self) -> &str {
        match *self {
            UntagResourceError::AccessDenied(ref cause) => cause,
            UntagResourceError::Authorization(ref cause) => cause,
            UntagResourceError::InvalidParameterValue(ref cause) => cause,
            UntagResourceError::RateLimitExceeded(ref cause) => cause,
            UntagResourceError::ServerInternal(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateLicenseConfiguration
#[derive(Debug, PartialEq)]
pub enum UpdateLicenseConfigurationError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValue(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl UpdateLicenseConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateLicenseConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateLicenseConfigurationError::AccessDenied(
                        err.msg,
                    ))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(UpdateLicenseConfigurationError::Authorization(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        UpdateLicenseConfigurationError::InvalidParameterValue(err.msg),
                    )
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(
                        UpdateLicenseConfigurationError::RateLimitExceeded(err.msg),
                    )
                }
                "ServerInternalException" => {
                    return RusotoError::Service(UpdateLicenseConfigurationError::ServerInternal(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateLicenseConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateLicenseConfigurationError {
    fn description(&self) -> &str {
        match *self {
            UpdateLicenseConfigurationError::AccessDenied(ref cause) => cause,
            UpdateLicenseConfigurationError::Authorization(ref cause) => cause,
            UpdateLicenseConfigurationError::InvalidParameterValue(ref cause) => cause,
            UpdateLicenseConfigurationError::RateLimitExceeded(ref cause) => cause,
            UpdateLicenseConfigurationError::ServerInternal(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateLicenseSpecificationsForResource
#[derive(Debug, PartialEq)]
pub enum UpdateLicenseSpecificationsForResourceError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValue(String),
    /// <p>License Manager cannot allocate a license to a resource because of its state. </p> <p>For example, you cannot allocate a license to an instance in the process of shutting down.</p>
    InvalidResourceState(String),
    /// <p>You do not have enough licenses available to support a new resource launch.</p>
    LicenseUsage(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl UpdateLicenseSpecificationsForResourceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateLicenseSpecificationsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        UpdateLicenseSpecificationsForResourceError::AccessDenied(err.msg),
                    )
                }
                "AuthorizationException" => {
                    return RusotoError::Service(
                        UpdateLicenseSpecificationsForResourceError::Authorization(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        UpdateLicenseSpecificationsForResourceError::InvalidParameterValue(err.msg),
                    )
                }
                "InvalidResourceStateException" => {
                    return RusotoError::Service(
                        UpdateLicenseSpecificationsForResourceError::InvalidResourceState(err.msg),
                    )
                }
                "LicenseUsageException" => {
                    return RusotoError::Service(
                        UpdateLicenseSpecificationsForResourceError::LicenseUsage(err.msg),
                    )
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(
                        UpdateLicenseSpecificationsForResourceError::RateLimitExceeded(err.msg),
                    )
                }
                "ServerInternalException" => {
                    return RusotoError::Service(
                        UpdateLicenseSpecificationsForResourceError::ServerInternal(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateLicenseSpecificationsForResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateLicenseSpecificationsForResourceError {
    fn description(&self) -> &str {
        match *self {
            UpdateLicenseSpecificationsForResourceError::AccessDenied(ref cause) => cause,
            UpdateLicenseSpecificationsForResourceError::Authorization(ref cause) => cause,
            UpdateLicenseSpecificationsForResourceError::InvalidParameterValue(ref cause) => cause,
            UpdateLicenseSpecificationsForResourceError::InvalidResourceState(ref cause) => cause,
            UpdateLicenseSpecificationsForResourceError::LicenseUsage(ref cause) => cause,
            UpdateLicenseSpecificationsForResourceError::RateLimitExceeded(ref cause) => cause,
            UpdateLicenseSpecificationsForResourceError::ServerInternal(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateServiceSettings
#[derive(Debug, PartialEq)]
pub enum UpdateServiceSettingsError {
    /// <p>Access to resource denied.</p>
    AccessDenied(String),
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    Authorization(String),
    /// <p>One or more parameter values are not valid.</p>
    InvalidParameterValue(String),
    /// <p>Too many requests have been submitted. Try again after a brief wait.</p>
    RateLimitExceeded(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternal(String),
}

impl UpdateServiceSettingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateServiceSettingsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateServiceSettingsError::AccessDenied(err.msg))
                }
                "AuthorizationException" => {
                    return RusotoError::Service(UpdateServiceSettingsError::Authorization(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(UpdateServiceSettingsError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "RateLimitExceededException" => {
                    return RusotoError::Service(UpdateServiceSettingsError::RateLimitExceeded(
                        err.msg,
                    ))
                }
                "ServerInternalException" => {
                    return RusotoError::Service(UpdateServiceSettingsError::ServerInternal(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateServiceSettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateServiceSettingsError {
    fn description(&self) -> &str {
        match *self {
            UpdateServiceSettingsError::AccessDenied(ref cause) => cause,
            UpdateServiceSettingsError::Authorization(ref cause) => cause,
            UpdateServiceSettingsError::InvalidParameterValue(ref cause) => cause,
            UpdateServiceSettingsError::RateLimitExceeded(ref cause) => cause,
            UpdateServiceSettingsError::ServerInternal(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS License Manager API. AWS License Manager clients implement this trait.
pub trait LicenseManager {
    /// <p>Creates a license configuration.</p> <p>A license configuration is an abstraction of a customer license agreement that can be consumed and enforced by License Manager. Components include specifications for the license type (licensing by instance, socket, CPU, or vCPU), allowed tenancy (shared tenancy, Dedicated Instance, Dedicated Host, or all of these), host affinity (how long a VM must be associated with a host), and the number of licenses purchased and used.</p>
    fn create_license_configuration(
        &self,
        input: CreateLicenseConfigurationRequest,
    ) -> RusotoFuture<CreateLicenseConfigurationResponse, CreateLicenseConfigurationError>;

    /// <p>Deletes the specified license configuration.</p> <p>You cannot delete a license configuration that is in use.</p>
    fn delete_license_configuration(
        &self,
        input: DeleteLicenseConfigurationRequest,
    ) -> RusotoFuture<DeleteLicenseConfigurationResponse, DeleteLicenseConfigurationError>;

    /// <p>Gets detailed information about the specified license configuration.</p>
    fn get_license_configuration(
        &self,
        input: GetLicenseConfigurationRequest,
    ) -> RusotoFuture<GetLicenseConfigurationResponse, GetLicenseConfigurationError>;

    /// <p>Gets the License Manager settings for the current Region.</p>
    fn get_service_settings(
        &self,
    ) -> RusotoFuture<GetServiceSettingsResponse, GetServiceSettingsError>;

    /// <p>Lists the resource associations for the specified license configuration.</p> <p>Resource associations need not consume licenses from a license configuration. For example, an AMI or a stopped instance might not consume a license (depending on the license rules).</p>
    fn list_associations_for_license_configuration(
        &self,
        input: ListAssociationsForLicenseConfigurationRequest,
    ) -> RusotoFuture<
        ListAssociationsForLicenseConfigurationResponse,
        ListAssociationsForLicenseConfigurationError,
    >;

    /// <p>Lists the license configuration operations that failed.</p>
    fn list_failures_for_license_configuration_operations(
        &self,
        input: ListFailuresForLicenseConfigurationOperationsRequest,
    ) -> RusotoFuture<
        ListFailuresForLicenseConfigurationOperationsResponse,
        ListFailuresForLicenseConfigurationOperationsError,
    >;

    /// <p>Lists the license configurations for your account.</p>
    fn list_license_configurations(
        &self,
        input: ListLicenseConfigurationsRequest,
    ) -> RusotoFuture<ListLicenseConfigurationsResponse, ListLicenseConfigurationsError>;

    /// <p>Describes the license configurations for the specified resource.</p>
    fn list_license_specifications_for_resource(
        &self,
        input: ListLicenseSpecificationsForResourceRequest,
    ) -> RusotoFuture<
        ListLicenseSpecificationsForResourceResponse,
        ListLicenseSpecificationsForResourceError,
    >;

    /// <p>Lists resources managed using Systems Manager inventory.</p>
    fn list_resource_inventory(
        &self,
        input: ListResourceInventoryRequest,
    ) -> RusotoFuture<ListResourceInventoryResponse, ListResourceInventoryError>;

    /// <p>Lists the tags for the specified license configuration.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError>;

    /// <p>Lists all license usage records for a license configuration, displaying license consumption details by resource at a selected point in time. Use this action to audit the current license consumption for any license inventory and configuration.</p>
    fn list_usage_for_license_configuration(
        &self,
        input: ListUsageForLicenseConfigurationRequest,
    ) -> RusotoFuture<ListUsageForLicenseConfigurationResponse, ListUsageForLicenseConfigurationError>;

    /// <p>Adds the specified tags to the specified license configuration.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError>;

    /// <p>Removes the specified tags from the specified license configuration.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError>;

    /// <p>Modifies the attributes of an existing license configuration.</p> <p>A license configuration is an abstraction of a customer license agreement that can be consumed and enforced by License Manager. Components include specifications for the license type (licensing by instance, socket, CPU, or vCPU), allowed tenancy (shared tenancy, Dedicated Instance, Dedicated Host, or all of these), host affinity (how long a VM must be associated with a host), and the number of licenses purchased and used.</p>
    fn update_license_configuration(
        &self,
        input: UpdateLicenseConfigurationRequest,
    ) -> RusotoFuture<UpdateLicenseConfigurationResponse, UpdateLicenseConfigurationError>;

    /// <p>Adds or removes the specified license configurations for the specified AWS resource.</p> <p>You can update the license specifications of AMIs, instances, and hosts. You cannot update the license specifications for launch templates and AWS CloudFormation templates, as they send license configurations to the operation that creates the resource.</p>
    fn update_license_specifications_for_resource(
        &self,
        input: UpdateLicenseSpecificationsForResourceRequest,
    ) -> RusotoFuture<
        UpdateLicenseSpecificationsForResourceResponse,
        UpdateLicenseSpecificationsForResourceError,
    >;

    /// <p>Updates License Manager settings for the current Region.</p>
    fn update_service_settings(
        &self,
        input: UpdateServiceSettingsRequest,
    ) -> RusotoFuture<UpdateServiceSettingsResponse, UpdateServiceSettingsError>;
}
/// A client for the AWS License Manager API.
#[derive(Clone)]
pub struct LicenseManagerClient {
    client: Client,
    region: region::Region,
}

impl LicenseManagerClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> LicenseManagerClient {
        Self::new_with_client(Client::shared(), region)
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> LicenseManagerClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        Self::new_with_client(
            Client::new_with(credentials_provider, request_dispatcher),
            region,
        )
    }

    pub fn new_with_client(client: Client, region: region::Region) -> LicenseManagerClient {
        LicenseManagerClient { client, region }
    }
}

impl fmt::Debug for LicenseManagerClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LicenseManagerClient")
            .field("region", &self.region)
            .finish()
    }
}

impl LicenseManager for LicenseManagerClient {
    /// <p>Creates a license configuration.</p> <p>A license configuration is an abstraction of a customer license agreement that can be consumed and enforced by License Manager. Components include specifications for the license type (licensing by instance, socket, CPU, or vCPU), allowed tenancy (shared tenancy, Dedicated Instance, Dedicated Host, or all of these), host affinity (how long a VM must be associated with a host), and the number of licenses purchased and used.</p>
    fn create_license_configuration(
        &self,
        input: CreateLicenseConfigurationRequest,
    ) -> RusotoFuture<CreateLicenseConfigurationResponse, CreateLicenseConfigurationError> {
        let mut request = SignedRequest::new("POST", "license-manager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSLicenseManager.CreateLicenseConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateLicenseConfigurationResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateLicenseConfigurationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes the specified license configuration.</p> <p>You cannot delete a license configuration that is in use.</p>
    fn delete_license_configuration(
        &self,
        input: DeleteLicenseConfigurationRequest,
    ) -> RusotoFuture<DeleteLicenseConfigurationResponse, DeleteLicenseConfigurationError> {
        let mut request = SignedRequest::new("POST", "license-manager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSLicenseManager.DeleteLicenseConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteLicenseConfigurationResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteLicenseConfigurationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Gets detailed information about the specified license configuration.</p>
    fn get_license_configuration(
        &self,
        input: GetLicenseConfigurationRequest,
    ) -> RusotoFuture<GetLicenseConfigurationResponse, GetLicenseConfigurationError> {
        let mut request = SignedRequest::new("POST", "license-manager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSLicenseManager.GetLicenseConfiguration");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetLicenseConfigurationResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetLicenseConfigurationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Gets the License Manager settings for the current Region.</p>
    fn get_service_settings(
        &self,
    ) -> RusotoFuture<GetServiceSettingsResponse, GetServiceSettingsError> {
        let mut request = SignedRequest::new("POST", "license-manager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSLicenseManager.GetServiceSettings");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetServiceSettingsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetServiceSettingsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the resource associations for the specified license configuration.</p> <p>Resource associations need not consume licenses from a license configuration. For example, an AMI or a stopped instance might not consume a license (depending on the license rules).</p>
    fn list_associations_for_license_configuration(
        &self,
        input: ListAssociationsForLicenseConfigurationRequest,
    ) -> RusotoFuture<
        ListAssociationsForLicenseConfigurationResponse,
        ListAssociationsForLicenseConfigurationError,
    > {
        let mut request = SignedRequest::new("POST", "license-manager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSLicenseManager.ListAssociationsForLicenseConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListAssociationsForLicenseConfigurationResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListAssociationsForLicenseConfigurationError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Lists the license configuration operations that failed.</p>
    fn list_failures_for_license_configuration_operations(
        &self,
        input: ListFailuresForLicenseConfigurationOperationsRequest,
    ) -> RusotoFuture<
        ListFailuresForLicenseConfigurationOperationsResponse,
        ListFailuresForLicenseConfigurationOperationsError,
    > {
        let mut request = SignedRequest::new("POST", "license-manager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSLicenseManager.ListFailuresForLicenseConfigurationOperations",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
                        if response.status.is_success() {
                            Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<ListFailuresForLicenseConfigurationOperationsResponse, _>()
                }))
                        } else {
                            Box::new(response.buffer().from_err().and_then(|response| {
                                Err(ListFailuresForLicenseConfigurationOperationsError::from_response(response))
                            }))
                        }
                    })
    }

    /// <p>Lists the license configurations for your account.</p>
    fn list_license_configurations(
        &self,
        input: ListLicenseConfigurationsRequest,
    ) -> RusotoFuture<ListLicenseConfigurationsResponse, ListLicenseConfigurationsError> {
        let mut request = SignedRequest::new("POST", "license-manager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSLicenseManager.ListLicenseConfigurations",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListLicenseConfigurationsResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListLicenseConfigurationsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Describes the license configurations for the specified resource.</p>
    fn list_license_specifications_for_resource(
        &self,
        input: ListLicenseSpecificationsForResourceRequest,
    ) -> RusotoFuture<
        ListLicenseSpecificationsForResourceResponse,
        ListLicenseSpecificationsForResourceError,
    > {
        let mut request = SignedRequest::new("POST", "license-manager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSLicenseManager.ListLicenseSpecificationsForResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListLicenseSpecificationsForResourceResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListLicenseSpecificationsForResourceError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Lists resources managed using Systems Manager inventory.</p>
    fn list_resource_inventory(
        &self,
        input: ListResourceInventoryRequest,
    ) -> RusotoFuture<ListResourceInventoryResponse, ListResourceInventoryError> {
        let mut request = SignedRequest::new("POST", "license-manager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSLicenseManager.ListResourceInventory");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListResourceInventoryResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListResourceInventoryError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Lists the tags for the specified license configuration.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError> {
        let mut request = SignedRequest::new("POST", "license-manager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSLicenseManager.ListTagsForResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListTagsForResourceResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListTagsForResourceError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Lists all license usage records for a license configuration, displaying license consumption details by resource at a selected point in time. Use this action to audit the current license consumption for any license inventory and configuration.</p>
    fn list_usage_for_license_configuration(
        &self,
        input: ListUsageForLicenseConfigurationRequest,
    ) -> RusotoFuture<ListUsageForLicenseConfigurationResponse, ListUsageForLicenseConfigurationError>
    {
        let mut request = SignedRequest::new("POST", "license-manager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSLicenseManager.ListUsageForLicenseConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListUsageForLicenseConfigurationResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListUsageForLicenseConfigurationError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Adds the specified tags to the specified license configuration.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError> {
        let mut request = SignedRequest::new("POST", "license-manager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSLicenseManager.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<TagResourceResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(TagResourceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Removes the specified tags from the specified license configuration.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError> {
        let mut request = SignedRequest::new("POST", "license-manager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSLicenseManager.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UntagResourceResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UntagResourceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Modifies the attributes of an existing license configuration.</p> <p>A license configuration is an abstraction of a customer license agreement that can be consumed and enforced by License Manager. Components include specifications for the license type (licensing by instance, socket, CPU, or vCPU), allowed tenancy (shared tenancy, Dedicated Instance, Dedicated Host, or all of these), host affinity (how long a VM must be associated with a host), and the number of licenses purchased and used.</p>
    fn update_license_configuration(
        &self,
        input: UpdateLicenseConfigurationRequest,
    ) -> RusotoFuture<UpdateLicenseConfigurationResponse, UpdateLicenseConfigurationError> {
        let mut request = SignedRequest::new("POST", "license-manager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSLicenseManager.UpdateLicenseConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateLicenseConfigurationResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateLicenseConfigurationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Adds or removes the specified license configurations for the specified AWS resource.</p> <p>You can update the license specifications of AMIs, instances, and hosts. You cannot update the license specifications for launch templates and AWS CloudFormation templates, as they send license configurations to the operation that creates the resource.</p>
    fn update_license_specifications_for_resource(
        &self,
        input: UpdateLicenseSpecificationsForResourceRequest,
    ) -> RusotoFuture<
        UpdateLicenseSpecificationsForResourceResponse,
        UpdateLicenseSpecificationsForResourceError,
    > {
        let mut request = SignedRequest::new("POST", "license-manager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSLicenseManager.UpdateLicenseSpecificationsForResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateLicenseSpecificationsForResourceResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateLicenseSpecificationsForResourceError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Updates License Manager settings for the current Region.</p>
    fn update_service_settings(
        &self,
        input: UpdateServiceSettingsRequest,
    ) -> RusotoFuture<UpdateServiceSettingsResponse, UpdateServiceSettingsError> {
        let mut request = SignedRequest::new("POST", "license-manager", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSLicenseManager.UpdateServiceSettings");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateServiceSettingsResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateServiceSettingsError::from_response(response))
                    }),
                )
            }
        })
    }
}
