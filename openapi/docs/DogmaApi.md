# \DogmaApi

All URIs are relative to *https://esi.evetech.net/latest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_dogma_attributes**](DogmaApi.md#get_dogma_attributes) | **GET** /dogma/attributes/ | Get attributes
[**get_dogma_attributes_attribute_id**](DogmaApi.md#get_dogma_attributes_attribute_id) | **GET** /dogma/attributes/{attribute_id}/ | Get attribute information
[**get_dogma_dynamic_items_type_id_item_id**](DogmaApi.md#get_dogma_dynamic_items_type_id_item_id) | **GET** /dogma/dynamic/items/{type_id}/{item_id}/ | Get dynamic item information
[**get_dogma_effects**](DogmaApi.md#get_dogma_effects) | **GET** /dogma/effects/ | Get effects
[**get_dogma_effects_effect_id**](DogmaApi.md#get_dogma_effects_effect_id) | **GET** /dogma/effects/{effect_id}/ | Get effect information



## get_dogma_attributes

> Vec<i32> get_dogma_attributes(datasource, if_none_match)
Get attributes

Get a list of dogma attribute ids  --- Alternate route: `/dev/dogma/attributes/`  Alternate route: `/legacy/dogma/attributes/`  Alternate route: `/v1/dogma/attributes/`  --- This route expires daily at 11:05

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |

### Return type

**Vec<i32>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dogma_attributes_attribute_id

> models::GetDogmaAttributesAttributeIdOk get_dogma_attributes_attribute_id(attribute_id, datasource, if_none_match)
Get attribute information

Get information on a dogma attribute  --- Alternate route: `/dev/dogma/attributes/{attribute_id}/`  Alternate route: `/legacy/dogma/attributes/{attribute_id}/`  Alternate route: `/v1/dogma/attributes/{attribute_id}/`  --- This route expires daily at 11:05

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attribute_id** | **i32** | A dogma attribute ID | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |

### Return type

[**models::GetDogmaAttributesAttributeIdOk**](get_dogma_attributes_attribute_id_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dogma_dynamic_items_type_id_item_id

> models::GetDogmaDynamicItemsTypeIdItemIdOk get_dogma_dynamic_items_type_id_item_id(item_id, type_id, datasource, if_none_match)
Get dynamic item information

Returns info about a dynamic item resulting from mutation with a mutaplasmid.  --- Alternate route: `/dev/dogma/dynamic/items/{type_id}/{item_id}/`  Alternate route: `/legacy/dogma/dynamic/items/{type_id}/{item_id}/`  Alternate route: `/v1/dogma/dynamic/items/{type_id}/{item_id}/`  --- This route expires daily at 11:05

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **i64** | item_id integer | [required] |
**type_id** | **i32** | type_id integer | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |

### Return type

[**models::GetDogmaDynamicItemsTypeIdItemIdOk**](get_dogma_dynamic_items_type_id_item_id_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dogma_effects

> Vec<i32> get_dogma_effects(datasource, if_none_match)
Get effects

Get a list of dogma effect ids  --- Alternate route: `/dev/dogma/effects/`  Alternate route: `/legacy/dogma/effects/`  Alternate route: `/v1/dogma/effects/`  --- This route expires daily at 11:05

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |

### Return type

**Vec<i32>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dogma_effects_effect_id

> models::GetDogmaEffectsEffectIdOk get_dogma_effects_effect_id(effect_id, datasource, if_none_match)
Get effect information

Get information on a dogma effect  --- Alternate route: `/dev/dogma/effects/{effect_id}/`  Alternate route: `/v2/dogma/effects/{effect_id}/`  --- This route expires daily at 11:05

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**effect_id** | **i32** | A dogma effect ID | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |

### Return type

[**models::GetDogmaEffectsEffectIdOk**](get_dogma_effects_effect_id_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

