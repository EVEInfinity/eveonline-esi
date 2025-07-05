# \CalendarApi

All URIs are relative to *https://esi.evetech.net/latest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_characters_character_id_calendar**](CalendarApi.md#get_characters_character_id_calendar) | **GET** /characters/{character_id}/calendar/ | List calendar event summaries
[**get_characters_character_id_calendar_event_id**](CalendarApi.md#get_characters_character_id_calendar_event_id) | **GET** /characters/{character_id}/calendar/{event_id}/ | Get an event
[**get_characters_character_id_calendar_event_id_attendees**](CalendarApi.md#get_characters_character_id_calendar_event_id_attendees) | **GET** /characters/{character_id}/calendar/{event_id}/attendees/ | Get attendees
[**put_characters_character_id_calendar_event_id**](CalendarApi.md#put_characters_character_id_calendar_event_id) | **PUT** /characters/{character_id}/calendar/{event_id}/ | Respond to an event



## get_characters_character_id_calendar

> Vec<models::GetCharactersCharacterIdCalendar200Ok> get_characters_character_id_calendar(character_id, datasource, from_event, if_none_match, token)
List calendar event summaries

Get 50 event summaries from the calendar. If no from_event ID is given, the resource will return the next 50 chronological event summaries from now. If a from_event ID is specified, it will return the next 50 chronological event summaries from after that event  --- Alternate route: `/dev/characters/{character_id}/calendar/`  Alternate route: `/legacy/characters/{character_id}/calendar/`  Alternate route: `/v1/characters/{character_id}/calendar/`  Alternate route: `/v2/characters/{character_id}/calendar/`  --- This route is cached for up to 5 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**from_event** | Option<**i32**> | The event ID to retrieve events from |  |
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

[**Vec<models::GetCharactersCharacterIdCalendar200Ok>**](get_characters_character_id_calendar_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_characters_character_id_calendar_event_id

> models::GetCharactersCharacterIdCalendarEventIdOk get_characters_character_id_calendar_event_id(character_id, event_id, datasource, if_none_match, token)
Get an event

Get all the information for a specific event  --- Alternate route: `/dev/characters/{character_id}/calendar/{event_id}/`  Alternate route: `/legacy/characters/{character_id}/calendar/{event_id}/`  Alternate route: `/v3/characters/{character_id}/calendar/{event_id}/`  Alternate route: `/v4/characters/{character_id}/calendar/{event_id}/`  --- This route is cached for up to 5 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**event_id** | **i32** | The id of the event requested | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

[**models::GetCharactersCharacterIdCalendarEventIdOk**](get_characters_character_id_calendar_event_id_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_characters_character_id_calendar_event_id_attendees

> Vec<models::GetCharactersCharacterIdCalendarEventIdAttendees200Ok> get_characters_character_id_calendar_event_id_attendees(character_id, event_id, datasource, if_none_match, token)
Get attendees

Get all invited attendees for a given event  --- Alternate route: `/dev/characters/{character_id}/calendar/{event_id}/attendees/`  Alternate route: `/legacy/characters/{character_id}/calendar/{event_id}/attendees/`  Alternate route: `/v1/characters/{character_id}/calendar/{event_id}/attendees/`  Alternate route: `/v2/characters/{character_id}/calendar/{event_id}/attendees/`  --- This route is cached for up to 600 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**event_id** | **i32** | The id of the event requested | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

[**Vec<models::GetCharactersCharacterIdCalendarEventIdAttendees200Ok>**](get_characters_character_id_calendar_event_id_attendees_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_characters_character_id_calendar_event_id

> put_characters_character_id_calendar_event_id(character_id, event_id, response, datasource, token)
Respond to an event

Set your response status to an event  --- Alternate route: `/dev/characters/{character_id}/calendar/{event_id}/`  Alternate route: `/legacy/characters/{character_id}/calendar/{event_id}/`  Alternate route: `/v3/characters/{character_id}/calendar/{event_id}/`  Alternate route: `/v4/characters/{character_id}/calendar/{event_id}/`  --- This route is cached for up to 5 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**event_id** | **i32** | The ID of the event requested | [required] |
**response** | [**PutCharactersCharacterIdCalendarEventIdResponse**](PutCharactersCharacterIdCalendarEventIdResponse.md) | The response value to set, overriding current value | [required] |
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

