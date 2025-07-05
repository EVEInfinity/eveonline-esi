# \SovereigntyApi

All URIs are relative to *https://esi.evetech.net/latest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_sovereignty_campaigns**](SovereigntyApi.md#get_sovereignty_campaigns) | **GET** /sovereignty/campaigns/ | List sovereignty campaigns
[**get_sovereignty_map**](SovereigntyApi.md#get_sovereignty_map) | **GET** /sovereignty/map/ | List sovereignty of systems
[**get_sovereignty_structures**](SovereigntyApi.md#get_sovereignty_structures) | **GET** /sovereignty/structures/ | List sovereignty structures



## get_sovereignty_campaigns

> Vec<models::GetSovereigntyCampaigns200Ok> get_sovereignty_campaigns(datasource, if_none_match)
List sovereignty campaigns

Shows sovereignty data for campaigns.  --- Alternate route: `/dev/sovereignty/campaigns/`  Alternate route: `/legacy/sovereignty/campaigns/`  Alternate route: `/v1/sovereignty/campaigns/`  --- This route is cached for up to 5 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |

### Return type

[**Vec<models::GetSovereigntyCampaigns200Ok>**](get_sovereignty_campaigns_200_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sovereignty_map

> Vec<models::GetSovereigntyMap200Ok> get_sovereignty_map(datasource, if_none_match)
List sovereignty of systems

Shows sovereignty information for solar systems  --- Alternate route: `/dev/sovereignty/map/`  Alternate route: `/legacy/sovereignty/map/`  Alternate route: `/v1/sovereignty/map/`  --- This route is cached for up to 3600 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |

### Return type

[**Vec<models::GetSovereigntyMap200Ok>**](get_sovereignty_map_200_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sovereignty_structures

> Vec<models::GetSovereigntyStructures200Ok> get_sovereignty_structures(datasource, if_none_match)
List sovereignty structures

Shows sovereignty data for structures.  --- Alternate route: `/dev/sovereignty/structures/`  Alternate route: `/legacy/sovereignty/structures/`  Alternate route: `/v1/sovereignty/structures/`  --- This route is cached for up to 120 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |

### Return type

[**Vec<models::GetSovereigntyStructures200Ok>**](get_sovereignty_structures_200_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

