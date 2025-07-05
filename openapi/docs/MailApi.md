# \MailApi

All URIs are relative to *https://esi.evetech.net/latest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_characters_character_id_mail_labels_label_id**](MailApi.md#delete_characters_character_id_mail_labels_label_id) | **DELETE** /characters/{character_id}/mail/labels/{label_id}/ | Delete a mail label
[**delete_characters_character_id_mail_mail_id**](MailApi.md#delete_characters_character_id_mail_mail_id) | **DELETE** /characters/{character_id}/mail/{mail_id}/ | Delete a mail
[**get_characters_character_id_mail**](MailApi.md#get_characters_character_id_mail) | **GET** /characters/{character_id}/mail/ | Return mail headers
[**get_characters_character_id_mail_labels**](MailApi.md#get_characters_character_id_mail_labels) | **GET** /characters/{character_id}/mail/labels/ | Get mail labels and unread counts
[**get_characters_character_id_mail_lists**](MailApi.md#get_characters_character_id_mail_lists) | **GET** /characters/{character_id}/mail/lists/ | Return mailing list subscriptions
[**get_characters_character_id_mail_mail_id**](MailApi.md#get_characters_character_id_mail_mail_id) | **GET** /characters/{character_id}/mail/{mail_id}/ | Return a mail
[**post_characters_character_id_mail**](MailApi.md#post_characters_character_id_mail) | **POST** /characters/{character_id}/mail/ | Send a new mail
[**post_characters_character_id_mail_labels**](MailApi.md#post_characters_character_id_mail_labels) | **POST** /characters/{character_id}/mail/labels/ | Create a mail label
[**put_characters_character_id_mail_mail_id**](MailApi.md#put_characters_character_id_mail_mail_id) | **PUT** /characters/{character_id}/mail/{mail_id}/ | Update metadata about a mail



## delete_characters_character_id_mail_labels_label_id

> delete_characters_character_id_mail_labels_label_id(character_id, label_id, datasource, token)
Delete a mail label

Delete a mail label  --- Alternate route: `/dev/characters/{character_id}/mail/labels/{label_id}/`  Alternate route: `/legacy/characters/{character_id}/mail/labels/{label_id}/`  Alternate route: `/v1/characters/{character_id}/mail/labels/{label_id}/` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**label_id** | **i32** | An EVE label id | [required] |
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


## delete_characters_character_id_mail_mail_id

> delete_characters_character_id_mail_mail_id(character_id, mail_id, datasource, token)
Delete a mail

Delete a mail  --- Alternate route: `/dev/characters/{character_id}/mail/{mail_id}/`  Alternate route: `/legacy/characters/{character_id}/mail/{mail_id}/`  Alternate route: `/v1/characters/{character_id}/mail/{mail_id}/` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**mail_id** | **i32** | An EVE mail ID | [required] |
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


## get_characters_character_id_mail

> Vec<models::GetCharactersCharacterIdMail200Ok> get_characters_character_id_mail(character_id, datasource, if_none_match, labels, last_mail_id, token)
Return mail headers

Return the 50 most recent mail headers belonging to the character that match the query criteria. Queries can be filtered by label, and last_mail_id can be used to paginate backwards  --- Alternate route: `/dev/characters/{character_id}/mail/`  Alternate route: `/legacy/characters/{character_id}/mail/`  Alternate route: `/v1/characters/{character_id}/mail/`  --- This route is cached for up to 30 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**labels** | Option<[**Vec<i32>**](i32.md)> | Fetch only mails that match one or more of the given labels |  |
**last_mail_id** | Option<**i32**> | List only mail with an ID lower than the given ID, if present |  |
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

[**Vec<models::GetCharactersCharacterIdMail200Ok>**](get_characters_character_id_mail_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_characters_character_id_mail_labels

> models::GetCharactersCharacterIdMailLabelsOk get_characters_character_id_mail_labels(character_id, datasource, if_none_match, token)
Get mail labels and unread counts

Return a list of the users mail labels, unread counts for each label and a total unread count.  --- Alternate route: `/dev/characters/{character_id}/mail/labels/`  Alternate route: `/v3/characters/{character_id}/mail/labels/`  --- This route is cached for up to 30 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

[**models::GetCharactersCharacterIdMailLabelsOk**](get_characters_character_id_mail_labels_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_characters_character_id_mail_lists

> Vec<models::GetCharactersCharacterIdMailLists200Ok> get_characters_character_id_mail_lists(character_id, datasource, if_none_match, token)
Return mailing list subscriptions

Return all mailing lists that the character is subscribed to  --- Alternate route: `/dev/characters/{character_id}/mail/lists/`  Alternate route: `/legacy/characters/{character_id}/mail/lists/`  Alternate route: `/v1/characters/{character_id}/mail/lists/`  --- This route is cached for up to 120 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

[**Vec<models::GetCharactersCharacterIdMailLists200Ok>**](get_characters_character_id_mail_lists_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_characters_character_id_mail_mail_id

> models::GetCharactersCharacterIdMailMailIdOk get_characters_character_id_mail_mail_id(character_id, mail_id, datasource, if_none_match, token)
Return a mail

Return the contents of an EVE mail  --- Alternate route: `/dev/characters/{character_id}/mail/{mail_id}/`  Alternate route: `/legacy/characters/{character_id}/mail/{mail_id}/`  Alternate route: `/v1/characters/{character_id}/mail/{mail_id}/`  --- This route is cached for up to 30 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**mail_id** | **i32** | An EVE mail ID | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

[**models::GetCharactersCharacterIdMailMailIdOk**](get_characters_character_id_mail_mail_id_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_characters_character_id_mail

> i32 post_characters_character_id_mail(character_id, mail, datasource, token)
Send a new mail

Create and send a new mail  --- Alternate route: `/dev/characters/{character_id}/mail/`  Alternate route: `/legacy/characters/{character_id}/mail/`  Alternate route: `/v1/characters/{character_id}/mail/` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**mail** | [**PostCharactersCharacterIdMailMail**](PostCharactersCharacterIdMailMail.md) | The mail to send | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

**i32**

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_characters_character_id_mail_labels

> i32 post_characters_character_id_mail_labels(character_id, label, datasource, token)
Create a mail label

Create a mail label  --- Alternate route: `/dev/characters/{character_id}/mail/labels/`  Alternate route: `/legacy/characters/{character_id}/mail/labels/`  Alternate route: `/v2/characters/{character_id}/mail/labels/` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**label** | [**PostCharactersCharacterIdMailLabelsLabel**](PostCharactersCharacterIdMailLabelsLabel.md) | Label to create | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

**i32**

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_characters_character_id_mail_mail_id

> put_characters_character_id_mail_mail_id(character_id, mail_id, contents, datasource, token)
Update metadata about a mail

Update metadata about a mail  --- Alternate route: `/dev/characters/{character_id}/mail/{mail_id}/`  Alternate route: `/legacy/characters/{character_id}/mail/{mail_id}/`  Alternate route: `/v1/characters/{character_id}/mail/{mail_id}/` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**mail_id** | **i32** | An EVE mail ID | [required] |
**contents** | [**PutCharactersCharacterIdMailMailIdContents**](PutCharactersCharacterIdMailMailIdContents.md) | Data used to update the mail | [required] |
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

