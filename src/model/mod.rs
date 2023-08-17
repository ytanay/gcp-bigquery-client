//! All the object definitions used by the BigQuery REST API.

// This module contains structs generated from the  Google API Explorer.
// To prevent Clippy warnings, we need to allow 'module_inception'.
// Example: The struct Model exists in the BQ Discovery Document and this name is also used
// for the current module.
#![allow(clippy::module_inception)]
pub mod aggregate_classification_metrics;
pub mod argument;
pub mod arima_coefficients;
pub mod arima_fitting_metrics;
pub mod arima_forecasting_metrics;
pub mod arima_model_info;
pub mod arima_order;
pub mod arima_result;
pub mod arima_single_model_forecasting_metrics;
pub mod audit_config;
pub mod audit_log_config;
pub mod bigquery_model_training;
pub mod bigtable_column;
pub mod bigtable_column_family;
pub mod bigtable_options;
pub mod binary_classification_metrics;
pub mod binary_confusion_matrix;
pub mod binding;
pub mod bqml_iteration_result;
pub mod bqml_training_run;
pub mod bqml_training_run_training_options;
pub mod categorical_value;
pub mod category_count;
pub mod cluster;
pub mod cluster_info;
pub mod clustering;
pub mod clustering_metrics;
pub mod confusion_matrix;
pub mod connection_property;
pub mod csv_options;
pub mod data_format_options;
pub mod data_split_result;
pub mod dataset;
pub mod dataset_reference;
pub mod datasets;
pub mod destination_table_properties;
pub mod dimensionality_reduction_metrics;
pub mod encryption_configuration;
pub mod entry;
pub mod error_proto;
pub mod evaluation_metrics;
pub mod explain_query_stage;
pub mod explain_query_step;
pub mod explanation;
pub mod expr;
pub mod external_data_configuration;
pub mod feature_value;
pub mod field_type;
pub mod get_iam_policy_request;
pub mod get_policy_options;
pub mod get_query_results_parameters;
pub mod get_query_results_response;
pub mod get_service_account_response;
pub mod global_explanation;
pub mod google_sheets_options;
pub mod hive_partitioning_options;
pub mod information_schema;
pub mod iteration_result;
pub mod job;
pub mod job_cancel_response;
pub mod job_configuration;
pub mod job_configuration_extract;
pub mod job_configuration_load;
pub mod job_configuration_query;
pub mod job_configuration_table_copy;
pub mod job_list;
pub mod job_list_jobs;
pub mod job_list_parameters;
pub mod job_reference;
pub mod job_statistics;
pub mod job_statistics2;
pub mod job_statistics3;
pub mod job_statistics4;
pub mod job_statistics_reservation_usage;
pub mod job_status;
pub mod list_models_response;
pub mod list_routines_response;
pub mod materialized_view_definition;
pub mod model;
pub mod model_definition;
pub mod model_definition_model_options;
pub mod model_reference;
pub mod multi_class_classification_metrics;
pub mod policy;
pub mod principal_component_info;
pub mod project_list;
pub mod project_reference;
pub mod query_parameter;
pub mod query_parameter_type;
pub mod query_parameter_type_struct_types;
pub mod query_parameter_value;
pub mod query_request;
pub mod query_response;
pub mod query_timeline_sample;
pub mod range_partitioning;
pub mod range_partitioning_range;
pub mod ranking_metrics;
pub mod regression_metrics;
pub mod routine;
pub mod routine_reference;
pub mod row;
pub mod row_access_policy;
pub mod row_access_policy_reference;
pub mod row_level_security_statistics;
pub mod script_stack_frame;
pub mod script_statistics;
pub mod set_iam_policy_request;
pub mod snapshot_definition;
pub mod standard_sql_data_type;
pub mod standard_sql_field;
pub mod standard_sql_struct_type;
pub mod streamingbuffer;
pub mod table;
pub mod table_cell;
pub mod table_data_insert_all_request;
pub mod table_data_insert_all_request_rows;
pub mod table_data_insert_all_response;
pub mod table_data_insert_all_response_insert_errors;
pub mod table_field_schema;
pub mod table_field_schema_categories;
pub mod table_field_schema_policy;
pub mod table_list;
pub mod table_list_tables;
pub mod table_list_view;
pub mod table_reference;
pub mod table_row;
pub mod table_schema;
pub mod test_iam_permissions_request;
pub mod test_iam_permissions_response;
pub mod time_partitioning;
pub mod training_options;
pub mod training_run;
pub mod transaction_info;
pub mod user_defined_function_resource;
pub mod view_definition;
