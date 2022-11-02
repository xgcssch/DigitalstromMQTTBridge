# \ApartmentApi

All URIs are relative to *https://dss.int.schau.org:8080/json*

Method | HTTP request | Description
------------- | ------------- | -------------
[**apartment_call_scene**](ApartmentApi.md#apartment_call_scene) | **GET** /apartment/callScene | Excutes the scene sceneNumber on a group of devices.
[**get_structure**](ApartmentApi.md#get_structure) | **GET** /apartment/getStructure | Returns an object containing the structure of the apartment. This includes detailed information about all zones, groups and devices.
[**set_value**](ApartmentApi.md#set_value) | **GET** /apartment/setValue | Set the output value of a group of devices to a given value.



## apartment_call_scene

> crate::models::Subscribe200Response apartment_call_scene(scene_number, group_id, group_name, force)
Excutes the scene sceneNumber on a group of devices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scene_number** | **i32** | Numerical value | [required] |
**group_id** | Option<**i32**> | Number of the target group |  |
**group_name** | Option<**String**> | Name of the target group |  |
**force** | Option<**bool**> | Boolean value, if set a forced scene call is issued |  |

### Return type

[**crate::models::Subscribe200Response**](subscribe_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_structure

> crate::models::GetStructure200Response get_structure()
Returns an object containing the structure of the apartment. This includes detailed information about all zones, groups and devices.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetStructure200Response**](getStructure_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_value

> crate::models::Subscribe200Response set_value(value, group_id, group_name)
Set the output value of a group of devices to a given value.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**value** | **i32** | Numerical value | [required] |
**group_id** | Option<**i32**> | Number of the target group |  |
**group_name** | Option<**String**> | Name of the target group |  |

### Return type

[**crate::models::Subscribe200Response**](subscribe_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

