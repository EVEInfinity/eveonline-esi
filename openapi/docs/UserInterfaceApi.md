# \UserInterfaceApi

All URIs are relative to *https://esi.evetech.net/latest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**post_ui_autopilot_waypoint**](UserInterfaceApi.md#post_ui_autopilot_waypoint) | **POST** /ui/autopilot/waypoint/ | Set Autopilot Waypoint
[**post_ui_openwindow_contract**](UserInterfaceApi.md#post_ui_openwindow_contract) | **POST** /ui/openwindow/contract/ | Open Contract Window
[**post_ui_openwindow_information**](UserInterfaceApi.md#post_ui_openwindow_information) | **POST** /ui/openwindow/information/ | Open Information Window
[**post_ui_openwindow_marketdetails**](UserInterfaceApi.md#post_ui_openwindow_marketdetails) | **POST** /ui/openwindow/marketdetails/ | Open Market Details
[**post_ui_openwindow_newmail**](UserInterfaceApi.md#post_ui_openwindow_newmail) | **POST** /ui/openwindow/newmail/ | Open New Mail Window



## post_ui_autopilot_waypoint

> post_ui_autopilot_waypoint(add_to_beginning, clear_other_waypoints, destination_id, datasource, token)
Set Autopilot Waypoint

Set a solar system as autopilot waypoint  --- Alternate route: `/dev/ui/autopilot/waypoint/`  Alternate route: `/legacy/ui/autopilot/waypoint/`  Alternate route: `/v2/ui/autopilot/waypoint/` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_to_beginning** | **bool** | Whether this solar system should be added to the beginning of all waypoints | [required] |[default to false]
**clear_other_waypoints** | **bool** | Whether clean other waypoints beforing adding this one | [required] |[default to false]
**destination_id** | **i64** | The destination to travel to, can be solar system, station or structure's id | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

 (empty response body)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_ui_openwindow_contract

> post_ui_openwindow_contract(contract_id, datasource, token)
Open Contract Window

Open the contract window inside the client  --- Alternate route: `/dev/ui/openwindow/contract/`  Alternate route: `/legacy/ui/openwindow/contract/`  Alternate route: `/v1/ui/openwindow/contract/` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **i32** | The contract to open | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

 (empty response body)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_ui_openwindow_information

> post_ui_openwindow_information(target_id, datasource, token)
Open Information Window

Open the information window for a character, corporation or alliance inside the client  --- Alternate route: `/dev/ui/openwindow/information/`  Alternate route: `/legacy/ui/openwindow/information/`  Alternate route: `/v1/ui/openwindow/information/` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target_id** | **i32** | The target to open | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

 (empty response body)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_ui_openwindow_marketdetails

> post_ui_openwindow_marketdetails(type_id, datasource, token)
Open Market Details

Open the market details window for a specific typeID inside the client  --- Alternate route: `/dev/ui/openwindow/marketdetails/`  Alternate route: `/legacy/ui/openwindow/marketdetails/`  Alternate route: `/v1/ui/openwindow/marketdetails/` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**type_id** | **i32** | The item type to open in market window | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

 (empty response body)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_ui_openwindow_newmail

> post_ui_openwindow_newmail(new_mail, datasource, token)
Open New Mail Window

Open the New Mail window, according to settings from the request if applicable  --- Alternate route: `/dev/ui/openwindow/newmail/`  Alternate route: `/legacy/ui/openwindow/newmail/`  Alternate route: `/v1/ui/openwindow/newmail/` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_mail** | [**PostUiOpenwindowNewmailNewMail**](PostUiOpenwindowNewmailNewMail.md) | The details of mail to create | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

 (empty response body)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

