<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>validate_booking_info_by_id</name>
   <tag></tag>
   <elementGuidId>e67708b1-725a-4ecc-8048-9964f304635d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>013b646a-0d3d-449d-8bae-45462df38b8e</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.3.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${api_url}/booking/${id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.api_url</defaultValue>
      <description></description>
      <id>198115aa-b1a8-48e2-8f5c-b78e464f138a</id>
      <masked>false</masked>
      <name>api_url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.id</defaultValue>
      <description></description>
      <id>b82c4ef2-93c5-40bb-bf22-cde2206a7557</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

def sluper = new JsonSlurper()
def result = sluper.parseText(response.getResponseBodyContent())

assertThat result['firstname'] != '' &amp;&amp; result['firstname'] != null
assertThat result['lastname'] != '' &amp;&amp; result['lastname'] != null
assertThat result['totalprice'] != 0 &amp;&amp; result['firstname'] != null
assertThat result['firstname'] != '' &amp;&amp; result['firstname'] != null
assertThat result['bookingdates']['checkin'] != '' &amp;&amp; result['bookingdates']['checkin'] != null
assertThat result['bookingdates']['checkout'] != '' &amp;&amp; result['bookingdates']['checkout'] != null




</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
