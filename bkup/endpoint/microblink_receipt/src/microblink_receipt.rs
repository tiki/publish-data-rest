/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use apache_avro::Schema;
use ingest_lib::Ingest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MicroblinkReceipt {
    receipt_date: Option<StringType>,
    receipt_time: Option<StringType>,
    retailer_id: Retailer,
    products: Option<Vec<Product>>,
    coupons: Option<Vec<Coupon>>,
    total: Option<FloatType>,
    tip: Option<FloatType>,
    subtotal: Option<FloatType>,
    taxes: Option<FloatType>,
    store_number: Option<StringType>,
    merchant_name: Option<StringType>,
    store_address: Option<StringType>,
    store_city: Option<StringType>,
    blink_receipt_id: Option<String>,
    store_state: Option<StringType>,
    store_zip: Option<StringType>,
    store_country: Option<StringType>,
    store_phone: Option<StringType>,
    cashier_id: Option<StringType>,
    transaction_id: Option<StringType>,
    register_id: Option<StringType>,
    payment_methods: Option<Vec<PaymentMethod>>,
    tax_id: Option<StringType>,
    mall_name: Option<StringType>,
    last4cc: Option<StringType>,
    ocr_confidence: f32,
    merchant_source: Option<String>,
    found_top_edge: bool,
    found_bottom_edge: bool,
    e_receipt_order_number: Option<String>,
    e_receipt_order_status: Option<String>,
    e_receipt_raw_html: Option<String>,
    e_receipt_shipping_address: Option<String>,
    shipments: Option<Vec<Shipment>>,
    long_transaction_id: Option<StringType>,
    subtotal_matches: bool,
    e_receipt_email_provider: Option<String>,
    e_receipt_email_id: Option<String>,
    e_receipt_authenticated: bool,
    instacart_shopper: bool,
    e_receipt: bool,
    e_receipt_component_emails: Option<Vec<MicroblinkReceipt>>,
    duplicate: bool,
    fraudulent: bool,
    receipt_date_time: Option<i64>,
    duplicate_blink_receipt_ids: Option<Vec<String>>,
    merchant_match_guess: Option<StringType>,
    products_pending_lookup: i32,
    qualified_promotions: Option<Vec<Promotion>>,
    unqualified_promotions: Option<Vec<Promotion>>,
    extended_fields: Option<HashMap<String, String>>,
    e_receipt_additional_fees: Option<HashMap<String, String>>,
    purchase_type: Option<StringType>,
    loyalty_for_banner: bool,
    channel: Option<StringType>,
    submission_date: Option<i64>,
    e_receipt_fulfilled_by: Option<String>,
    e_receipt_shipping_status: Option<String>,
    #[serde(rename = "eReceiptPOSSystem")]
    e_receipt_pos_system: Option<String>,
    e_receipt_sub_merchant: Option<String>,
    qualified_surveys: Option<Vec<Survey>>,
    barcode: Option<String>,
    e_receipt_merchant_email: Option<String>,
    e_receipt_email_subject: Option<String>,
    e_receipt_shipping_costs: Option<f32>,
    currency_code: Option<String>,
    client_merchant_name: Option<String>,
    loyalty_program: bool,
    merchant_sources: Option<Vec<i32>>,
    payment_terminal_id: Option<StringType>,
    payment_transaction_id: Option<StringType>,
    combined_raw_text: Option<StringType>,
}

