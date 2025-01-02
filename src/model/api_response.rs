//! This module defines the structure for the high-level API response, including metadata,
//! pagination links, and data. The structure is deserialized from the JSON response from the API.
//! The detailed structures are defined in the submodules.

use crate::model::data::Data;
use crate::model::links::PaginationLinks;
use crate::model::meta::Meta;
use serde::de::{self, Deserializer};
use serde::Deserialize;

/// Represents the API response.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct ApiResponse {
    /// The metadata of the response.
    pub meta: Option<Meta>,
    /// The pagination links of the response.
    pub links: Option<PaginationLinks>,
    /// The data of the response.
    #[serde(deserialize_with = "data_or_vec")]
    pub data: DataOrVec,
}

/// Represents either a single `Data` object or a vector of `Data` objects.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum DataOrVec {
    Single(Box<Data>),
    Multiple(Vec<Data>),
}

/// Custom deserialization function to handle both single `Data` object and a vector of `Data` objects.
fn data_or_vec<'de, D>(deserializer: D) -> Result<DataOrVec, D::Error>
where
    D: Deserializer<'de>,
{
    struct DataOrVecVisitor;

    impl<'de> de::Visitor<'de> for DataOrVecVisitor {
        type Value = DataOrVec;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a single Data object or a sequence of Data objects")
        }

        fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>,
        {
            let vec: Vec<Data> =
                Deserialize::deserialize(de::value::SeqAccessDeserializer::new(seq))?;
            Ok(DataOrVec::Multiple(vec))
        }

        fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
        where
            A: de::MapAccess<'de>,
        {
            let data: Data = Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))?;
            Ok(DataOrVec::Single(Box::new(data)))
        }
    }

    deserializer.deserialize_any(DataOrVecVisitor)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn test_single_data_deserialization() {
        let json_data = json!({
            "meta": {
                "goldenCopy": {
                    "publishDate": "2025-01-02T08:00:00Z"
                }
            },
            "data": {
                "type": "lei-records",
                "id": "254900OPPU84GM83MG36",
                "attributes": {
                    "lei": "254900OPPU84GM83MG36",
                    "entity": {
                        "legalName": {
                            "name": "GLEIF Americas A NJ Nonprofit Corporation",
                            "language": "en"
                        },
                        "otherNames": [
                            {
                                "name": "GLEIF Americas",
                                "language": "en",
                                "type": "TRADING_OR_OPERATING_NAME"
                            }
                        ],
                        "transliteratedOtherNames": [],
                        "legalAddress": {
                            "language": "en",
                            "addressLines": [
                                "2500 Plaza 5",
                                "25th Floor",
                                "Harborside Financial Center"
                            ],
                            "city": "Jersey City",
                            "region": "US-NJ",
                            "country": "US",
                            "postalCode": "07311"
                        },
                        "headquartersAddress": {
                            "language": "en",
                            "addressLines": [
                                "2500 Plaza 5",
                                "25th Floor",
                                "Harborside Financial Center"
                            ],
                            "city": "Jersey City",
                            "region": "US-NJ",
                            "country": "US",
                            "postalCode": "07311"
                        },
                        "registeredAt": {
                            "id": "RA000625",
                            "other": null
                        },
                        "registeredAs": "0450486330",
                        "jurisdiction": "US-NJ",
                        "category": "GENERAL",
                        "legalForm": {
                            "id": "T4M6",
                            "other": null
                        },
                        "associatedEntity": {
                            "lei": null,
                            "name": null
                        },
                        "status": "ACTIVE",
                        "expiration": {
                            "date": null,
                            "reason": null
                        },
                        "successorEntity": {
                            "lei": null,
                            "name": null
                        },
                        "successorEntities": [],
                        "creationDate": "2020-05-01T00:00:00Z",
                        "subCategory": null,
                        "otherAddresses": [],
                        "eventGroups": []
                    },
                    "registration": {
                        "initialRegistrationDate": "2020-05-14T14:05:11Z",
                        "lastUpdateDate": "2024-03-19T18:01:37Z",
                        "status": "ISSUED",
                        "nextRenewalDate": "2025-04-17T08:45:13Z",
                        "managingLou": "5493001KJTIIGC8Y1R12",
                        "corroborationLevel": "FULLY_CORROBORATED",
                        "validatedAt": {
                            "id": "RA000625",
                            "other": null
                        },
                        "validatedAs": "0450486330",
                        "otherValidationAuthorities": []
                    },
                    "bic": null,
                    "mic": null,
                    "ocid": "us_nj/0450486330",
                    "spglobal": [
                        "668126391"
                    ],
                    "conformityFlag": "CONFORMING"
                },
                "relationships": {
                    "managing-lou": {
                        "links": {
                            "related": "https://api.gleif.org/api/v1/lei-records/254900OPPU84GM83MG36/managing-lou"
                        }
                    },
                    "lei-issuer": {
                        "links": {
                            "related": "https://api.gleif.org/api/v1/lei-records/254900OPPU84GM83MG36/lei-issuer"
                        }
                    },
                    "field-modifications": {
                        "links": {
                            "related": "https://api.gleif.org/api/v1/lei-records/254900OPPU84GM83MG36/field-modifications"
                        }
                    },
                    "direct-parent": {
                        "links": {
                            "reporting-exception": "https://api.gleif.org/api/v1/lei-records/254900OPPU84GM83MG36/direct-parent-reporting-exception"
                        }
                    },
                    "ultimate-parent": {
                        "links": {
                            "reporting-exception": "https://api.gleif.org/api/v1/lei-records/254900OPPU84GM83MG36/ultimate-parent-reporting-exception"
                        }
                    }
                },
                "links": {
                    "self": "https://api.gleif.org/api/v1/lei-records/254900OPPU84GM83MG36"
                }
            }
        });

        let api_response: ApiResponse = serde_json::from_value(json_data).unwrap();

        match api_response.data {
            DataOrVec::Single(data) => {
                assert_eq!(data.id, "254900OPPU84GM83MG36");
            }
            DataOrVec::Multiple(_) => {
                panic!("Expected single data object");
            }
        }
    }

    #[test]
    fn test_multiple_data_deserialization() {
        let json_data = json!({
            "meta": {
                "goldenCopy": {
                    "publishDate": "2025-01-02T08:00:00Z"
                }
            },
            "data": [
                {
                    "type": "lei-records",
                    "id": "254900OPPU84GM83MG36",
                    "attributes": {
                        "lei": "254900OPPU84GM83MG36",
                        "entity": {
                            "legalName": {
                                "name": "GLEIF Americas A NJ Nonprofit Corporation",
                                "language": "en"
                            },
                            "otherNames": [
                                {
                                    "name": "GLEIF Americas",
                                    "language": "en",
                                    "type": "TRADING_OR_OPERATING_NAME"
                                }
                            ],
                            "transliteratedOtherNames": [],
                            "legalAddress": {
                                "language": "en",
                                "addressLines": [
                                    "2500 Plaza 5",
                                    "25th Floor",
                                    "Harborside Financial Center"
                                ],
                                "city": "Jersey City",
                                "region": "US-NJ",
                                "country": "US",
                                "postalCode": "07311"
                            },
                            "headquartersAddress": {
                                "language": "en",
                                "addressLines": [
                                    "2500 Plaza 5",
                                    "25th Floor",
                                    "Harborside Financial Center"
                                ],
                                "city": "Jersey City",
                                "region": "US-NJ",
                                "country": "US",
                                "postalCode": "07311"
                            },
                            "registeredAt": {
                                "id": "RA000625",
                                "other": null
                            },
                            "registeredAs": "0450486330",
                            "jurisdiction": "US-NJ",
                            "category": "GENERAL",
                            "legalForm": {
                                "id": "T4M6",
                                "other": null
                            },
                            "associatedEntity": {
                                "lei": null,
                                "name": null
                            },
                            "status": "ACTIVE",
                            "expiration": {
                                "date": null,
                                "reason": null
                            },
                            "successorEntity": {
                                "lei": null,
                                "name": null
                            },
                            "successorEntities": [],
                            "creationDate": "2020-05-01T00:00:00Z",
                            "subCategory": null,
                            "otherAddresses": [],
                            "eventGroups": []
                        },
                        "registration": {
                            "initialRegistrationDate": "2020-05-14T14:05:11Z",
                            "lastUpdateDate": "2024-03-19T18:01:37Z",
                            "status": "ISSUED",
                            "nextRenewalDate": "2025-04-17T08:45:13Z",
                            "managingLou": "5493001KJTIIGC8Y1R12",
                            "corroborationLevel": "FULLY_CORROBORATED",
                            "validatedAt": {
                                "id": "RA000625",
                                "other": null
                            },
                            "validatedAs": "0450486330",
                            "otherValidationAuthorities": []
                        },
                        "bic": null,
                        "mic": null,
                        "ocid": "us_nj/0450486330",
                        "spglobal": [
                            "668126391"
                        ],
                        "conformityFlag": "CONFORMING"
                    },
                    "relationships": {
                        "managing-lou": {
                            "links": {
                                "related": "https://api.gleif.org/api/v1/lei-records/254900OPPU84GM83MG36/managing-lou"
                            }
                        },
                        "lei-issuer": {
                            "links": {
                                "related": "https://api.gleif.org/api/v1/lei-records/254900OPPU84GM83MG36/lei-issuer"
                            }
                        },
                        "field-modifications": {
                            "links": {
                                "related": "https://api.gleif.org/api/v1/lei-records/254900OPPU84GM83MG36/field-modifications"
                            }
                        },
                        "direct-parent": {
                            "links": {
                                "reporting-exception": "https://api.gleif.org/api/v1/lei-records/254900OPPU84GM83MG36/direct-parent-reporting-exception"
                            }
                        },
                        "ultimate-parent": {
                            "links": {
                                "reporting-exception": "https://api.gleif.org/api/v1/lei-records/254900OPPU84GM83MG36/ultimate-parent-reporting-exception"
                            }
                        }
                    },
                    "links": {
                        "self": "https://api.gleif.org/api/v1/lei-records/254900OPPU84GM83MG36"
                    }
                },
                {
                    "type": "lei-records",
                    "id": "254900OPPU84GM83MG37",
                    "attributes": {
                        "lei": "254900OPPU84GM83MG37",
                        "entity": {
                            "legalName": {
                                "name": "Another Entity",
                                "language": "en"
                            },
                            "otherNames": [],
                            "transliteratedOtherNames": [],
                            "legalAddress": {
                                "language": "en",
                                "addressLines": [
                                    "123 Another St"
                                ],
                                "city": "Another City",
                                "region": "US-CA",
                                "country": "US",
                                "postalCode": "90001"
                            },
                            "headquartersAddress": {
                                "language": "en",
                                "addressLines": [
                                    "123 Another St"
                                ],
                                "city": "Another City",
                                "region": "US-CA",
                                "country": "US",
                                "postalCode": "90001"
                            },
                            "registeredAt": {
                                "id": "RA000626",
                                "other": null
                            },
                            "registeredAs": "1234567890",
                            "jurisdiction": "US-CA",
                            "category": "GENERAL",
                            "legalForm": {
                                "id": "T4M7",
                                "other": null
                            },
                            "associatedEntity": {
                                "lei": null,
                                "name": null
                            },
                            "status": "ACTIVE",
                            "expiration": {
                                "date": null,
                                "reason": null
                            },
                            "successorEntity": {
                                "lei": null,
                                "name": null
                            },
                            "successorEntities": [],
                            "creationDate": "2021-01-01T00:00:00Z",
                            "subCategory": null,
                            "otherAddresses": [],
                            "eventGroups": []
                        },
                        "registration": {
                            "initialRegistrationDate": "2021-01-01T00:00:00Z",
                            "lastUpdateDate": "2024-01-01T00:00:00Z",
                            "status": "ISSUED",
                            "nextRenewalDate": "2025-01-01T00:00:00Z",
                            "managingLou": "5493001KJTIIGC8Y1R13",
                            "corroborationLevel": "FULLY_CORROBORATED",
                            "validatedAt": {
                                "id": "RA000626",
                                "other": null
                            },
                            "validatedAs": "1234567890",
                            "otherValidationAuthorities": []
                        },
                        "bic": null,
                        "mic": null,
                        "ocid": "us_ca/1234567890",
                        "spglobal": [],
                        "conformityFlag": "CONFORMING"
                    },
                    "relationships": {
                        "managing-lou": {
                            "links": {
                                "related": "https://api.gleif.org/api/v1/lei-records/254900OPPU84GM83MG37/managing-lou"
                            }
                        },
                        "lei-issuer": {
                            "links": {
                                "related": "https://api.gleif.org/api/v1/lei-records/254900OPPU84GM83MG37/lei-issuer"
                            }
                        },
                        "field-modifications": {
                            "links": {
                                "related": "https://api.gleif.org/api/v1/lei-records/254900OPPU84GM83MG37/field-modifications"
                            }
                        },
                        "direct-parent": {
                            "links": {
                                "reporting-exception": "https://api.gleif.org/api/v1/lei-records/254900OPPU84GM83MG37/direct-parent-reporting-exception"
                            }
                        },
                        "ultimate-parent": {
                            "links": {
                                "reporting-exception": "https://api.gleif.org/api/v1/lei-records/254900OPPU84GM83MG37/ultimate-parent-reporting-exception"
                            }
                        }
                    },
                    "links": {
                        "self": "https://api.gleif.org/api/v1/lei-records/254900OPPU84GM83MG37"
                    }
                }
            ]
        });

        let api_response: ApiResponse = serde_json::from_value(json_data).unwrap();

        match api_response.data {
            DataOrVec::Single(_) => {
                panic!("Expected multiple data objects");
            }
            DataOrVec::Multiple(data_vec) => {
                assert_eq!(data_vec.len(), 2);
                assert_eq!(data_vec[0].id, "254900OPPU84GM83MG36");
                assert_eq!(data_vec[1].id, "254900OPPU84GM83MG37");
            }
        }
    }
}
