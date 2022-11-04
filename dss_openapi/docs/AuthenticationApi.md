# \AuthenticationApi

All URIs are relative to *https://dss.acme.com:8080/json*

Method | HTTP request | Description
------------- | ------------- | -------------
[**enable_token**](AuthenticationApi.md#enable_token) | **GET** /system/enableToken | Enables an application token, caller must be logged in.
[**logged_in_user**](AuthenticationApi.md#logged_in_user) | **GET** /system/loggedInUser | Returns the name of the currently logged in user.
[**login**](AuthenticationApi.md#login) | **GET** /system/login | Creates a new session using the provided credentials.
[**login_application**](AuthenticationApi.md#login_application) | **GET** /system/loginApplication | Creates a new session using the registered application token.
[**logout**](AuthenticationApi.md#logout) | **GET** /system/logout | Destroys the session and signs out the user.
[**request_application_token**](AuthenticationApi.md#request_application_token) | **GET** /system/requestApplicationToken | Creates a new session using the provided credentials.
[**revoke_token**](AuthenticationApi.md#revoke_token) | **GET** /system/revokeToken | Revokes an application token, caller must be logged in.



## enable_token

> crate::models::Status enable_token(application_token)
Enables an application token, caller must be logged in.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_token** | **String** | application token as string | [required] |

### Return type

[**crate::models::Status**](Status.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logged_in_user

> crate::models::LoggedInUser200Response logged_in_user()
Returns the name of the currently logged in user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LoggedInUser200Response**](loggedInUser_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## login

> crate::models::SessionTokenResponse login(user, password)
Creates a new session using the provided credentials.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user** | **String** | user name string | [required] |
**password** | **String** | password string | [required] |

### Return type

[**crate::models::SessionTokenResponse**](SessionTokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## login_application

> crate::models::SessionTokenResponse login_application(login_token)
Creates a new session using the registered application token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**login_token** | **String** | application token as string | [required] |

### Return type

[**crate::models::SessionTokenResponse**](SessionTokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logout

> crate::models::Status logout()
Destroys the session and signs out the user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Status**](Status.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## request_application_token

> crate::models::RequestApplicationToken200Response request_application_token(application_name)
Creates a new session using the provided credentials.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_name** | **String** | name of the application that requests the token | [required] |

### Return type

[**crate::models::RequestApplicationToken200Response**](requestApplicationToken_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_token

> crate::models::Status revoke_token(application_token)
Revokes an application token, caller must be logged in.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_token** | **String** | application token as string | [required] |

### Return type

[**crate::models::Status**](Status.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

