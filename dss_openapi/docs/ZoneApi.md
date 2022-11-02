# \ZoneApi

All URIs are relative to *https://dss.int.schau.org:8080/json*

Method | HTTP request | Description
------------- | ------------- | -------------
[**zone_call_scene**](ZoneApi.md#zone_call_scene) | **GET** /zone/callScene | Excutes the scene sceneNumber in a zone for a group of devices.



## zone_call_scene

> crate::models::Subscribe200Response zone_call_scene(scene_number, id, group_id, group_name, force)
Excutes the scene sceneNumber in a zone for a group of devices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scene_number** | **i32** | Numerical value | [required] |
**id** | Option<**i32**> | Number of the target zone |  |
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

