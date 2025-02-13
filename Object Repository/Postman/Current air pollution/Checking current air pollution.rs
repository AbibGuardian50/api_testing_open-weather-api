<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Checking current air pollution</name>
   <tag></tag>
   <elementGuidId>71db5784-5d1f-4137-a35e-edd6dc9b3076</elementGuidId>
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
   <restUrl>http://api.openweathermap.org/data/2.5/air_pollution?lat=-6.261493&amp;lon=106.8106&amp;appid=4f2f563e8db71f1a6833b7221c4ca836</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()
ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

// Status Code Validation
WS.verifyResponseStatusCode(response, 200)
assert response.getStatusCode() == 200 : &quot;Expected status code 200 but got ${response.getStatusCode()}&quot;

// Header Validation
def contentType = response.getHeaderFields().get('Content-Type')[0]
assert contentType.contains(&quot;application/json&quot;) : &quot;Unexpected Content-Type: ${contentType}&quot;

// Response Body | Validate JSON Parsing
def jsonResponse = new JsonSlurper().parseText(response.getResponseText())
assert jsonResponse != null : &quot;Response body is not valid JSON&quot;

// Response Body | Validate JSON Structure
assert jsonResponse.containsKey('coord') : &quot;Missing 'coord' in response&quot;
assert jsonResponse.containsKey('list') : &quot;Missing 'list' in response&quot;

// Response Body | Validate Coordinates
assert jsonResponse.coord.containsKey('lon') : &quot;Missing 'lon' in 'coord'&quot;
assert jsonResponse.coord.containsKey('lat') : &quot;Missing 'lat' in 'coord'&quot;
assert jsonResponse.coord.lon instanceof Number
assert jsonResponse.coord.lat instanceof Number

// Response Body | Validate 'list' is an array and not empty
assert jsonResponse.list instanceof List : &quot;'list' should be an array&quot;
assert jsonResponse.list.size() > 0 : &quot;'list' should not be empty&quot;

// Response Body | Validate AQI range
def aqi = jsonResponse.list[0].main.aqi
assert aqi instanceof Number &amp;&amp; aqi >= 1 &amp;&amp; aqi &lt;= 5 : &quot;AQI out of range: ${aqi}&quot;

// Response Body | Validate Components have all pollutants with numeric values
def components = jsonResponse.list[0].components
assert components.containsKey('co') &amp;&amp; components.co instanceof Number
assert components.containsKey('no') &amp;&amp; components.no instanceof Number
assert components.containsKey('no2') &amp;&amp; components.no2 instanceof Number
assert components.containsKey('o3') &amp;&amp; components.o3 instanceof Number
assert components.containsKey('so2') &amp;&amp; components.so2 instanceof Number
assert components.containsKey('pm2_5') &amp;&amp; components.pm2_5 instanceof Number
assert components.containsKey('pm10') &amp;&amp; components.pm10 instanceof Number
assert components.containsKey('nh3') &amp;&amp; components.nh3 instanceof Number

// JSON Schema | Validate JSON Schema
def expectedSchema = [
    &quot;coord&quot; : [
        &quot;lon&quot; : &quot;number&quot;,
        &quot;lat&quot; : &quot;number&quot;
    ],
    &quot;list&quot; : [
        [
            &quot;main&quot; : [
                &quot;aqi&quot; : &quot;number&quot;
            ],
            &quot;components&quot; : [
                &quot;co&quot; : &quot;number&quot;,
                &quot;no&quot; : &quot;number&quot;,
                &quot;no2&quot; : &quot;number&quot;,
                &quot;o3&quot; : &quot;number&quot;,
                &quot;so2&quot; : &quot;number&quot;,
                &quot;pm2_5&quot; : &quot;number&quot;,
                &quot;pm10&quot; : &quot;number&quot;,
                &quot;nh3&quot; : &quot;number&quot;
            ],
            &quot;dt&quot; : &quot;number&quot;
        ]
    ]
]

// Check if response matches expected schema
assert jsonResponse.coord instanceof Map
assert jsonResponse.coord.lon instanceof Number
assert jsonResponse.coord.lat instanceof Number
assert jsonResponse.list instanceof List
assert jsonResponse.list[0].main.aqi instanceof Number
assert jsonResponse.list[0].components.co instanceof Number
assert jsonResponse.list[0].components.no instanceof Number
assert jsonResponse.list[0].components.no2 instanceof Number
assert jsonResponse.list[0].components.o3 instanceof Number
assert jsonResponse.list[0].components.so2 instanceof Number
assert jsonResponse.list[0].components.pm2_5 instanceof Number
assert jsonResponse.list[0].components.pm10 instanceof Number
assert jsonResponse.list[0].components.nh3 instanceof Number
assert jsonResponse.list[0].dt instanceof Number

// Status Code Validation (Ensure it's not 4xx or 5xx)
assert response.getStatusCode() &lt; 400 : &quot;Unexpected error status: ${response.getStatusCode()}&quot;
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
