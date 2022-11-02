# \PropertyApi

All URIs are relative to *https://dss.int.schau.org:8080/json*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_boolean**](PropertyApi.md#get_boolean) | **GET** /property/getBoolean | Returns the boolean value of the property, this call will fail if the property is not of type ’boolean’.
[**get_children**](PropertyApi.md#get_children) | **GET** /property/getChildren | Returns an array of child nodes.
[**get_flags**](PropertyApi.md#get_flags) | **GET** /property/getFlags | Returns the flag values of a property.
[**get_integer**](PropertyApi.md#get_integer) | **GET** /property/getInteger | Returns the integer value of the property, this call will fail if the property is not of type ’integer’.
[**get_string**](PropertyApi.md#get_string) | **GET** /property/getString | Returns the string value of the property, this call will fail if the property is not of type ’string’.
[**get_type**](PropertyApi.md#get_type) | **GET** /property/getType | Returns the type of the property, this can be “none”, “string”, “integer” or “boolean”.
[**query2**](PropertyApi.md#query2) | **GET** /property/query2 | Returns a part of the tree specified by query.
[**remove**](PropertyApi.md#remove) | **GET** /property/remove | Removes a property node.
[**set_boolean**](PropertyApi.md#set_boolean) | **GET** /property/setBoolean | Sets the boolean value of the property, this call will fail if the property is not of type ’boolean’.
[**set_flags**](PropertyApi.md#set_flags) | **GET** /property/setFlags | Sets a given flag of a property.
[**set_integer**](PropertyApi.md#set_integer) | **GET** /property/setInteger | Sets the integer value of the property, this call will fail if the property is not of type ’integer’.
[**set_string**](PropertyApi.md#set_string) | **GET** /property/setString | Sets the string value of the property, this call will fail if the property is not of type ’string’.



## get_boolean

> crate::models::GetBoolean200Response get_boolean(path)
Returns the boolean value of the property, this call will fail if the property is not of type ’boolean’.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** | path of the property | [required] |

### Return type

[**crate::models::GetBoolean200Response**](getBoolean_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_children

> crate::models::GetChildren200Response get_children(path)
Returns an array of child nodes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** | path of the property | [required] |

### Return type

[**crate::models::GetChildren200Response**](getChildren_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_flags

> crate::models::GetFlags200Response get_flags(path)
Returns the flag values of a property.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** | path of the property | [required] |

### Return type

[**crate::models::GetFlags200Response**](getFlags_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integer

> crate::models::GetInteger200Response get_integer(path)
Returns the integer value of the property, this call will fail if the property is not of type ’integer’.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** | path of the property | [required] |

### Return type

[**crate::models::GetInteger200Response**](getInteger_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_string

> crate::models::GetString200Response get_string(path)
Returns the string value of the property, this call will fail if the property is not of type ’string’.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** | path of the property | [required] |

### Return type

[**crate::models::GetString200Response**](getString_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_type

> crate::models::GetType200Response get_type(path)
Returns the type of the property, this can be “none”, “string”, “integer” or “boolean”.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** | path of the property | [required] |

### Return type

[**crate::models::GetType200Response**](getType_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query2

> crate::models::Query2200Response query2(query)
Returns a part of the tree specified by query.

All queries start from the root. The properties to be included have to be put in parentheses. A query to get all device from zone4 would look like this: ’/apartment/ zones/zone4/_*(ZoneID,name)’. More complex combinations (see example below) are also possible.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | query string | [required] |

### Return type

[**crate::models::Query2200Response**](query2_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove

> crate::models::Subscribe200Response remove(path)
Removes a property node.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** | path of the property | [required] |

### Return type

[**crate::models::Subscribe200Response**](subscribe_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_boolean

> crate::models::Subscribe200Response set_boolean(path, value)
Sets the boolean value of the property, this call will fail if the property is not of type ’boolean’.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** | path of the property | [required] |
**value** | **bool** | boolean value to set | [required] |

### Return type

[**crate::models::Subscribe200Response**](subscribe_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_flags

> crate::models::Subscribe200Response set_flags(path, flag, value)
Sets a given flag of a property.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** | path of the property | [required] |
**flag** | **String** | flag identifier | [required] |
**value** | **bool** | boolean flag value | [required] |

### Return type

[**crate::models::Subscribe200Response**](subscribe_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_integer

> crate::models::Subscribe200Response set_integer(path, value)
Sets the integer value of the property, this call will fail if the property is not of type ’integer’.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** | path of the property | [required] |
**value** | **i32** | integer value to set | [required] |

### Return type

[**crate::models::Subscribe200Response**](subscribe_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_string

> crate::models::Subscribe200Response set_string(path, value)
Sets the string value of the property, this call will fail if the property is not of type ’string’.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** | path of the property | [required] |
**value** | **String** | string value to set | [required] |

### Return type

[**crate::models::Subscribe200Response**](subscribe_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

