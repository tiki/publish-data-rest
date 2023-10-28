/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use apache_avro::Schema;
use ingest_lib::Ingest;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShopifyOrder {
    pub app_id: Option<u64>,
    pub billing_address: Option<Address>,
    pub browser_ip: Option<String>,
    pub buyer_accepts_marketing: Option<bool>,
    pub cancel_reason: Option<String>,
    pub cancelled_at: Option<String>,
    pub cart_token: Option<String>,
    pub checkout_token: Option<String>,
    pub client_details: Option<ClientDetails>,
    pub closed_at: Option<String>,
    pub company: Option<String>,
    pub created_at: Option<String>,
    pub currency: Option<String>,
    pub current_total_additional_fees_set: Option<PriceSet>,
    pub current_total_discounts: Option<String>,
    pub current_total_discounts_set: Option<PriceSet>,
    pub current_total_duties_set: Option<PriceSet>,
    pub current_total_price: Option<String>,
    pub current_total_price_set: Option<PriceSet>,
    pub current_subtotal_price: Option<String>,
    pub current_subtotal_price_set: Option<PriceSet>,
    pub current_total_tax: Option<String>,
    pub current_total_tax_set: Option<PriceSet>,
    pub customer: Option<Customer>,
    pub customer_locale: Option<String>,
    pub discount_applications: Option<Vec<DiscountApplication>>,
    pub discount_codes: Option<Vec<DiscountCode>>,
    pub email: Option<String>,
    pub estimated_taxes: Option<bool>,
    pub financial_status: Option<String>,
    pub fulfillments: Option<Vec<Fulfillment>>,
    pub fulfillment_status: Option<String>,
    pub gateway: Option<String>,
    pub id: u64,
    pub landing_site: Option<String>,
    pub line_items: Option<Vec<LineItem>>,
    pub location_id: Option<u64>,
    pub merchant_of_record_app_id: Option<u64>,
    pub name: Option<String>,
    pub note: Option<String>,
    pub note_attributes: Option<Vec<NoteAttribute>>,
    pub number: Option<u64>,
    pub order_number: Option<u64>,
    pub original_total_additional_fees_set: Option<PriceSet>,
    pub original_total_duties_set: Option<PriceSet>,
    pub payment_terms: Option<PaymentTerm>,
    pub payment_gateway_names: Option<Vec<String>>,
    pub phone: Option<String>,
    pub presentment_currency: Option<String>,
    pub processed_at: Option<String>,
    pub referring_site: Option<String>,
    pub refunds: Option<Vec<Refund>>,
    pub shipping_address: Option<Address>,
    pub shipping_lines: Option<Vec<ShippingLine>>,
    pub source_name: Option<String>,
    pub source_identifier: Option<String>,
    pub source_url: Option<String>,
    pub subtotal_price: Option<String>,
    pub subtotal_price_set: Option<PriceSet>,
    pub tags: Option<String>,
    pub tax_lines: Option<Vec<TaxLine>>,
    pub taxes_included: Option<bool>,
    pub test: Option<bool>,
    pub token: Option<String>,
    pub total_discounts: Option<String>,
    pub total_discounts_set: Option<PriceSet>,
    pub total_line_items_price: Option<String>,
    pub total_line_items_price_set: Option<PriceSet>,
    pub total_outstanding: Option<String>,
    pub total_price: Option<String>,
    pub total_price_set: Option<PriceSet>,
    pub total_shipping_price_set: Option<PriceSet>,
    pub total_tax: Option<String>,
    pub total_tax_set: Option<PriceSet>,
    pub total_tip_received: Option<String>,
    pub total_weight: Option<u64>,
    pub updated_at: Option<String>,
    pub user_id: Option<u64>,
    pub order_status_url: Option<String>,
}

