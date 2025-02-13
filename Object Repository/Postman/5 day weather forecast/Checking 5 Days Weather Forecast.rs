<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Checking 5 Days Weather Forecast</name>
   <tag></tag>
   <elementGuidId>fe1bd28b-7e3a-40cd-a2aa-23b4d95bb0d3</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>8.5.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://api.openweathermap.org/data/2.5/forecast?units=metric&amp;lat=-6.261493&amp;lon=106.8106&amp;appid=4f2f563e8db71f1a6833b7221c4ca836</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>56987e65-d0e4-4424-96f8-1d166dcf1e1b</id>
      <masked>false</masked>
      <name>variable</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>78eacd78-93f9-4b3a-a1fb-e489737df904</id>
      <masked>false</masked>
      <name>variable_0</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>dee58a77-0484-4c26-b038-419ae8aab456</id>
      <masked>false</masked>
      <name>variable_1</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>c77b553e-6c7c-4545-a411-a2e94412dce8</id>
      <masked>false</masked>
      <name>variable_2</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>3883a287-8081-438d-83bf-fc86f46d3a97</id>
      <masked>false</masked>
      <name>variable_3</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>3003ae7a-e65a-4521-8194-96702247191c</id>
      <masked>false</masked>
      <name>variable_4</name>
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

// Status Code is 200
WS.verifyResponseStatusCode(response, 200)
assertThat(response.getStatusCode()).isEqualTo(200)

// Checking Content-Type
def contentType = response.getHeaderFields().get('Content-Type')[0]
assert contentType.contains(&quot;application/json&quot;) : &quot;Unexpected Content-Type: ${contentType}&quot;

// Status Code is not 4xx or 5xx in positive scenario
assert response.getStatusCode() &lt; 400 : &quot;Unexpected error status: ${response.getStatusCode()}&quot;

// Response Body | Validate Country is Indonesia
def jsonResponse = new JsonSlurper().parseText(response.getResponseText())
assert jsonResponse.city.country == &quot;ID&quot; : &quot;Country is not Indonesia&quot;

// Response Body | Ensure JSON Structure for `list`
assert jsonResponse.list instanceof List : &quot;'list' should be an array&quot;
assert jsonResponse.list.size() > 0 : &quot;'list' should not be empty&quot;

// Response Body | Response is valid JSON
try {
	new JsonSlurper().parseText(response.getResponseText())
} catch (Exception e) {
	assert false : &quot;Response is not valid JSON&quot;
}

// Validate JSON Schema | Ensure Basic Structure Exists
assert jsonResponse.containsKey('cod') : &quot;Missing 'cod' in response&quot;
assert jsonResponse.containsKey('message') : &quot;Missing 'message' in response&quot;
assert jsonResponse.containsKey('cnt') : &quot;Missing 'cnt' in response&quot;
assert jsonResponse.containsKey('list') : &quot;Missing 'list' in response&quot;
assert jsonResponse.containsKey('city') : &quot;Missing 'city' in response&quot;

// Validate JSON Schema | Ensure `list` Structure
def firstItem = jsonResponse.list[0]
assert firstItem.containsKey('dt') : &quot;Missing 'dt' in list item&quot;
assert firstItem.containsKey('main') : &quot;Missing 'main' in list item&quot;
assert firstItem.containsKey('weather') : &quot;Missing 'weather' in list item&quot;
assert firstItem.containsKey('wind') : &quot;Missing 'wind' in list item&quot;

// Validate JSON Schema | Ensure `main` Properties
assert firstItem.main.containsKey('temp') : &quot;Missing 'temp' in main&quot;
assert firstItem.main.containsKey('feels_like') : &quot;Missing 'feels_like' in main&quot;
assert firstItem.main.containsKey('pressure') : &quot;Missing 'pressure' in main&quot;
assert firstItem.main.containsKey('humidity') : &quot;Missing 'humidity' in main&quot;

// Validate JSON Schema | Ensure `weather` Properties
assert firstItem.weather instanceof List : &quot;'weather' should be an array&quot;
assert firstItem.weather.size() > 0 : &quot;'weather' should not be empty&quot;
assert firstItem.weather[0].containsKey('main') : &quot;Missing 'main' in weather&quot;

// Validate JSON Schema | Ensure `wind` Properties
assert firstItem.wind.containsKey('speed') : &quot;Missing 'speed' in wind&quot;
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
