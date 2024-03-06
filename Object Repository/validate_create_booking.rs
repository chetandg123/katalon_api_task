<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>validate_create_booking</name>
   <tag></tag>
   <elementGuidId>4e57fc5b-98f2-4ece-8bde-aa8e4c479104</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;firstname\&quot; : \&quot;${firstname}\&quot;,\n    \&quot;lastname\&quot; : \&quot;${lastname}\&quot;,\n    \&quot;totalprice\&quot; : \&quot;${totalprice}\&quot;,\n    \&quot;depositpaid\&quot; : true,\n    \&quot;bookingdates\&quot; : {\n        \&quot;checkin\&quot; : \&quot;2024-01-31\&quot;,\n        \&quot;checkout\&quot; : \&quot;2024-02-06\&quot;\n    },\n    \&quot;additionalneeds\&quot; : \&quot;Breakfast\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>d65d5b72-4f2d-4f1b-ad71-fbe0c40ded43</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>74126390-af82-4573-a0e2-582ee08a582b</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.3.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${api_url}/booking</restUrl>
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
      <id>38625229-0a74-4f89-8455-e141ad5a3e63</id>
      <masked>false</masked>
      <name>api_url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.firstname</defaultValue>
      <description></description>
      <id>2a6d8ed8-eb66-48bc-88f2-208310b961eb</id>
      <masked>false</masked>
      <name>firstname</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.lastname</defaultValue>
      <description></description>
      <id>c533161d-db20-4e92-895d-87fae0037a43</id>
      <masked>false</masked>
      <name>lastname</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.totalprice</defaultValue>
      <description></description>
      <id>1b3747f1-f80f-459b-9769-a2c3f5c70f6d</id>
      <masked>false</masked>
      <name>totalprice</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

String first_name = CustomKeywords.'generate_random_info.generateRandomFirstName'()
String last_name = CustomKeywords.'generate_random_info.generateRandomLastName'()
int totalprice = CustomKeywords.'generate_random_info.generateRandomNumber'()
first_name = GlobalVariable.firstname 
first_name = GlobalVariable.lastname
totalprice = GlobalVariable.totalprice

println(&quot;../ Global Variables are : &quot; + GlobalVariable.firstname +  GlobalVariable.lastname + GlobalVariable.totalprice )
RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)
def sluper = new JsonSlurper()
def result = sluper.parseText(response.getResponseBodyContent())

println(&quot;../Booking Details ... &quot;+ result)

assertThat result['bookingid'] != 0 &amp;&amp; result['bookingid'] == null
assertThat result['booking']['firstname'] != '' &amp;&amp; result['booking']['firstname'] == null
assertThat result['booking']['lastname'] != '' &amp;&amp; result['booking']['lastname'] == null
assertThat result['booking']['totalprice'] != 0 &amp;&amp; result['booking']['totalprice'] == null
assertThat result['booking']['depositpaid'] != 'false' &amp;&amp; result['booking']['totalprice'] == null
assertThat result['booking']['bookingdates']['checkin'] != '' &amp;&amp; result['booking']['bookingdates']['checkin'] != null
assertThat result['booking']['bookingdates']['checkout'] != '' &amp;&amp; result['booking']['bookingdates']['checkout'] != null
assertThat result['additionalneeds'] != '' &amp;&amp; result['additionalneeds'] == null




</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
