//! Information about a single training query run for the model.
use crate::model::data_split_result::DataSplitResult;
use crate::model::evaluation_metrics::EvaluationMetrics;
use crate::model::global_explanation::GlobalExplanation;
use crate::model::iteration_result::IterationResult;
use crate::model::training_options::TrainingOptions;
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrainingRun {
    /// The evaluation metrics over training/eval data that were computed at the end of training.
    pub evaluation_metrics: Option<EvaluationMetrics>,
    /// The start time of this training run.
    #[serde(with = "time::serde::rfc3339")]
    pub start_time: OffsetDateTime,
    /// Data split result of the training run. Only set when the input data is actually split.
    pub data_split_result: Option<DataSplitResult>,
    /// Options that were used for this training run, includes user specified and default options that were used.
    pub training_options: Option<TrainingOptions>,
    /// Global explanations for important features of the model. For multi-class models, there is one entry for each label class. For other models, there is only one entry in the list.
    pub global_explanations: Option<Vec<GlobalExplanation>>,
    /// Output of each iteration run, results.size() <= max_iterations.
    pub results: Option<Vec<IterationResult>>,
}

impl Default for TrainingRun {
    fn default() -> Self {
        Self {
            evaluation_metrics: None,
            start_time: OffsetDateTime::now_utc(),
            data_split_result: None,
            training_options: None,
            global_explanations: None,
            results: None,
        }
    }
}
