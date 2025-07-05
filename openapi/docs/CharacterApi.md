# \CharacterApi

All URIs are relative to *https://esi.evetech.net/latest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_characters_character_id**](CharacterApi.md#get_characters_character_id) | **GET** /characters/{character_id}/ | Get character's public information
[**get_characters_character_id_agents_research**](CharacterApi.md#get_characters_character_id_agents_research) | **GET** /characters/{character_id}/agents_research/ | Get agents research
[**get_characters_character_id_blueprints**](CharacterApi.md#get_characters_character_id_blueprints) | **GET** /characters/{character_id}/blueprints/ | Get blueprints
[**get_characters_character_id_corporationhistory**](CharacterApi.md#get_characters_character_id_corporationhistory) | **GET** /characters/{character_id}/corporationhistory/ | Get corporation history
[**get_characters_character_id_fatigue**](CharacterApi.md#get_characters_character_id_fatigue) | **GET** /characters/{character_id}/fatigue/ | Get jump fatigue
[**get_characters_character_id_medals**](CharacterApi.md#get_characters_character_id_medals) | **GET** /characters/{character_id}/medals/ | Get medals
[**get_characters_character_id_notifications**](CharacterApi.md#get_characters_character_id_notifications) | **GET** /characters/{character_id}/notifications/ | Get character notifications
[**get_characters_character_id_notifications_contacts**](CharacterApi.md#get_characters_character_id_notifications_contacts) | **GET** /characters/{character_id}/notifications/contacts/ | Get new contact notifications
[**get_characters_character_id_portrait**](CharacterApi.md#get_characters_character_id_portrait) | **GET** /characters/{character_id}/portrait/ | Get character portraits
[**get_characters_character_id_roles**](CharacterApi.md#get_characters_character_id_roles) | **GET** /characters/{character_id}/roles/ | Get character corporation roles
[**get_characters_character_id_standings**](CharacterApi.md#get_characters_character_id_standings) | **GET** /characters/{character_id}/standings/ | Get standings
[**get_characters_character_id_titles**](CharacterApi.md#get_characters_character_id_titles) | **GET** /characters/{character_id}/titles/ | Get character corporation titles
[**post_characters_affiliation**](CharacterApi.md#post_characters_affiliation) | **POST** /characters/affiliation/ | Character affiliation
[**post_characters_character_id_cspa**](CharacterApi.md#post_characters_character_id_cspa) | **POST** /characters/{character_id}/cspa/ | Calculate a CSPA charge cost



## get_characters_character_id

> models::GetCharactersCharacterIdOk get_characters_character_id(character_id, datasource, if_none_match)
Get character's public information

Public information about a character  --- Alternate route: `/dev/characters/{character_id}/`  Alternate route: `/legacy/characters/{character_id}/`  Alternate route: `/v5/characters/{character_id}/`  --- This route is cached for up to 604800 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |

### Return type

[**models::GetCharactersCharacterIdOk**](get_characters_character_id_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_characters_character_id_agents_research

> Vec<models::GetCharactersCharacterIdAgentsResearch200Ok> get_characters_character_id_agents_research(character_id, datasource, if_none_match, token)
Get agents research

Return a list of agents research information for a character. The formula for finding the current research points with an agent is: currentPoints = remainderPoints + pointsPerDay * days(currentTime - researchStartDate)  --- Alternate route: `/dev/characters/{character_id}/agents_research/`  Alternate route: `/legacy/characters/{character_id}/agents_research/`  Alternate route: `/v1/characters/{character_id}/agents_research/`  Alternate route: `/v2/characters/{character_id}/agents_research/`  --- This route is cached for up to 3600 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

[**Vec<models::GetCharactersCharacterIdAgentsResearch200Ok>**](get_characters_character_id_agents_research_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_characters_character_id_blueprints

> Vec<models::GetCharactersCharacterIdBlueprints200Ok> get_characters_character_id_blueprints(character_id, datasource, if_none_match, page, token)
Get blueprints

Return a list of blueprints the character owns  --- Alternate route: `/dev/characters/{character_id}/blueprints/`  Alternate route: `/legacy/characters/{character_id}/blueprints/`  Alternate route: `/v2/characters/{character_id}/blueprints/`  Alternate route: `/v3/characters/{character_id}/blueprints/`  --- This route is cached for up to 3600 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**page** | Option<**i32**> | Which page of results to return |  |[default to 1]
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

[**Vec<models::GetCharactersCharacterIdBlueprints200Ok>**](get_characters_character_id_blueprints_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_characters_character_id_corporationhistory

> Vec<models::GetCharactersCharacterIdCorporationhistory200Ok> get_characters_character_id_corporationhistory(character_id, datasource, if_none_match)
Get corporation history

Get a list of all the corporations a character has been a member of  --- Alternate route: `/dev/characters/{character_id}/corporationhistory/`  Alternate route: `/legacy/characters/{character_id}/corporationhistory/`  Alternate route: `/v1/characters/{character_id}/corporationhistory/`  Alternate route: `/v2/characters/{character_id}/corporationhistory/`  --- This route is cached for up to 86400 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |

### Return type

[**Vec<models::GetCharactersCharacterIdCorporationhistory200Ok>**](get_characters_character_id_corporationhistory_200_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_characters_character_id_fatigue

> models::GetCharactersCharacterIdFatigueOk get_characters_character_id_fatigue(character_id, datasource, if_none_match, token)
Get jump fatigue

Return a character's jump activation and fatigue information  --- Alternate route: `/dev/characters/{character_id}/fatigue/`  Alternate route: `/legacy/characters/{character_id}/fatigue/`  Alternate route: `/v1/characters/{character_id}/fatigue/`  Alternate route: `/v2/characters/{character_id}/fatigue/`  --- This route is cached for up to 300 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

[**models::GetCharactersCharacterIdFatigueOk**](get_characters_character_id_fatigue_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_characters_character_id_medals

> Vec<models::GetCharactersCharacterIdMedals200Ok> get_characters_character_id_medals(character_id, datasource, if_none_match, token)
Get medals

Return a list of medals the character has  --- Alternate route: `/dev/characters/{character_id}/medals/`  Alternate route: `/legacy/characters/{character_id}/medals/`  Alternate route: `/v1/characters/{character_id}/medals/`  Alternate route: `/v2/characters/{character_id}/medals/`  --- This route is cached for up to 3600 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

[**Vec<models::GetCharactersCharacterIdMedals200Ok>**](get_characters_character_id_medals_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_characters_character_id_notifications

> Vec<models::GetCharactersCharacterIdNotifications200Ok> get_characters_character_id_notifications(character_id, datasource, if_none_match, token)
Get character notifications

Return character notifications  --- Alternate route: `/dev/characters/{character_id}/notifications/`  Alternate route: `/legacy/characters/{character_id}/notifications/`  Alternate route: `/v4/characters/{character_id}/notifications/`  Alternate route: `/v5/characters/{character_id}/notifications/`  Alternate route: `/v6/characters/{character_id}/notifications/`  --- This route is cached for up to 600 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

[**Vec<models::GetCharactersCharacterIdNotifications200Ok>**](get_characters_character_id_notifications_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_characters_character_id_notifications_contacts

> Vec<models::GetCharactersCharacterIdNotificationsContacts200Ok> get_characters_character_id_notifications_contacts(character_id, datasource, if_none_match, token)
Get new contact notifications

Return notifications about having been added to someone's contact list  --- Alternate route: `/dev/characters/{character_id}/notifications/contacts/`  Alternate route: `/legacy/characters/{character_id}/notifications/contacts/`  Alternate route: `/v1/characters/{character_id}/notifications/contacts/`  Alternate route: `/v2/characters/{character_id}/notifications/contacts/`  --- This route is cached for up to 600 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

[**Vec<models::GetCharactersCharacterIdNotificationsContacts200Ok>**](get_characters_character_id_notifications_contacts_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_characters_character_id_portrait

> models::GetCharactersCharacterIdPortraitOk get_characters_character_id_portrait(character_id, datasource, if_none_match)
Get character portraits

Get portrait urls for a character  --- Alternate route: `/dev/characters/{character_id}/portrait/`  Alternate route: `/v2/characters/{character_id}/portrait/`  Alternate route: `/v3/characters/{character_id}/portrait/`  --- This route expires daily at 11:05

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |

### Return type

[**models::GetCharactersCharacterIdPortraitOk**](get_characters_character_id_portrait_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_characters_character_id_roles

> models::GetCharactersCharacterIdRolesOk get_characters_character_id_roles(character_id, datasource, if_none_match, token)
Get character corporation roles

Returns a character's corporation roles  --- Alternate route: `/dev/characters/{character_id}/roles/`  Alternate route: `/legacy/characters/{character_id}/roles/`  Alternate route: `/v2/characters/{character_id}/roles/`  Alternate route: `/v3/characters/{character_id}/roles/`  --- This route is cached for up to 3600 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

[**models::GetCharactersCharacterIdRolesOk**](get_characters_character_id_roles_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_characters_character_id_standings

> Vec<models::GetCharactersCharacterIdStandings200Ok> get_characters_character_id_standings(character_id, datasource, if_none_match, token)
Get standings

Return character standings from agents, NPC corporations, and factions  --- Alternate route: `/dev/characters/{character_id}/standings/`  Alternate route: `/legacy/characters/{character_id}/standings/`  Alternate route: `/v1/characters/{character_id}/standings/`  Alternate route: `/v2/characters/{character_id}/standings/`  --- This route is cached for up to 3600 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

[**Vec<models::GetCharactersCharacterIdStandings200Ok>**](get_characters_character_id_standings_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_characters_character_id_titles

> Vec<models::GetCharactersCharacterIdTitles200Ok> get_characters_character_id_titles(character_id, datasource, if_none_match, token)
Get character corporation titles

Returns a character's titles  --- Alternate route: `/dev/characters/{character_id}/titles/`  Alternate route: `/legacy/characters/{character_id}/titles/`  Alternate route: `/v1/characters/{character_id}/titles/`  Alternate route: `/v2/characters/{character_id}/titles/`  --- This route is cached for up to 3600 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

[**Vec<models::GetCharactersCharacterIdTitles200Ok>**](get_characters_character_id_titles_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_characters_affiliation

> Vec<models::PostCharactersAffiliation200Ok> post_characters_affiliation(characters, datasource)
Character affiliation

Bulk lookup of character IDs to corporation, alliance and faction  --- Alternate route: `/dev/characters/affiliation/`  Alternate route: `/legacy/characters/affiliation/`  Alternate route: `/v1/characters/affiliation/`  Alternate route: `/v2/characters/affiliation/`  --- This route is cached for up to 3600 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**characters** | [**Vec<i32>**](i32.md) | The character IDs to fetch affiliations for. All characters must exist, or none will be returned | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]

### Return type

[**Vec<models::PostCharactersAffiliation200Ok>**](post_characters_affiliation_200_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_characters_character_id_cspa

> f32 post_characters_character_id_cspa(character_id, characters, datasource, token)
Calculate a CSPA charge cost

Takes a source character ID in the url and a set of target character ID's in the body, returns a CSPA charge cost  --- Alternate route: `/dev/characters/{character_id}/cspa/`  Alternate route: `/legacy/characters/{character_id}/cspa/`  Alternate route: `/v4/characters/{character_id}/cspa/`  Alternate route: `/v5/characters/{character_id}/cspa/` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**characters** | [**Vec<i32>**](i32.md) | The target characters to calculate the charge for | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

**f32**

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