impl Ingest for MicroblinkReceipt {
    fn id(&self) -> String {
        self.blink_receipt_id
            .as_ref()
            .expect("Missing required field: blinkReceiptId")
            .to_string()
    }
    fn schema() -> Schema {
        let raw_schema = r#"
{
  "type": "record",
  "name": "Receipt",
  "namespace": "microblink",
  "fields": [
    {
      "name": "receiptDate",
      "type": [
        "null",
        {
          "type": "record",
          "name": "StringType",
          "namespace": "microblink.receipt",
          "fields": [
            {
              "name": "confidence",
              "type": "float"
            },
            {
              "name": "value",
              "type": [
                "null",
                "string"
              ]
            }
          ]
        }
      ]
    },
    {
      "name": "receiptTime",
      "type": [
        "null",
        "microblink.receipt.StringType"
      ]
    },
    {
      "name": "retailerId",
      "type": {
        "type": "record",
        "name": "Retailer",
        "namespace": "microblink.receipt",
        "fields": [
          {
            "name": "id",
            "type": "int"
          },
          {
            "name": "bannerId",
            "type": "int"
          }
        ]
      }
    },
    {
      "name": "products",
      "type": [
        "null",
        {
          "type": "array",
          "default": [],
          "items": {
            "type": "record",
            "name": "Product",
            "namespace": "microblink.receipt",
            "fields": [
              {
                "name": "productNumber",
                "type": [
                  "null",
                  "microblink.receipt.StringType"
                ]
              },
              {
                "name": "description",
                "type": [
                  "null",
                  "microblink.receipt.StringType"
                ]
              },
              {
                "name": "quantity",
                "type": [
                  "null",
                  {
                    "type": "record",
                    "name": "FloatType",
                    "namespace": "microblink.receipt",
                    "fields": [
                      {
                        "name": "confidence",
                        "type": "float"
                      },
                      {
                        "name": "value",
                        "type": "float"
                      }
                    ]
                  }
                ]
              },
              {
                "name": "unitPrice",
                "type": [
                  "null",
                  "microblink.receipt.FloatType"
                ]
              },
              {
                "name": "unitOfMeasure",
                "type": [
                  "null",
                  "microblink.receipt.StringType"
                ]
              },
              {
                "name": "totalPrice",
                "type": [
                  "null",
                  "microblink.receipt.FloatType"
                ]
              },
              {
                "name": "fullPrice",
                "type": "float"
              },
              {
                "name": "line",
                "type": "int"
              },
              {
                "name": "productName",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "brand",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "category",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "size",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "rewardsGroup",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "competitorRewardsGroup",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "upc",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "imageUrl",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "shippingStatus",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "additionalLines",
                "type": [
                  "null",
                  {
                    "type": "array",
                    "items": {
                      "type": "record",
                      "name": "AdditionalLine",
                      "namespace": "microblink.receipt.product",
                      "fields": [
                        {
                          "name": "type",
                          "type": [
                            "null",
                            "microblink.receipt.StringType"
                          ]
                        },
                        {
                          "name": "text",
                          "type": [
                            "null",
                            "microblink.receipt.StringType"
                          ]
                        },
                        {
                          "name": "lineNumber",
                          "type": "int"
                        }
                      ]
                    }
                  }
                ]
              },
              {
                "name": "priceAfterCoupons",
                "type": [
                  "null",
                  "microblink.receipt.FloatType"
                ]
              },
              {
                "name": "voided",
                "type": "boolean"
              },
              {
                "name": "probability",
                "type": "double"
              },
              {
                "name": "sensitive",
                "type": "boolean",
                "default": false
              },
              {
                "name": "possibleProducts",
                "type": [
                  "null",
                  {
                    "type": "array",
                    "items": "microblink.receipt.Product"
                  }
                ]
              },
              {
                "name": "subProducts",
                "type": [
                  "null",
                  {
                    "type": "array",
                    "items": "microblink.receipt.Product"
                  }
                ]
              },
              {
                "name": "added",
                "type": "boolean",
                "default": false
              },
              {
                "name": "blinkReceiptBrand",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "blinkReceiptCategory",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "extendedFields",
                "type": [
                  "null",
                  {
                    "type": "map",
                    "values": "string"
                  }
                ]
              },
              {
                "name": "fuelType",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "descriptionPrefix",
                "type": [
                  "null",
                  "microblink.receipt.StringType"
                ]
              },
              {
                "name": "descriptionPostfix",
                "type": [
                  "null",
                  "microblink.receipt.StringType"
                ]
              },
              {
                "name": "skuPrefix",
                "type": [
                  "null",
                  "microblink.receipt.StringType"
                ]
              },
              {
                "name": "skuPostfix",
                "type": [
                  "null",
                  "microblink.receipt.StringType"
                ]
              },
              {
                "name": "attributes",
                "type": [
                  "null",
                  {
                    "type": "array",
                    "items": {
                      "type": "map",
                      "values": "string"
                    }
                  }
                ]
              },
              {
                "name": "sector",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "department",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "majorCategory",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "subCategory",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "itemType",
                "type": [
                  "null",
                  "string"
                ]
              }
            ]
          }
        }
      ]
    },
    {
      "name": "coupons",
      "type": [
        "null",
        {
          "type": "array",
          "default": [],
          "items": {
            "type": "record",
            "name": "Coupon",
            "namespace": "microblink.receipt",
            "fields": [
              {
                "name": "type",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "amount",
                "type": [
                  "null",
                  "microblink.receipt.FloatType"
                ]
              },
              {
                "name": "sku",
                "type": [
                  "null",
                  "microblink.receipt.StringType"
                ]
              },
              {
                "name": "description",
                "type": [
                  "null",
                  "microblink.receipt.StringType"
                ]
              },
              {
                "name": "relatedProductIndex",
                "type": "int"
              }
            ]
          }
        }
      ]
    },
    {
      "name": "total",
      "type": [
        "null",
        "microblink.receipt.FloatType"
      ]
    },
    {
      "name": "tip",
      "type": [
        "null",
        "microblink.receipt.FloatType"
      ]
    },
    {
      "name": "subtotal",
      "type": [
        "null",
        "microblink.receipt.FloatType"
      ]
    },
    {
      "name": "taxes",
      "type": [
        "null",
        "microblink.receipt.FloatType"
      ]
    },
    {
      "name": "storeNumber",
      "type": [
        "null",
        "microblink.receipt.StringType"
      ]
    },
    {
      "name": "merchantName",
      "type": [
        "null",
        "microblink.receipt.StringType"
      ]
    },
    {
      "name": "storeAddress",
      "type": [
        "null",
        "microblink.receipt.StringType"
      ]
    },
    {
      "name": "storeCity",
      "type": [
        "null",
        "microblink.receipt.StringType"
      ]
    },
    {
      "name": "blinkReceiptId",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "storeState",
      "type": [
        "null",
        "microblink.receipt.StringType"
      ]
    },
    {
      "name": "storeZip",
      "type": [
        "null",
        "microblink.receipt.StringType"
      ]
    },
    {
      "name": "storeCountry",
      "type": [
        "null",
        "microblink.receipt.StringType"
      ]
    },
    {
      "name": "storePhone",
      "type": [
        "null",
        "microblink.receipt.StringType"
      ]
    },
    {
      "name": "cashierId",
      "type": [
        "null",
        "microblink.receipt.StringType"
      ]
    },
    {
      "name": "transactionId",
      "type": [
        "null",
        "microblink.receipt.StringType"
      ]
    },
    {
      "name": "registerId",
      "type": [
        "null",
        "microblink.receipt.StringType"
      ]
    },
    {
      "name": "paymentMethods",
      "type": [
        "null",
        {
          "type": "array",
          "default": [],
          "items": {
            "type": "record",
            "name": "PaymentMethod",
            "namespace": "microblink.receipt",
            "fields": [
              {
                "name": "paymentMethod",
                "type": [
                  "null",
                  "microblink.receipt.StringType"
                ]
              },
              {
                "name": "cardType",
                "type": [
                  "null",
                  "microblink.receipt.StringType"
                ]
              },
              {
                "name": "cardIssuer",
                "type": [
                  "null",
                  "microblink.receipt.StringType"
                ]
              },
              {
                "name": "amount",
                "type": [
                  "null",
                  "microblink.receipt.FloatType"
                ]
              }
            ]
          }
        }
      ]
    },
    {
      "name": "taxId",
      "type": [
        "null",
        "microblink.receipt.StringType"
      ]
    },
    {
      "name": "mallName",
      "type": [
        "null",
        "microblink.receipt.StringType"
      ]
    },
    {
      "name": "last4cc",
      "type": [
        "null",
        "microblink.receipt.StringType"
      ]
    },
    {
      "name": "ocrConfidence",
      "type": "float"
    },
    {
      "name": "merchantSource",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "foundTopEdge",
      "type": "boolean",
      "default": false
    },
    {
      "name": "foundBottomEdge",
      "type": "boolean",
      "default": false
    },
    {
      "name": "eReceiptOrderNumber",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "eReceiptOrderStatus",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "eReceiptRawHtml",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "eReceiptShippingAddress",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "shipments",
      "type": [
        "null",
        {
          "type": "array",
          "default": [],
          "items": {
            "type": "record",
            "name": "Shipment",
            "namespace": "microblink.receipt",
            "fields": [
              {
                "name": "status",
                "type": "string"
              },
              {
                "name": "products",
                "type": [
                  "null",
                  {
                    "type": "array",
                    "default": [],
                    "items": "microblink.receipt.Product"
                  }
                ]
              }
            ]
          }
        }
      ]
    },
    {
      "name": "longTransactionId",
      "type": [
        "null",
        "microblink.receipt.StringType"
      ]
    },
    {
      "name": "subtotalMatches",
      "type": "boolean",
      "default": false
    },
    {
      "name": "eReceiptEmailProvider",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "eReceiptEmailId",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "eReceiptAuthenticated",
      "type": "boolean",
      "default": false
    },
    {
      "name": "instacartShopper",
      "type": "boolean",
      "default": false
    },
    {
      "name": "eReceipt",
      "type": "boolean",
      "default": false
    },
    {
      "name": "eReceiptComponentEmails",
      "type": [
        "null",
        {
          "type": "array",
          "default": [],
          "items": "microblink.Receipt"
        }
      ]
    },
    {
      "name": "duplicate",
      "type": "boolean",
      "default": false
    },
    {
      "name": "fraudulent",
      "type": "boolean",
      "default": false
    },
    {
      "name": "receiptDateTime",
      "type": [
        "null",
        "long"
      ]
    },
    {
      "name": "duplicateBlinkReceiptIds",
      "type": [
        "null",
        {
          "type": "array",
          "default": [],
          "items": "string"
        }
      ]
    },
    {
      "name": "merchantMatchGuess",
      "type": [
        "null",
        "microblink.receipt.StringType"
      ]
    },
    {
      "name": "productsPendingLookup",
      "type": "int"
    },
    {
      "name": "qualifiedPromotions",
      "type": [
        "null",
        {
          "type": "array",
          "default": [],
          "items": {
            "type": "record",
            "name": "Promotion",
            "namespace": "microblink.receipt",
            "fields": [
              {
                "name": "id",
                "type": "long"
              },
              {
                "name": "slug",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "reward",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "rewardCurrency",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "errorCode",
                "type": "int"
              },
              {
                "name": "errorMessage",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "relatedProductIndexes",
                "type": [
                  "null",
                  {
                    "type": "array",
                    "default": [],
                    "items": "int"
                  }
                ]
              },
              {
                "name": "qualifications",
                "type": [
                  "null",
                  {
                    "type": "array",
                    "default": [],
                    "items": {
                      "type": "array",
                      "default": [],
                      "items": "int"
                    }
                  }
                ]
              }
            ]
          }
        }
      ]
    },
    {
      "name": "unqualifiedPromotions",
      "type": [
        "null",
        {
          "type": "array",
          "default": [],
          "items": "microblink.receipt.Promotion"
        }
      ]
    },
    {
      "name": "extendedFields",
      "type": [
        "null",
        {
          "type": "map",
          "values": "string"
        }
      ]
    },
    {
      "name": "eReceiptAdditionalFees",
      "type": [
        "null",
        {
          "type": "map",
          "values": "string"
        }
      ]
    },
    {
      "name": "purchaseType",
      "type": [
        "null",
        "microblink.receipt.StringType"
      ]
    },
    {
      "name": "loyaltyForBanner",
      "type": "boolean",
      "default": false
    },
    {
      "name": "channel",
      "type": [
        "null",
        "microblink.receipt.StringType"
      ]
    },
    {
      "name": "submissionDate",
      "type": [
        "null",
        "long"
      ]
    },
    {
      "name": "eReceiptFulfilledBy",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "eReceiptShippingStatus",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "eReceiptPOSSystem",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "eReceiptSubMerchant",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "qualifiedSurveys",
      "type": [
        "null",
        {
          "type": "array",
          "default": [],
          "items": {
            "type": "record",
            "name": "Survey",
            "namespace": "microblink.receipt",
            "fields": [
              {
                "name": "clientUserId",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "serverId",
                "type": "int"
              },
              {
                "name": "slug",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "rewardValue",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "startDate",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "endDate",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "questions",
                "type": [
                  "null",
                  {
                    "type": "array",
                    "default": [],
                    "items": {
                      "type": "record",
                      "name": "SurveyQuestion",
                      "namespace": "microblink.receipt.survey",
                      "fields": [
                        {
                          "name": "myIndex",
                          "type": "int"
                        },
                        {
                          "name": "lastQuestion",
                          "type": "boolean",
                          "default": false
                        },
                        {
                          "name": "nextQuestionIndex",
                          "type": "int"
                        },
                        {
                          "name": "serverId",
                          "type": "int"
                        },
                        {
                          "name": "text",
                          "type": [
                            "null",
                            "string"
                          ]
                        },
                        {
                          "name": "type",
                          "type": [
                            "null",
                            "string"
                          ]
                        },
                        {
                          "name": "answers",
                          "type": [
                            "null",
                            {
                              "type": "array",
                              "default": [],
                              "items": {
                                "type": "record",
                                "name": "SurveyAnswer",
                                "namespace": "microblink.receipt.survey",
                                "fields": [
                                  {
                                    "name": "id",
                                    "type": "int"
                                  },
                                  {
                                    "name": "text",
                                    "type": [
                                      "null",
                                      "string"
                                    ]
                                  },
                                  {
                                    "name": "nextQuestionIndex",
                                    "type": [
                                      "null",
                                      "int"
                                    ]
                                  }
                                ]
                              }
                            }
                          ]
                        },
                        {
                          "name": "multipleAnswers",
                          "type": "boolean",
                          "default": false
                        },
                        {
                          "name": "totalNumberOfQuestions",
                          "type": "int"
                        }
                      ]
                    }
                  }
                ]
              }
            ]
          }
        }
      ]
    },
    {
      "name": "barcode",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "eReceiptMerchantEmail",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "eReceiptEmailSubject",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "eReceiptShippingCosts",
      "type": [
        "null",
        "float"
      ]
    },
    {
      "name": "currencyCode",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "clientMerchantName",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "loyaltyProgram",
      "type": "boolean",
      "default": false
    },
    {
      "name": "merchantSources",
      "type": [
        "null",
        {
          "type": "array",
          "default": [],
          "items": "int"
        }
      ]
    },
    {
      "name": "paymentTerminalId",
      "type": [
        "null",
        "microblink.receipt.StringType"
      ]
    },
    {
      "name": "paymentTransactionId",
      "type": [
        "null",
        "microblink.receipt.StringType"
      ]
    },
    {
      "name": "combinedRawText",
      "type": [
        "null",
        "microblink.receipt.StringType"
      ]
    }
  ]
}

"#;
        Schema::parse_str(raw_schema).unwrap()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FloatType {
    confidence: f32,
    value: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StringType {
    confidence: f32,
    value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalLine {
    #[serde(rename = "type")]
    r#type: Option<StringType>,
    text: Option<StringType>,
    line_number: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coupon {
    #[serde(rename = "type")]
    r#type: Option<String>,
    amount: Option<FloatType>,
    sku: Option<StringType>,
    description: Option<StringType>,
    related_product_index: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentMethod {
    payment_method: Option<StringType>,
    card_type: Option<StringType>,
    card_issuer: Option<StringType>,
    amount: Option<FloatType>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Promotion {
    id: i64,
    slug: Option<String>,
    reward: Option<String>,
    reward_currency: Option<String>,
    error_code: i32,
    error_message: Option<String>,
    related_product_indexes: Option<Vec<i32>>,
    qualifications: Option<Vec<Vec<i32>>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Retailer {
    id: i32,
    banner_id: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurveyAnswer {
    id: i32,
    text: Option<String>,
    next_question_index: Option<i32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurveyQuestion {
    my_index: i32,
    last_question: bool,
    next_question_index: i32,
    server_id: i32,
    text: Option<String>,
    #[serde(rename = "type")]
    r#type: Option<String>,
    answers: Option<Vec<SurveyAnswer>>,
    multiple_answers: bool,
    total_number_of_questions: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Survey {
    client_user_id: Option<String>,
    server_id: i32,
    slug: Option<String>,
    reward_value: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    questions: Option<Vec<SurveyQuestion>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    product_number: Option<StringType>,
    description: Option<StringType>,
    quantity: Option<FloatType>,
    unit_price: Option<FloatType>,
    unit_of_measure: Option<StringType>,
    total_price: Option<FloatType>,
    full_price: f32,
    line: i32,
    product_name: Option<String>,
    brand: Option<String>,
    category: Option<String>,
    size: Option<String>,
    rewards_group: Option<String>,
    competitor_rewards_group: Option<String>,
    upc: Option<String>,
    image_url: Option<String>,
    shipping_status: Option<String>,
    additional_lines: Option<Vec<AdditionalLine>>,
    price_after_coupons: Option<FloatType>,
    voided: bool,
    probability: f64,
    sensitive: bool,
    possible_products: Option<Vec<Product>>,
    sub_products: Option<Vec<Product>>,
    added: bool,
    blink_receipt_brand: Option<String>,
    blink_receipt_category: Option<String>,
    extended_fields: Option<HashMap<String, String>>,
    fuel_type: Option<String>,
    description_prefix: Option<StringType>,
    description_postfix: Option<StringType>,
    sku_prefix: Option<StringType>,
    sku_postfix: Option<StringType>,
    attributes: Option<Vec<HashMap<String, String>>>,
    sector: Option<String>,
    department: Option<String>,
    major_category: Option<String>,
    sub_category: Option<String>,
    item_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shipment {
    status: String,
    products: Option<Vec<Product>>,
}
