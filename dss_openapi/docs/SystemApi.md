# \SystemApi

All URIs are relative to *https://dss.acme.com:8080/json*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_dsid**](SystemApi.md#get_dsid) | **GET** /system/getDSID | Returns the dSUID and dSID of the digitalSTROM Server.
[**time**](SystemApi.md#time) | **GET** /system/time | Gets the installation time
[**version**](SystemApi.md#version) | **GET** /system/version | Gets the server version information



## get_dsid

> crate::models::GetDsid200Response get_dsid()
Returns the dSUID and dSID of the digitalSTROM Server.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetDsid200Response**](getDSID_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## time

> crate::models::Time200Response time()
Gets the installation time

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Time200Response**](time_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## version

> crate::models::Version200Response version()
Gets the server version information

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Version200Response**](version_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

