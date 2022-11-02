# \EventApi

All URIs are relative to *https://dss.int.schau.org:8080/json*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get**](EventApi.md#get) | **GET** /event/get | Get event and context information for an event subscription. All events subscribed with the given Id will be handled by this call. An optional timeout value in milliseconds can be specified and will block the call until either an event or the timeout occurs. If the timeout value is zero or missing the call will not timeout.
[**subscribe**](EventApi.md#subscribe) | **GET** /event/subscribe | Subscribe to an event with the given name and registers the callers subscriptionId. A unique subscriptionId can be selected by the subscriber. It is possible to subscribe to several events reusing the same subscriptionId.
[**unsubscribe**](EventApi.md#unsubscribe) | **GET** /event/unsubscribe | Unsubscribes for the previously registered events by giving the event name and the unique subscriptionId.



## get

> crate::models::Get200Response get(subscription_id, timeout)
Get event and context information for an event subscription. All events subscribed with the given Id will be handled by this call. An optional timeout value in milliseconds can be specified and will block the call until either an event or the timeout occurs. If the timeout value is zero or missing the call will not timeout.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **i32** | numerical unique value | [required] |
**timeout** | Option<**i32**> | numerical unique value |  |

### Return type

[**crate::models::Get200Response**](get_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscribe

> crate::models::Subscribe200Response subscribe(name, subscription_id)
Subscribe to an event with the given name and registers the callers subscriptionId. A unique subscriptionId can be selected by the subscriber. It is possible to subscribe to several events reusing the same subscriptionId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | identifier string for the event | [required] |
**subscription_id** | **i32** | numerical unique value | [required] |

### Return type

[**crate::models::Subscribe200Response**](subscribe_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unsubscribe

> crate::models::Subscribe200Response unsubscribe(name, subscription_id)
Unsubscribes for the previously registered events by giving the event name and the unique subscriptionId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | identifier string for the event | [required] |
**subscription_id** | **i32** | numerical unique value | [required] |

### Return type

[**crate::models::Subscribe200Response**](subscribe_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

