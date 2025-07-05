# \ContractsApi

All URIs are relative to *https://esi.evetech.net/latest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_characters_character_id_contracts**](ContractsApi.md#get_characters_character_id_contracts) | **GET** /characters/{character_id}/contracts/ | Get contracts
[**get_characters_character_id_contracts_contract_id_bids**](ContractsApi.md#get_characters_character_id_contracts_contract_id_bids) | **GET** /characters/{character_id}/contracts/{contract_id}/bids/ | Get contract bids
[**get_characters_character_id_contracts_contract_id_items**](ContractsApi.md#get_characters_character_id_contracts_contract_id_items) | **GET** /characters/{character_id}/contracts/{contract_id}/items/ | Get contract items
[**get_contracts_public_bids_contract_id**](ContractsApi.md#get_contracts_public_bids_contract_id) | **GET** /contracts/public/bids/{contract_id}/ | Get public contract bids
[**get_contracts_public_items_contract_id**](ContractsApi.md#get_contracts_public_items_contract_id) | **GET** /contracts/public/items/{contract_id}/ | Get public contract items
[**get_contracts_public_region_id**](ContractsApi.md#get_contracts_public_region_id) | **GET** /contracts/public/{region_id}/ | Get public contracts
[**get_corporations_corporation_id_contracts**](ContractsApi.md#get_corporations_corporation_id_contracts) | **GET** /corporations/{corporation_id}/contracts/ | Get corporation contracts
[**get_corporations_corporation_id_contracts_contract_id_bids**](ContractsApi.md#get_corporations_corporation_id_contracts_contract_id_bids) | **GET** /corporations/{corporation_id}/contracts/{contract_id}/bids/ | Get corporation contract bids
[**get_corporations_corporation_id_contracts_contract_id_items**](ContractsApi.md#get_corporations_corporation_id_contracts_contract_id_items) | **GET** /corporations/{corporation_id}/contracts/{contract_id}/items/ | Get corporation contract items



## get_characters_character_id_contracts

> Vec<models::GetCharactersCharacterIdContracts200Ok> get_characters_character_id_contracts(character_id, datasource, if_none_match, page, token)
Get contracts

Returns contracts available to a character, only if the character is issuer, acceptor or assignee. Only returns contracts no older than 30 days, or if the status is \"in_progress\".  --- Alternate route: `/dev/characters/{character_id}/contracts/`  Alternate route: `/legacy/characters/{character_id}/contracts/`  Alternate route: `/v1/characters/{character_id}/contracts/`  --- This route is cached for up to 300 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**page** | Option<**i32**> | Which page of results to return |  |[default to 1]
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

[**Vec<models::GetCharactersCharacterIdContracts200Ok>**](get_characters_character_id_contracts_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_characters_character_id_contracts_contract_id_bids

> Vec<models::GetCharactersCharacterIdContractsContractIdBids200Ok> get_characters_character_id_contracts_contract_id_bids(character_id, contract_id, datasource, if_none_match, token)
Get contract bids

Lists bids on a particular auction contract  --- Alternate route: `/dev/characters/{character_id}/contracts/{contract_id}/bids/`  Alternate route: `/legacy/characters/{character_id}/contracts/{contract_id}/bids/`  Alternate route: `/v1/characters/{character_id}/contracts/{contract_id}/bids/`  --- This route is cached for up to 300 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**contract_id** | **i32** | ID of a contract | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

[**Vec<models::GetCharactersCharacterIdContractsContractIdBids200Ok>**](get_characters_character_id_contracts_contract_id_bids_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_characters_character_id_contracts_contract_id_items

> Vec<models::GetCharactersCharacterIdContractsContractIdItems200Ok> get_characters_character_id_contracts_contract_id_items(character_id, contract_id, datasource, if_none_match, token)
Get contract items

Lists items of a particular contract  --- Alternate route: `/dev/characters/{character_id}/contracts/{contract_id}/items/`  Alternate route: `/legacy/characters/{character_id}/contracts/{contract_id}/items/`  Alternate route: `/v1/characters/{character_id}/contracts/{contract_id}/items/`  --- This route is cached for up to 3600 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**contract_id** | **i32** | ID of a contract | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

[**Vec<models::GetCharactersCharacterIdContractsContractIdItems200Ok>**](get_characters_character_id_contracts_contract_id_items_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contracts_public_bids_contract_id

> Vec<models::GetContractsPublicBidsContractId200Ok> get_contracts_public_bids_contract_id(contract_id, datasource, if_none_match, page)
Get public contract bids

Lists bids on a public auction contract  --- Alternate route: `/dev/contracts/public/bids/{contract_id}/`  Alternate route: `/legacy/contracts/public/bids/{contract_id}/`  Alternate route: `/v1/contracts/public/bids/{contract_id}/`  --- This route is cached for up to 300 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **i32** | ID of a contract | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**page** | Option<**i32**> | Which page of results to return |  |[default to 1]

### Return type

[**Vec<models::GetContractsPublicBidsContractId200Ok>**](get_contracts_public_bids_contract_id_200_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contracts_public_items_contract_id

> Vec<models::GetContractsPublicItemsContractId200Ok> get_contracts_public_items_contract_id(contract_id, datasource, if_none_match, page)
Get public contract items

Lists items of a public contract  --- Alternate route: `/dev/contracts/public/items/{contract_id}/`  Alternate route: `/legacy/contracts/public/items/{contract_id}/`  Alternate route: `/v1/contracts/public/items/{contract_id}/`  --- This route is cached for up to 3600 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **i32** | ID of a contract | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**page** | Option<**i32**> | Which page of results to return |  |[default to 1]

### Return type

[**Vec<models::GetContractsPublicItemsContractId200Ok>**](get_contracts_public_items_contract_id_200_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contracts_public_region_id

> Vec<models::GetContractsPublicRegionId200Ok> get_contracts_public_region_id(region_id, datasource, if_none_match, page)
Get public contracts

Returns a paginated list of all public contracts in the given region  --- Alternate route: `/dev/contracts/public/{region_id}/`  Alternate route: `/legacy/contracts/public/{region_id}/`  Alternate route: `/v1/contracts/public/{region_id}/`  --- This route is cached for up to 1800 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region_id** | **i32** | An EVE region id | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**page** | Option<**i32**> | Which page of results to return |  |[default to 1]

### Return type

[**Vec<models::GetContractsPublicRegionId200Ok>**](get_contracts_public_region_id_200_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_corporations_corporation_id_contracts

> Vec<models::GetCorporationsCorporationIdContracts200Ok> get_corporations_corporation_id_contracts(corporation_id, datasource, if_none_match, page, token)
Get corporation contracts

Returns contracts available to a corporation, only if the corporation is issuer, acceptor or assignee. Only returns contracts no older than 30 days, or if the status is \"in_progress\".  --- Alternate route: `/dev/corporations/{corporation_id}/contracts/`  Alternate route: `/legacy/corporations/{corporation_id}/contracts/`  Alternate route: `/v1/corporations/{corporation_id}/contracts/`  --- This route is cached for up to 300 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**corporation_id** | **i32** | An EVE corporation ID | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**page** | Option<**i32**> | Which page of results to return |  |[default to 1]
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

[**Vec<models::GetCorporationsCorporationIdContracts200Ok>**](get_corporations_corporation_id_contracts_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_corporations_corporation_id_contracts_contract_id_bids

> Vec<models::GetCorporationsCorporationIdContractsContractIdBids200Ok> get_corporations_corporation_id_contracts_contract_id_bids(contract_id, corporation_id, datasource, if_none_match, page, token)
Get corporation contract bids

Lists bids on a particular auction contract  --- Alternate route: `/dev/corporations/{corporation_id}/contracts/{contract_id}/bids/`  Alternate route: `/legacy/corporations/{corporation_id}/contracts/{contract_id}/bids/`  Alternate route: `/v1/corporations/{corporation_id}/contracts/{contract_id}/bids/`  --- This route is cached for up to 3600 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **i32** | ID of a contract | [required] |
**corporation_id** | **i32** | An EVE corporation ID | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**page** | Option<**i32**> | Which page of results to return |  |[default to 1]
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

[**Vec<models::GetCorporationsCorporationIdContractsContractIdBids200Ok>**](get_corporations_corporation_id_contracts_contract_id_bids_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_corporations_corporation_id_contracts_contract_id_items

> Vec<models::GetCorporationsCorporationIdContractsContractIdItems200Ok> get_corporations_corporation_id_contracts_contract_id_items(contract_id, corporation_id, datasource, if_none_match, token)
Get corporation contract items

Lists items of a particular contract  --- Alternate route: `/dev/corporations/{corporation_id}/contracts/{contract_id}/items/`  Alternate route: `/legacy/corporations/{corporation_id}/contracts/{contract_id}/items/`  Alternate route: `/v1/corporations/{corporation_id}/contracts/{contract_id}/items/`  --- This route is cached for up to 3600 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **i32** | ID of a contract | [required] |
**corporation_id** | **i32** | An EVE corporation ID | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

[**Vec<models::GetCorporationsCorporationIdContractsContractIdItems200Ok>**](get_corporations_corporation_id_contracts_contract_id_items_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