impl Ingest for ShopifyOrder {
    fn id(&self) -> String {
        self.id.to_string()
    }
    fn schema() -> Schema {
        let raw_schema = r#"
{
  "type": "record",
  "name": "Order",
  "namespace": "shopify.order",
  "fields": [
    {
      "name": "app_id",
      "type": [
        "null",
        "long"
      ]
    },
    {
      "name": "billing_address",
      "type": [
        "null",
        {
          "type": "record",
          "name": "Address",
          "namespace": "shopify.order.address",
          "fields": [
            {
              "name": "address1",
              "type": [
                "null",
                "string"
              ]
            },
            {
              "name": "address2",
              "type": [
                "null",
                "string"
              ]
            },
            {
              "name": "city",
              "type": [
                "null",
                "string"
              ]
            },
            {
              "name": "company",
              "type": [
                "null",
                "string"
              ]
            },
            {
              "name": "country",
              "type": [
                "null",
                "string"
              ]
            },
            {
              "name": "first_name",
              "type": [
                "null",
                "string"
              ]
            },
            {
              "name": "last_name",
              "type": [
                "null",
                "string"
              ]
            },
            {
              "name": "phone",
              "type": [
                "null",
                "string"
              ]
            },
            {
              "name": "province",
              "type": [
                "null",
                "string"
              ]
            },
            {
              "name": "zip",
              "type": [
                "null",
                "string"
              ]
            },
            {
              "name": "name",
              "type": [
                "null",
                "string"
              ]
            },
            {
              "name": "province_code",
              "type": [
                "null",
                "string"
              ]
            },
            {
              "name": "country_code",
              "type": [
                "null",
                "string"
              ]
            },
            {
              "name": "country_name",
              "type": [
                "null",
                "string"
              ]
            },
            {
              "name": "latitude",
              "type": [
                "null",
                "string"
              ]
            },
            {
              "name": "longitude",
              "type": [
                "null",
                "string"
              ]
            },
            {
              "name": "default",
              "type": [
                "null",
                "boolean"
              ]
            },
            {
              "name": "id",
              "type": [
                "null",
                "long"
              ]
            },
            {
              "name": "customer_id",
              "type": [
                "null",
                "long"
              ]
            }
          ]
        }
      ]
    },
    {
      "name": "browser_ip",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "buyer_accepts_marketing",
      "type": [
        "null",
        "boolean"
      ]
    },
    {
      "name": "cancel_reason",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "cancelled_at",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "cart_token",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "checkout_token",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "client_details",
      "type": [
        "null",
        {
          "type": "record",
          "name": "ClientDetails",
          "namespace": "shopify.order.client_details",
          "fields": [
            {
              "name": "accept_language",
              "type": [
                "null",
                "string"
              ]
            },
            {
              "name": "browser_height",
              "type": [
                "null",
                "int"
              ]
            },
            {
              "name": "browser_ip",
              "type": [
                "null",
                "string"
              ]
            },
            {
              "name": "browser_width",
              "type": [
                "null",
                "int"
              ]
            },
            {
              "name": "session_hash",
              "type": [
                "null",
                "string"
              ]
            },
            {
              "name": "user_agent",
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
      "name": "closed_at",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "company",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "created_at",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "currency",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "current_total_additional_fees_set",
      "type": [
        "null",
        {
          "type": "record",
          "name": "PriceSet",
          "namespace": "shopify.order.price_set",
          "fields": [
            {
              "name": "shop_money",
              "type": [
                "null",
                {
                  "type": "record",
                  "name": "PriceSetMoney",
                  "namespace": "shopify.order.price_set.price_set_money",
                  "fields": [
                    {
                      "name": "amount",
                      "type": [
                        "null",
                        "string"
                      ]
                    },
                    {
                      "name": "currency_code",
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
              "name": "presentment_money",
              "type": [
                "null",
                "shopify.order.price_set.price_set_money.PriceSetMoney"
              ]
            }
          ]
        }
      ]
    },
    {
      "name": "current_total_discounts",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "current_total_discounts_set",
      "type": [
        "null",
        "shopify.order.price_set.PriceSet"
      ]
    },
    {
      "name": "current_total_duties_set",
      "type": [
        "null",
        "shopify.order.price_set.PriceSet"
      ]
    },
    {
      "name": "current_total_price",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "current_total_price_set",
      "type": [
        "null",
        "shopify.order.price_set.PriceSet"
      ]
    },
    {
      "name": "current_subtotal_price",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "current_subtotal_price_set",
      "type": [
        "null",
        "shopify.order.price_set.PriceSet"
      ]
    },
    {
      "name": "current_total_tax",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "current_total_tax_set",
      "type": [
        "null",
        "shopify.order.price_set.PriceSet"
      ]
    },
    {
      "name": "customer",
      "type": [
        "null",
        {
          "type": "record",
          "name": "Customer",
          "namespace": "shopify.order.customer",
          "fields": [
            {
              "name": "accepts_marketing",
              "type": [
                "null",
                "boolean"
              ]
            },
            {
              "name": "accepts_marketing_updated_at",
              "type": [
                "null",
                "string"
              ]
            },
            {
              "name": "addresses",
              "type": [
                "null",
                {
                  "type": "array",
                  "default": [],
                  "items": "shopify.order.address.Address"
                }
              ]
            },
            {
              "name": "currency",
              "type": [
                "null",
                "string"
              ]
            },
            {
              "name": "created_at",
              "type": [
                "null",
                "string"
              ]
            },
            {
              "name": "default_address",
              "type": [
                "null",
                "shopify.order.address.Address"
              ]
            },
            {
              "name": "email",
              "type": [
                "null",
                "string"
              ]
            },
            {
              "name": "email_marketing_consent",
              "type": [
                "null",
                {
                  "type": "record",
                  "name": "MarketingConsent",
                  "namespace": "shopify.order.customer.marketing_consent",
                  "fields": [
                    {
                      "name": "state",
                      "type": [
                        "null",
                        "string"
                      ]
                    },
                    {
                      "name": "opt_in_level",
                      "type": [
                        "null",
                        "string"
                      ]
                    },
                    {
                      "name": "consent_updated_at",
                      "type": [
                        "null",
                        "string"
                      ]
                    },
                    {
                      "name": "consent_collected_from",
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
              "name": "first_name",
              "type": [
                "null",
                "string"
              ]
            },
            {
              "name": "id",
              "type": [
                "null",
                "long"
              ]
            },
            {
              "name": "last_name",
              "type": [
                "null",
                "string"
              ]
            },
            {
              "name": "last_order_id",
              "type": [
                "null",
                "long"
              ]
            },
            {
              "name": "last_order_name",
              "type": [
                "null",
                "string"
              ]
            },
            {
              "name": "metafield",
              "type": [
                "null",
                {
                  "type": "record",
                  "name": "Metafield",
                  "namespace": "shopify.order.customer.metafield",
                  "fields": [
                    {
                      "name": "key",
                      "type": [
                        "null",
                        "string"
                      ]
                    },
                    {
                      "name": "namespace",
                      "type": [
                        "null",
                        "string"
                      ]
                    },
                    {
                      "name": "value",
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
                    }
                  ]
                }
              ]
            },
            {
              "name": "marketing_opt_in_level",
              "type": [
                "null",
                "string"
              ]
            },
            {
              "name": "note",
              "type": [
                "null",
                "string"
              ]
            },
            {
              "name": "orders_count",
              "type": [
                "null",
                "long"
              ]
            },
            {
              "name": "phone",
              "type": [
                "null",
                "string"
              ]
            },
            {
              "name": "sms_marketing_consent",
              "type": [
                "null",
                "shopify.order.customer.marketing_consent.MarketingConsent"
              ]
            },
            {
              "name": "state",
              "type": [
                "null",
                "string"
              ]
            },
            {
              "name": "tags",
              "type": [
                "null",
                "string"
              ]
            },
            {
              "name": "tax_exempt",
              "type": [
                "null",
                "boolean"
              ]
            },
            {
              "name": "tax_exemptions",
              "type": [
                "null",
                {
                  "type": "array",
                  "items": "string",
                  "default": []
                }
              ]
            },
            {
              "name": "total_spent",
              "type": [
                "null",
                "string"
              ]
            },
            {
              "name": "updated_at",
              "type": [
                "null",
                "string"
              ]
            },
            {
              "name": "verified_email",
              "type": [
                "null",
                "boolean"
              ]
            }
          ]
        }
      ]
    },
    {
      "name": "customer_locale",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "discount_applications",
      "type": [
        "null",
        {
          "type": "array",
          "default": [],
          "items": {
            "type": "record",
            "name": "DiscountApplication",
            "namespace": "shopify.order.discount_application",
            "fields": [
              {
                "name": "type",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "title",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "description",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "value",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "value_type",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "allocation_method",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "target_selection",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "target_type",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "code",
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
      "name": "discount_codes",
      "type": [
        "null",
        {
          "type": "array",
          "default": [],
          "items": {
            "type": "record",
            "namespace": "shopify.order.discount_code",
            "name": "DiscountCode",
            "fields": [
              {
                "name": "code",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "amount",
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
              }
            ]
          }
        }
      ]
    },
    {
      "name": "email",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "estimated_taxes",
      "type": [
        "null",
        "boolean"
      ]
    },
    {
      "name": "financial_status",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "fulfillment_status",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "gateway",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "id",
      "type": "long"
    },
    {
      "name": "landing_site",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "tax_lines",
      "type": [
        "null",
        {
          "type": "array",
          "default": [],
          "items": {
            "type": "record",
            "name": "TaxLine",
            "namespace": "shopify.order.tax_line",
            "fields": [
              {
                "name": "title",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "price",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "rate",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "price_set",
                "type": [
                  "null",
                  "shopify.order.price_set.PriceSet"
                ]
              }
            ]
          }
        }
      ]
    },
    {
      "name": "line_items",
      "type": [
        "null",
        {
          "type": "array",
          "default": [],
          "items": {
            "type": "record",
            "name": "LineItem",
            "namespace": "shopify.order.line_item",
            "fields": [
              {
                "name": "id",
                "type": [
                  "null",
                  "long"
                ]
              },
              {
                "name": "variant_id",
                "type": [
                  "null",
                  "long"
                ]
              },
              {
                "name": "title",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "quantity",
                "type": [
                  "null",
                  "long"
                ]
              },
              {
                "name": "price",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "grams",
                "type": [
                  "null",
                  "long"
                ]
              },
              {
                "name": "sku",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "variant_title",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "vendor",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "fulfillment_service",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "product_id",
                "type": [
                  "null",
                  "long"
                ]
              },
              {
                "name": "requires_shipping",
                "type": [
                  "null",
                  "boolean"
                ]
              },
              {
                "name": "taxable",
                "type": [
                  "null",
                  "boolean"
                ]
              },
              {
                "name": "gift_card",
                "type": [
                  "null",
                  "boolean"
                ]
              },
              {
                "name": "name",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "variant_inventory_management",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "properties",
                "type": [
                  "null",
                  {
                    "type": "array",
                    "default": [],
                    "items": {
                      "type": "record",
                      "name": "Property",
                      "namespace": "shopify.order.line_item.property",
                      "fields": [
                        {
                          "name": "name",
                          "type": [
                            "null",
                            "string"
                          ]
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
                  }
                ]
              },
              {
                "name": "product_exists",
                "type": [
                  "null",
                  "boolean"
                ]
              },
              {
                "name": "fulfillable_quantity",
                "type": [
                  "null",
                  "long"
                ]
              },
              {
                "name": "total_discount",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "fulfillment_status",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "fulfillment_line_item_id",
                "type": [
                  "null",
                  "long"
                ]
              },
              {
                "name": "tax_lines",
                "type": [
                  "null",
                  {
                    "type": "array",
                    "default": [],
                    "items": "shopify.order.tax_line.TaxLine"
                  }
                ]
              },
              {
                "name": "duties",
                "type": [
                  "null",
                  {
                    "type": "array",
                    "default": [],
                    "items": {
                      "type": "record",
                      "name": "Duty",
                      "namespace": "shopify.order.line_item.duty",
                      "fields": [
                        {
                          "name": "id",
                          "type": [
                            "null",
                            "long"
                          ]
                        },
                        {
                          "name": "harmonized_system_code",
                          "type": [
                            "null",
                            "string"
                          ]
                        },
                        {
                          "name": "country_code_of_origin",
                          "type": [
                            "null",
                            "string"
                          ]
                        },
                        {
                          "name": "shop_money",
                          "type": [
                            "null",
                            "shopify.order.price_set.price_set_money.PriceSetMoney"
                          ]
                        },
                        {
                          "name": "presentment_money",
                          "type": [
                            "null",
                            "shopify.order.price_set.price_set_money.PriceSetMoney"
                          ]
                        },
                        {
                          "name": "tax_lines",
                          "type": [
                            "null",
                            {
                              "type": "array",
                              "default": [],
                              "items": "shopify.order.tax_line.TaxLine"
                            }
                          ]
                        },
                        {
                          "name": "admin_graphql_api_id",
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
                "name": "attributed_staffs",
                "type": [
                  "null",
                  {
                    "type": "array",
                    "default": [],
                    "items": {
                      "type": "record",
                      "name": "Staff",
                      "namespace": "shopify.order.line_item.staff",
                      "fields": [
                        {
                          "name": "id",
                          "type": [
                            "null",
                            "string"
                          ]
                        },
                        {
                          "name": "quantity",
                          "type": [
                            "null",
                            "long"
                          ]
                        }
                      ]
                    }
                  }
                ]
              },
              {
                "name": "price_set",
                "type": [
                  "null",
                  "shopify.order.price_set.PriceSet"
                ]
              },
              {
                "name": "total_discount_set",
                "type": [
                  "null",
                  "shopify.order.price_set.PriceSet"
                ]
              },
              {
                "name": "discount_allocations",
                "type": [
                  "null",
                  {
                    "type": "array",
                    "default": [],
                    "items": {
                      "type": "record",
                      "name": "DiscountAllocation",
                      "namespace": "shopify.order.line_item.discount_allocation",
                      "fields": [
                        {
                          "name": "amount",
                          "type": [
                            "null",
                            "string"
                          ]
                        },
                        {
                          "name": "discount_application_index",
                          "type": [
                            "null",
                            "long"
                          ]
                        },
                        {
                          "name": "amount_set",
                          "type": [
                            "null",
                            "shopify.order.price_set.PriceSet"
                          ]
                        }
                      ]
                    }
                  }
                ]
              },
              {
                "name": "origin_location",
                "type": [
                  "null",
                  "shopify.order.address.Address"
                ]
              }
            ]
          }
        }
      ]
    },
    {
      "name": "fulfillments",
      "type": [
        "null",
        {
          "type": "array",
          "default": [],
          "items": {
            "type": "record",
            "name": "Fulfillment",
            "namespace": "shopify.order.fulfillment",
            "fields": [
              {
                "name": "created_at",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "id",
                "type": [
                  "null",
                  "long"
                ]
              },
              {
                "name": "line_items",
                "type": [
                  "null",
                  {
                    "type": "array",
                    "default": [],
                    "items": "shopify.order.line_item.LineItem"
                  }
                ]
              },
              {
                "name": "location_id",
                "type": [
                  "null",
                  "long"
                ]
              },
              {
                "name": "name",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "notify_customer",
                "type": [
                  "null",
                  "boolean"
                ]
              },
              {
                "name": "order_id",
                "type": [
                  "null",
                  "long"
                ]
              },
              {
                "name": "origin_address",
                "type": [
                  "null",
                  "shopify.order.address.Address"
                ]
              },
              {
                "name": "receipt",
                "type": [
                  "null",
                  {
                    "type": "record",
                    "name": "Receipt",
                    "namespace": "shopify.order.fulfillment.receipt",
                    "fields": [
                      {
                        "name": "id",
                        "type": [
                          "null",
                          "long"
                        ]
                      },
                      {
                        "name": "variant_id",
                        "type": [
                          "null",
                          "long"
                        ]
                      },
                      {
                        "name": "title",
                        "type": [
                          "null",
                          "string"
                        ]
                      },
                      {
                        "name": "quantity",
                        "type": [
                          "null",
                          "long"
                        ]
                      },
                      {
                        "name": "price",
                        "type": [
                          "null",
                          "string"
                        ]
                      },
                      {
                        "name": "grams",
                        "type": [
                          "null",
                          "long"
                        ]
                      },
                      {
                        "name": "sku",
                        "type": [
                          "null",
                          "string"
                        ]
                      },
                      {
                        "name": "variant_title",
                        "type": [
                          "null",
                          "string"
                        ]
                      },
                      {
                        "name": "vendor",
                        "type": [
                          "null",
                          "string"
                        ]
                      },
                      {
                        "name": "fulfillment_service",
                        "type": [
                          "null",
                          "string"
                        ]
                      },
                      {
                        "name": "product_id",
                        "type": [
                          "null",
                          "long"
                        ]
                      },
                      {
                        "name": "requires_shipping",
                        "type": [
                          "null",
                          "boolean"
                        ]
                      },
                      {
                        "name": "taxable",
                        "type": [
                          "null",
                          "boolean"
                        ]
                      },
                      {
                        "name": "gift_card",
                        "type": [
                          "null",
                          "boolean"
                        ]
                      },
                      {
                        "name": "name",
                        "type": [
                          "null",
                          "string"
                        ]
                      },
                      {
                        "name": "variant_inventory_management",
                        "type": [
                          "null",
                          "string"
                        ]
                      },
                      {
                        "name": "properties",
                        "type": [
                          "null",
                          {
                            "type": "array",
                            "default": [],
                            "items": "shopify.order.line_item.property.Property"
                          }
                        ]
                      },
                      {
                        "name": "product_exists",
                        "type": [
                          "null",
                          "boolean"
                        ]
                      },
                      {
                        "name": "fulfillable_quantity",
                        "type": [
                          "null",
                          "long"
                        ]
                      },
                      {
                        "name": "total_discount",
                        "type": [
                          "null",
                          "string"
                        ]
                      },
                      {
                        "name": "fulfillment_status",
                        "type": [
                          "null",
                          "string"
                        ]
                      },
                      {
                        "name": "fulfillment_line_item_id",
                        "type": [
                          "null",
                          "long"
                        ]
                      },
                      {
                        "name": "tax_lines",
                        "type": [
                          "null",
                          {
                            "type": "array",
                            "default": [],
                            "items": "shopify.order.tax_line.TaxLine"
                          }
                        ]
                      },
                      {
                        "name": "duties",
                        "type": [
                          "null",
                          {
                            "type": "array",
                            "default": [],
                            "items": "shopify.order.line_item.duty.Duty"
                          }
                        ]
                      },
                      {
                        "name": "attributed_staffs",
                        "type": [
                          "null",
                          {
                            "type": "array",
                            "default": [],
                            "items": "shopify.order.line_item.staff.Staff"
                          }
                        ]
                      },
                      {
                        "name": "price_set",
                        "type": [
                          "null",
                          "shopify.order.price_set.PriceSet"
                        ]
                      },
                      {
                        "name": "total_discount_set",
                        "type": [
                          "null",
                          "shopify.order.price_set.PriceSet"
                        ]
                      },
                      {
                        "name": "discount_allocations",
                        "type": [
                          "null",
                          {
                            "type": "array",
                            "default": [],
                            "items": "shopify.order.line_item.discount_allocation.DiscountAllocation"
                          }
                        ]
                      },
                      {
                        "name": "origin_location",
                        "type": [
                          "null",
                          "shopify.order.address.Address"
                        ]
                      }
                    ]
                  }
                ]
              },
              {
                "name": "service",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "shipment_status",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "status",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "tracking_company",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "tracking_numbers",
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
                "name": "tracking_number",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "tracking_urls",
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
                "name": "tracking_url",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "updated_at",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "variant_inventory_management",
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
      "name": "location_id",
      "type": [
        "null",
        "long"
      ]
    },
    {
      "name": "merchant_of_record_app_id",
      "type": [
        "null",
        "long"
      ]
    },
    {
      "name": "name",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "note",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "note_attributes",
      "type": [
        "null",
        {
          "type": "array",
          "default": [],
          "items": {
            "type": "record",
            "name": "NoteAttribute",
            "namespace": "shopify.order.note_attribute",
            "fields": [
              {
                "name": "name",
                "type": [
                  "null",
                  "string"
                ]
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
        }
      ]
    },
    {
      "name": "number",
      "type": [
        "null",
        "long"
      ]
    },
    {
      "name": "order_number",
      "type": [
        "null",
        "long"
      ]
    },
    {
      "name": "original_total_additional_fees_set",
      "type": [
        "null",
        "shopify.order.price_set.PriceSet"
      ]
    },
    {
      "name": "original_total_duties_set",
      "type": [
        "null",
        "shopify.order.price_set.PriceSet"
      ]
    },
    {
      "name": "payment_terms",
      "type": [
        "null",
        {
          "type": "record",
          "name": "PaymentTerm",
          "namespace": "shopify.order.payment_term",
          "fields": [
            {
              "name": "amount",
              "type": [
                "null",
                "string"
              ]
            },
            {
              "name": "currency",
              "type": [
                "null",
                "string"
              ]
            },
            {
              "name": "payment_terms_name",
              "type": [
                "null",
                "string"
              ]
            },
            {
              "name": "payment_terms_type",
              "type": [
                "null",
                "string"
              ]
            },
            {
              "name": "due_in_days",
              "type": [
                "null",
                "long"
              ]
            },
            {
              "name": "payment_schedules",
              "type": [
                "null",
                {
                  "type": "array",
                  "default": [],
                  "items": {
                    "type": "record",
                    "name": "PaymentSchedule",
                    "namespace": "shopify.order.payment_term.payment_schedule",
                    "fields": [
                      {
                        "name": "amount",
                        "type": [
                          "null",
                          "string"
                        ]
                      },
                      {
                        "name": "currency",
                        "type": [
                          "null",
                          "string"
                        ]
                      },
                      {
                        "name": "issued_at",
                        "type": [
                          "null",
                          "string"
                        ]
                      },
                      {
                        "name": "due_at",
                        "type": [
                          "null",
                          "string"
                        ]
                      },
                      {
                        "name": "completed_at",
                        "type": [
                          "null",
                          "string"
                        ]
                      },
                      {
                        "name": "expected_payment_method",
                        "type": [
                          "null",
                          "string"
                        ]
                      }
                    ]
                  }
                }
              ]
            }
          ]
        }
      ]
    },
    {
      "name": "payment_gateway_names",
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
      "name": "phone",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "presentment_currency",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "processed_at",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "referring_site",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "refunds",
      "type": [
        "null",
        {
          "type": "array",
          "default": [],
          "items": {
            "type": "record",
            "namespace": "shopify.order.refund",
            "name": "Refund",
            "fields": [
              {
                "name": "created_at",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "duties",
                "type": [
                  "null",
                  {
                    "type": "array",
                    "default": [],
                    "items": "shopify.order.line_item.duty.Duty"
                  }
                ]
              },
              {
                "name": "id",
                "type": [
                  "null",
                  "long"
                ]
              },
              {
                "name": "note",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "order_adjustments",
                "type": [
                  "null",
                  {
                    "type": "array",
                    "default": [],
                    "items": {
                      "type": "record",
                      "name": "OrderAdjustment",
                      "namespace": "shopify.order.refund.order_adjustment",
                      "fields": [
                        {
                          "name": "id",
                          "type": [
                            "null",
                            "long"
                          ]
                        },
                        {
                          "name": "order_id",
                          "type": [
                            "null",
                            "long"
                          ]
                        },
                        {
                          "name": "refund_id",
                          "type": [
                            "null",
                            "long"
                          ]
                        },
                        {
                          "name": "amount",
                          "type": [
                            "null",
                            "string"
                          ]
                        },
                        {
                          "name": "tax_amount",
                          "type": [
                            "null",
                            "string"
                          ]
                        },
                        {
                          "name": "kind",
                          "type": [
                            "null",
                            "string"
                          ]
                        },
                        {
                          "name": "reason",
                          "type": [
                            "null",
                            "string"
                          ]
                        },
                        {
                          "name": "amount_set",
                          "type": [
                            "null",
                            "shopify.order.price_set.PriceSet"
                          ]
                        },
                        {
                          "name": "tax_amount_set",
                          "type": [
                            "null",
                            "shopify.order.price_set.PriceSet"
                          ]
                        }
                      ]
                    }
                  }
                ]
              },
              {
                "name": "processed_at",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "refund_duties",
                "type": [
                  "null",
                  {
                    "type": "array",
                    "default": [],
                    "items": {
                      "type": "record",
                      "name": "RefundDuty",
                      "namespace": "shopify.order.refund.refund_duty",
                      "fields": [
                        {
                          "name": "duty_id",
                          "type": [
                            "null",
                            "long"
                          ]
                        },
                        {
                          "name": "refund_type",
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
                "name": "refund_line_items",
                "type": [
                  "null",
                  {
                    "type": "array",
                    "default": [],
                    "items": {
                      "type": "record",
                      "name": "RefundLineItem",
                      "namespace": "shopify.order.refund.refund_line_item",
                      "fields": [
                        {
                          "name": "id",
                          "type": [
                            "null",
                            "long"
                          ]
                        },
                        {
                          "name": "line_item",
                          "type": [
                            "null",
                            "shopify.order.line_item.LineItem"
                          ]
                        },
                        {
                          "name": "line_item_id",
                          "type": [
                            "null",
                            "long"
                          ]
                        },
                        {
                          "name": "quantity",
                          "type": [
                            "null",
                            "long"
                          ]
                        },
                        {
                          "name": "location_id",
                          "type": [
                            "null",
                            "long"
                          ]
                        },
                        {
                          "name": "restock_type",
                          "type": [
                            "null",
                            "string"
                          ]
                        },
                        {
                          "name": "subtotal",
                          "type": [
                            "null",
                            "string"
                          ]
                        },
                        {
                          "name": "total_tax",
                          "type": [
                            "null",
                            "string"
                          ]
                        },
                        {
                          "name": "subtotal_set",
                          "type": [
                            "null",
                            "shopify.order.price_set.PriceSet"
                          ]
                        },
                        {
                          "name": "total_tax_set",
                          "type": [
                            "null",
                            "shopify.order.price_set.PriceSet"
                          ]
                        }
                      ]
                    }
                  }
                ]
              },
              {
                "name": "restock",
                "type": [
                  "null",
                  "boolean"
                ]
              },
              {
                "name": "transactions",
                "type": [
                  "null",
                  {
                    "type": "array",
                    "default": [],
                    "items": {
                      "type": "record",
                      "name": "Transaction",
                      "namespace": "shopify.order.refund.transaction",
                      "fields": [
                        {
                          "name": "id",
                          "type": [
                            "null",
                            "long"
                          ]
                        },
                        {
                          "name": "order_id",
                          "type": [
                            "null",
                            "long"
                          ]
                        },
                        {
                          "name": "amount",
                          "type": [
                            "null",
                            "string"
                          ]
                        },
                        {
                          "name": "kind",
                          "type": [
                            "null",
                            "string"
                          ]
                        },
                        {
                          "name": "gateway",
                          "type": [
                            "null",
                            "string"
                          ]
                        },
                        {
                          "name": "status",
                          "type": [
                            "null",
                            "string"
                          ]
                        },
                        {
                          "name": "message",
                          "type": [
                            "null",
                            "string"
                          ]
                        },
                        {
                          "name": "created_at",
                          "type": [
                            "null",
                            "string"
                          ]
                        },
                        {
                          "name": "test",
                          "type": [
                            "null",
                            "boolean"
                          ]
                        },
                        {
                          "name": "authorization",
                          "type": [
                            "null",
                            "string"
                          ]
                        },
                        {
                          "name": "currency",
                          "type": [
                            "null",
                            "string"
                          ]
                        },
                        {
                          "name": "location_id",
                          "type": [
                            "null",
                            "long"
                          ]
                        },
                        {
                          "name": "user_id",
                          "type": [
                            "null",
                            "long"
                          ]
                        },
                        {
                          "name": "parent_id",
                          "type": [
                            "null",
                            "long"
                          ]
                        },
                        {
                          "name": "device_id",
                          "type": [
                            "null",
                            "long"
                          ]
                        },
                        {
                          "name": "receipt",
                          "type": [
                            "null",
                            "shopify.order.fulfillment.receipt.Receipt"
                          ]
                        },
                        {
                          "name": "error_code",
                          "type": [
                            "null",
                            "string"
                          ]
                        },
                        {
                          "name": "source_name",
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
                "name": "user_id",
                "type": [
                  "null",
                  "long"
                ]
              }
            ]
          }
        }
      ]
    },
    {
      "name": "shipping_address",
      "type": [
        "null",
        "shopify.order.address.Address"
      ]
    },
    {
      "name": "shipping_lines",
      "type": [
        "null",
        {
          "type": "array",
          "default": [],
          "items": {
            "type": "record",
            "name": "ShippingLine",
            "namespace": "shopify.order.shipping_line",
            "fields": [
              {
                "name": "code",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "price",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "price_set",
                "type": [
                  "null",
                  "shopify.order.price_set.PriceSet"
                ]
              },
              {
                "name": "discounted_price",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "discounted_price_set",
                "type": [
                  "null",
                  "shopify.order.price_set.PriceSet"
                ]
              },
              {
                "name": "source",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "title",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "tax_lines",
                "type": [
                  "null",
                  {
                    "type": "array",
                    "default": [],
                    "items": "shopify.order.tax_line.TaxLine"
                  }
                ]
              },
              {
                "name": "carrier_identifier",
                "type": [
                  "null",
                  "string"
                ]
              },
              {
                "name": "requested_fulfillment_service_id",
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
      "name": "source_name",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "source_identifier",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "source_url",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "subtotal_price",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "subtotal_price_set",
      "type": [
        "null",
        "shopify.order.price_set.PriceSet"
      ]
    },
    {
      "name": "tags",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "taxes_included",
      "type": [
        "null",
        "boolean"
      ]
    },
    {
      "name": "test",
      "type": [
        "null",
        "boolean"
      ]
    },
    {
      "name": "token",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "total_discounts",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "total_discounts_set",
      "type": [
        "null",
        "shopify.order.price_set.PriceSet"
      ]
    },
    {
      "name": "total_line_items_price",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "total_line_items_price_set",
      "type": [
        "null",
        "shopify.order.price_set.PriceSet"
      ]
    },
    {
      "name": "total_outstanding",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "total_price",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "total_price_set",
      "type": [
        "null",
        "shopify.order.price_set.PriceSet"
      ]
    },
    {
      "name": "total_shipping_price_set",
      "type": [
        "null",
        "shopify.order.price_set.PriceSet"
      ]
    },
    {
      "name": "total_tax",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "total_tax_set",
      "type": [
        "null",
        "shopify.order.price_set.PriceSet"
      ]
    },
    {
      "name": "total_tip_received",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "total_weight",
      "type": [
        "null",
        "long"
      ]
    },
    {
      "name": "updated_at",
      "type": [
        "null",
        "string"
      ]
    },
    {
      "name": "user_id",
      "type": [
        "null",
        "long"
      ]
    },
    {
      "name": "order_status_url",
      "type": [
        "null",
        "string"
      ]
    }
  ]
}

"#;
        Schema::parse_str(raw_schema).unwrap()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Address {
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub city: Option<String>,
    pub company: Option<String>,
    pub country: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub province: Option<String>,
    pub zip: Option<String>,
    pub name: Option<String>,
    pub province_code: Option<String>,
    pub country_code: Option<String>,
    pub country_name: Option<String>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub default: Option<bool>,
    pub id: Option<u64>,
    pub customer_id: Option<u64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientDetails {
    pub accept_language: Option<String>,
    pub browser_height: Option<u16>,
    pub browser_ip: Option<String>,
    pub browser_width: Option<u16>,
    pub session_hash: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriceSet {
    pub shop_money: Option<PriceSetMoney>,
    pub presentment_money: Option<PriceSetMoney>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriceSetMoney {
    pub amount: Option<String>,
    pub currency_code: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Customer {
    pub accepts_marketing: Option<bool>,
    pub accepts_marketing_updated_at: Option<String>,
    pub addresses: Option<Vec<Address>>,
    pub currency: Option<String>,
    pub created_at: Option<String>,
    pub default_address: Option<Address>,
    pub email: Option<String>,
    pub email_marketing_consent: Option<MarketingConsent>,
    pub first_name: Option<String>,
    pub id: Option<u64>,
    pub last_name: Option<String>,
    pub last_order_id: Option<u64>,
    pub last_order_name: Option<String>,
    pub metafield: Option<Metafield>,
    pub marketing_opt_in_level: Option<String>,
    // pub multipass_identifier: Option<String>,
    pub note: Option<String>,
    pub orders_count: Option<u64>,
    // pub password: Option<String>,
    // pub password_confirmation: Option<String>,
    pub phone: Option<String>,
    pub sms_marketing_consent: Option<MarketingConsent>,
    pub state: Option<String>,
    pub tags: Option<String>,
    pub tax_exempt: Option<bool>,
    pub tax_exemptions: Option<Vec<String>>,
    pub total_spent: Option<String>,
    pub updated_at: Option<String>,
    pub verified_email: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketingConsent {
    pub state: Option<String>,
    pub opt_in_level: Option<String>,
    pub consent_updated_at: Option<String>,
    pub consent_collected_from: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metafield {
    pub key: Option<String>,
    pub namespace: Option<String>,
    pub value: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiscountApplication {
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub value: Option<String>,
    pub value_type: Option<String>,
    pub allocation_method: Option<String>,
    pub target_selection: Option<String>,
    pub target_type: Option<String>,
    pub code: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiscountCode {
    pub code: Option<String>,
    pub amount: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fulfillment {
    pub created_at: Option<String>,
    pub id: Option<u64>,
    pub line_items: Option<Vec<LineItem>>,
    pub location_id: Option<u64>,
    pub name: Option<String>,
    pub notify_customer: Option<bool>,
    pub order_id: Option<u64>,
    pub origin_address: Option<Address>,
    pub receipt: Option<Receipt>,
    pub service: Option<String>,
    pub shipment_status: Option<String>,
    pub status: Option<String>,
    pub tracking_company: Option<String>,
    pub tracking_numbers: Option<Vec<String>>,
    pub tracking_number: Option<String>,
    pub tracking_urls: Option<Vec<String>>,
    pub tracking_url: Option<String>,
    pub updated_at: Option<String>,
    pub variant_inventory_management: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Receipt {
    pub testcase: Option<bool>,
    pub authorization: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LineItem {
    pub id: Option<u64>,
    pub variant_id: Option<u64>,
    pub title: Option<String>,
    pub quantity: Option<u64>,
    pub price: Option<String>,
    pub grams: Option<u64>,
    pub sku: Option<String>,
    pub variant_title: Option<String>,
    pub vendor: Option<String>,
    pub fulfillment_service: Option<String>,
    pub product_id: Option<u64>,
    pub requires_shipping: Option<bool>,
    pub taxable: Option<bool>,
    pub gift_card: Option<bool>,
    pub name: Option<String>,
    pub variant_inventory_management: Option<String>,
    pub properties: Option<Vec<Property>>,
    pub product_exists: Option<bool>,
    pub fulfillable_quantity: Option<u64>,
    pub total_discount: Option<String>,
    pub fulfillment_status: Option<String>,
    pub fulfillment_line_item_id: Option<u64>,
    pub tax_lines: Option<Vec<TaxLine>>,
    pub duties: Option<Vec<Duty>>,
    pub attributed_staffs: Option<Vec<Staff>>,
    pub price_set: Option<PriceSet>,
    pub total_discount_set: Option<PriceSet>,
    pub discount_allocations: Option<Vec<DiscountAllocation>>,
    pub origin_location: Option<Address>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaxLine {
    pub title: Option<String>,
    pub price: Option<String>,
    pub rate: Option<String>,
    pub price_set: Option<PriceSet>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Duty {
    pub id: Option<String>,
    pub harmonized_system_code: Option<String>,
    pub country_code_of_origin: Option<String>,
    pub shop_money: Option<PriceSetMoney>,
    pub presentment_money: Option<PriceSetMoney>,
    pub tax_lines: Option<Vec<TaxLine>>,
    pub admin_graphql_api_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Property {
    pub name: Option<String>,
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Staff {
    pub id: Option<String>,
    pub quantity: Option<u64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiscountAllocation {
    pub amount: Option<String>,
    pub discount_application_index: Option<u64>,
    pub amount_set: Option<PriceSet>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoteAttribute {
    pub name: Option<String>,
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentTerm {
    pub amount: Option<String>,
    pub currency: Option<String>,
    pub payment_terms_name: Option<String>,
    pub payment_terms_type: Option<String>,
    pub due_in_days: Option<u64>,
    pub payment_schedules: Option<Vec<PaymentSchedule>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentSchedule {
    pub amount: Option<String>,
    pub currency: Option<String>,
    pub issued_at: Option<String>,
    pub due_at: Option<String>,
    pub completed_at: Option<String>,
    pub expected_payment_method: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Refund {
    pub created_at: Option<String>,
    pub duties: Option<Vec<Duty>>,
    pub id: Option<u64>,
    pub note: Option<String>,
    pub order_adjustments: Option<Vec<OrderAdjustment>>,
    pub processed_at: Option<String>,
    pub refund_duties: Option<Vec<RefundDuty>>,
    pub refund_line_items: Option<Vec<RefundLineItem>>,
    pub restock: Option<bool>,
    pub transactions: Option<Vec<Transaction>>,
    pub user_id: Option<u64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderAdjustment {
    pub id: Option<u64>,
    pub order_id: Option<u64>,
    pub refund_id: Option<u64>,
    pub amount: Option<String>,
    pub tax_amount: Option<String>,
    pub kind: Option<String>,
    pub reason: Option<String>,
    pub amount_set: Option<PriceSet>,
    pub tax_amount_set: Option<PriceSet>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RefundDuty {
    pub duty_id: Option<u64>,
    pub refund_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RefundLineItem {
    pub id: Option<u64>,
    pub line_item: Option<LineItem>,
    pub line_item_id: Option<u64>,
    pub quantity: Option<u64>,
    pub location_id: Option<u64>,
    pub restock_type: Option<String>,
    pub subtotal: Option<String>,
    pub total_tax: Option<String>,
    pub subtotal_set: Option<PriceSet>,
    pub total_tax_set: Option<PriceSet>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Option<u64>,
    pub order_id: Option<u64>,
    pub amount: Option<String>,
    pub kind: Option<String>,
    pub gateway: Option<String>,
    pub status: Option<String>,
    pub message: Option<String>,
    pub created_at: Option<String>,
    pub test: Option<bool>,
    pub authorization: Option<String>,
    pub currency: Option<String>,
    pub location_id: Option<u64>,
    pub user_id: Option<u64>,
    pub parent_id: Option<u64>,
    pub device_id: Option<u64>,
    pub receipt: Option<Receipt>,
    pub error_code: Option<String>,
    pub source_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShippingLine {
    pub code: Option<String>,
    pub price: Option<String>,
    pub price_set: Option<PriceSet>,
    pub discounted_price: Option<String>,
    pub discounted_price_set: Option<PriceSet>,
    pub source: Option<String>,
    pub title: Option<String>,
    pub tax_lines: Option<Vec<TaxLine>>,
    pub carrier_identifier: Option<String>,
    pub requested_fulfillment_service_id: Option<String>,
}
