# \DeviceApi

All URIs are relative to *https://dss.acme.com:8080/json*

Method | HTTP request | Description
------------- | ------------- | -------------
[**decrease_value**](DeviceApi.md#decrease_value) | **GET** /device/decreaseValue | Tells devices to execute the scene DEC.
[**increase_value**](DeviceApi.md#increase_value) | **GET** /device/increaseValue | Tells devices to execute the scene INC.
[**turn_off**](DeviceApi.md#turn_off) | **GET** /device/turnOff | Tells devices to execute the scene MIN.
[**turn_on**](DeviceApi.md#turn_on) | **GET** /device/turnOn | Tells devices to execute the scene MAX.



## decrease_value

> crate::models::Status decrease_value(dsid)
Tells devices to execute the scene DEC.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dsid** | **String** | dsid of the device | [required] |

### Return type

[**crate::models::Status**](Status.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## increase_value

> crate::models::Status increase_value(dsid)
Tells devices to execute the scene INC.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dsid** | **String** | dsid of the device | [required] |

### Return type

[**crate::models::Status**](Status.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## turn_off

> crate::models::Status turn_off(dsid)
Tells devices to execute the scene MIN.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dsid** | **String** | dsid of the device | [required] |

### Return type

[**crate::models::Status**](Status.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## turn_on

> crate::models::Status turn_on(dsid)
Tells devices to execute the scene MAX.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dsid** | **String** | dsid of the device | [required] |

### Return type

[**crate::models::Status**](Status.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

