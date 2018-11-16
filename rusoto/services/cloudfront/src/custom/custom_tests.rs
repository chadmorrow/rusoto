extern crate rusoto_mock;

use ::{CloudFront, CloudFrontClient, ListDistributionsRequest};

use rusoto_core::Region;
use self::rusoto_mock::*;

#[test]
fn should_list_distributions() {
    // reproduced from botocore `cloudfront-list-distributions.xml`
    let mock = MockRequestDispatcher::with_status(200)
        .with_body(r#"<?xml version="1.0" encoding="UTF-8"?>
<DistributionList xmlns="http://cloudfront.amazonaws.com/doc/2014-05-31/">
   <Marker>RMPARXS293KSTG7</Marker>
   <NextMarker>EMLARXS9EXAMPLE</NextMarker>
   <MaxItems>2</MaxItems>
   <IsTruncated>true</IsTruncated>
   <Quantity>1</Quantity>
   <Items>
      <DistributionSummary>
         <Id>EDFDVBD6EXAMPLE</Id>
         <Status>Deployed</Status>
         <LastModifiedTime>2012-05-19T19:37:58Z</LastModifiedTime>
         <DomainName>d111111abcdef8.cloudfront.net</DomainName>
         <Aliases>
            <Quantity>1</Quantity>
            <Items>
               <CNAME>www.example.com</CNAME>
            </Items>
         </Aliases>
         <Origins>
            <Quantity>2</Quantity>
            <Items>
               <Origin>
                  <Id>example-Amazon S3-origin</Id>
                  <DomainName>myawsbucket.s3.amazonaws.com</DomainName>
                  <S3OriginConfig>
                     <OriginAccessIdentity>origin-access-identity/cloudfront/E74FTE3AEXAMPLE</OriginAccessIdentity>
                  </S3OriginConfig>
               </Origin>
               <Origin>
                  <Id>example-custom-origin</Id>
                  <DomainName>example.com</DomainName>
                  <CustomOriginConfig>
                     <HTTPPort>80</HTTPPort>
                     <HTTPSPort>443</HTTPSPort>
                     <OriginProtocolPolicy>match-viewer</OriginProtocolPolicy>
                  </CustomOriginConfig>
               </Origin>
            </Items>
         </Origins>
         <DefaultCacheBehavior>
            <TargetOriginId>example-Amazon S3-origin</TargetOriginId>
            <ForwardedValues>
               <QueryString>true</QueryString>
               <Cookies>
                  <Forward>whitelist</Forward>
                  <WhitelistedNames>
                     <Quantity>1</Quantity>
                     <Items>
                        <Name>example-cookie</Name>
                     </Items>
                  </WhitelistedNames>
               </Cookies>
            </ForwardedValues>
            <TrustedSigners>
               <Enabled>true</Enabled>
               <Quantity>3</Quantity>
               <Items>
                  <AwsAccountNumber>self</AwsAccountNumber>
                  <AwsAccountNumber>111122223333</AwsAccountNumber>
                  <AwsAccountNumber>444455556666</AwsAccountNumber>
               </Items>
            </TrustedSigners>
            <ViewerProtocolPolicy>https-only</ViewerProtocolPolicy>
            <MinTTL>0</MinTTL>
         </DefaultCacheBehavior>
         <CacheBehaviors>
            <Quantity>1</Quantity>
            <Items>
               <CacheBehavior>
                  <PathPattern>*.jpg</PathPattern>
                  <TargetOriginId>example-custom-origin</TargetOriginId>
                  <ForwardedValues>
                     <QueryString>false</QueryString>
                     <Cookies>
                        <Forward>all</Forward>
                     </Cookies>
                  </ForwardedValues>
                  <TrustedSigners>
                     <Enabled>true</Enabled>
                     <Quantity>2</Quantity>
                     <Items>
                        <AwsAccountNumber>self</AwsAccountNumber>
                        <AwsAccountNumber>111122223333</AwsAccountNumber>
                     </Items>
                  </TrustedSigners>
                  <ViewerProtocolPolicy>allow-all</ViewerProtocolPolicy>
                  <MinTTL>86400</MinTTL>
               </CacheBehavior>
            </Items>
         </CacheBehaviors>
         <Comment>example comment</Comment>
         <Logging>
            <Enabled>true</Enabled>
            <IncludeCookies>true</IncludeCookies>
            <Bucket>myawslogbucket.s3.amazonaws.com</Bucket>
            <Prefix>example.com.</Prefix>
         </Logging>
         <ViewerCertificate>
            <IAMCertificateId>AS1A2M3P4L5E67SIIXR3J</IAMCertificateId>
         </ViewerCertificate>
         <PriceClass>PriceClass_All</PriceClass>
         <Enabled>true</Enabled>
      </DistributionSummary>
   </Items>
</DistributionList>"#);

    let request = ListDistributionsRequest::default();

    let client = CloudFrontClient::new_with(mock, MockCredentialsProvider, Region::UsEast1);
    let result = client.list_distributions(request).sync().unwrap();
    assert!(&result.distribution_list.is_some());
    let parsed_result = result.distribution_list.unwrap();
    assert!(&parsed_result.items.is_some());
    assert_eq!(parsed_result.items.unwrap().len(), 1);
}
