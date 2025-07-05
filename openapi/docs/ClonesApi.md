# \ClonesApi

All URIs are relative to *https://esi.evetech.net/latest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_characters_character_id_clones**](ClonesApi.md#get_characters_character_id_clones) | **GET** /characters/{character_id}/clones/ | Get clones
[**get_characters_character_id_implants**](ClonesApi.md#get_characters_character_id_implants) | **GET** /characters/{character_id}/implants/ | Get active implants



## get_characters_character_id_clones

> models::GetCharactersCharacterIdClonesOk get_characters_character_id_clones(character_id, datasource, if_none_match, token)
Get clones

A list of the character's clones  --- Alternate route: `/dev/characters/{character_id}/clones/`  Alternate route: `/legacy/characters/{character_id}/clones/`  Alternate route: `/v2/characters/{character_id}/clones/`  Alternate route: `/v3/characters/{character_id}/clones/`  Alternate route: `/v4/characters/{character_id}/clones/`  --- This route is cached for up to 120 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

[**models::GetCharactersCharacterIdClonesOk**](get_characters_character_id_clones_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_characters_character_id_implants

> Vec<i32> get_characters_character_id_implants(character_id, datasource, if_none_match, token)
Get active implants

Return implants on the active clone of a character  --- Alternate route: `/dev/characters/{character_id}/implants/`  Alternate route: `/legacy/characters/{character_id}/implants/`  Alternate route: `/v1/characters/{character_id}/implants/`  Alternate route: `/v2/characters/{character_id}/implants/`  --- This route is cached for up to 120 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

**Vec<i32>**

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

