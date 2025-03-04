use numpy::PyUntypedArray;
use pyo3::exceptions::{PySystemError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use std::collections::HashMap;
mod data_bias;
use data_bias::{pre_training_bias, PreTraining};
mod model_bias;
use model_bias::{post_training_bias, PostTrainingData};
mod data_handler;
use data_handler::{apply_label, perform_segmentation_data_bias, perform_segmentation_model_bias};
mod runtime;
use runtime::{DataBiasRuntime, ModelBiasRuntime};
mod models;
use models::{FailureRuntimeReturn, ModelType, PassedRuntimeReturn};
mod macros;
mod model_perf;
use model_perf::{
    model_perf_classification, model_perf_logistic_regression, model_perf_regression,
    LinearRegressionReport, ModelPerformanceType,
};

#[pyfunction]
#[pyo3(signature = (
    baseline,
    latest,
    threshold=0.10
)
)]
pub fn data_bias_runtime_check<'py>(
    baseline: HashMap<String, f32>,
    latest: HashMap<String, f32>,
    threshold: f32,
) -> PyResult<String> {
    let current = match DataBiasRuntime::try_from(latest) {
        Ok(obj) => obj,
        Err(_) => return Err(PyValueError::new_err("Invalid metrics body passed")),
    };

    let baseline = match DataBiasRuntime::try_from(baseline) {
        Ok(obj) => obj,
        Err(_) => return Err(PyValueError::new_err("Invalid baseline body passed")),
    };
    let failure_report: HashMap<String, String> = current.runtime_check(baseline, threshold);
    if failure_report.len() > 0 {
        match serde_json::to_string(&FailureRuntimeReturn {
            passed: false,
            fail_report: Some(failure_report),
        }) {
            Ok(val) => Ok(val),
            Err(_) => Err(PySystemError::new_err("Internal error")),
        }
    } else {
        match serde_json::to_string(&PassedRuntimeReturn { passed: true }) {
            Ok(val) => Ok(val),
            Err(_) => Err(PySystemError::new_err("Internal error")),
        }
    }
}

#[pyfunction]
#[pyo3(signature = (
    baseline,
    latest,
    threshold=0.10
)
)]
pub fn model_bias_runtime_check<'py>(
    baseline: HashMap<String, f32>,
    latest: HashMap<String, f32>,
    threshold: f32,
) -> PyResult<String> {
    let current = match ModelBiasRuntime::try_from(latest) {
        Ok(obj) => obj,
        Err(_) => return Err(PyValueError::new_err("Invalid metrics body passed")),
    };
    let baseline = match ModelBiasRuntime::try_from(baseline) {
        Ok(obj) => obj,
        Err(_) => return Err(PyValueError::new_err("Invalid baseline body passed")),
    };
    let failure_report: HashMap<String, String> = current.runtime_check(baseline, threshold);
    if failure_report.len() > 0 {
        match serde_json::to_string(&FailureRuntimeReturn {
            passed: false,
            fail_report: Some(failure_report),
        }) {
            Ok(val) => Ok(val),
            Err(_) => Err(PySystemError::new_err("Internal error")),
        }
    } else {
        match serde_json::to_string(&PassedRuntimeReturn { passed: true }) {
            Ok(val) => Ok(val),
            Err(_) => Err(PySystemError::new_err("Internal error")),
        }
    }
}

#[pyfunction]
#[pyo3(signature = (
    feature_array,
    ground_truth_array,
    prediction_array,
    feature_label_or_threshold,
    ground_truth_label_or_threshold,
    prediction_label_or_threshold)
)]
pub fn model_bias_analyzer<'py>(
    py: Python<'_>,
    feature_array: &Bound<'_, PyUntypedArray>,
    ground_truth_array: &Bound<'_, PyUntypedArray>,
    prediction_array: &Bound<'_, PyUntypedArray>,
    feature_label_or_threshold: Bound<'py, PyAny>, //fix
    ground_truth_label_or_threshold: Bound<'py, PyAny>, //fix
    prediction_label_or_threshold: Bound<'py, PyAny>, // fix
) -> PyResult<HashMap<String, f32>> {
    let labeled_predictions: Vec<i16> =
        match apply_label(py, prediction_array, prediction_label_or_threshold) {
            Ok(array) => array,
            Err(err) => return Err(PyTypeError::new_err(err)),
        };
    let labeled_ground_truth: Vec<i16> =
        match apply_label(py, ground_truth_array, ground_truth_label_or_threshold) {
            Ok(array) => array,
            Err(err) => return Err(PyTypeError::new_err(err)),
        };
    let labeled_features: Vec<i16> =
        match apply_label(py, feature_array, feature_label_or_threshold) {
            Ok(array) => array,
            Err(err) => return Err(PyTypeError::new_err(err)),
        };
    let post_training_data: PostTrainingData = match perform_segmentation_model_bias(
        labeled_features,
        labeled_predictions,
        labeled_ground_truth,
    ) {
        Ok(res) => res,
        Err(err) => return Err(PyTypeError::new_err(err)),
    };
    match post_training_bias(post_training_data) {
        Ok(value) => Ok(value),
        Err(err) => Err(PyTypeError::new_err(err)),
    }
}

#[pyfunction]
#[pyo3(signature = (
    feature_array,
    ground_truth_array,
    feature_label_or_threshold,
    ground_truth_label_or_threshold)
)]
fn data_bias_analyzer<'py>(
    py: Python<'_>,
    feature_array: &Bound<'_, PyUntypedArray>,
    ground_truth_array: &Bound<'_, PyUntypedArray>,
    feature_label_or_threshold: Bound<'py, PyAny>, //fix
    ground_truth_label_or_threshold: Bound<'py, PyAny>, //fix
) -> PyResult<HashMap<String, f32>> {
    let labeled_ground_truth =
        match apply_label(py, ground_truth_array, ground_truth_label_or_threshold) {
            Ok(array) => array,
            Err(err) => return Err(PyTypeError::new_err(err)),
        };

    let labeled_feature = match apply_label(py, feature_array, feature_label_or_threshold) {
        Ok(array) => array,
        Err(err) => return Err(PyTypeError::new_err(err)),
    };

    let pre_training: PreTraining =
        match perform_segmentation_data_bias(labeled_feature, labeled_ground_truth) {
            Ok(values) => values,
            Err(err) => return Err(PyTypeError::new_err(err)),
        };

    match pre_training_bias(pre_training) {
        Ok(result) => Ok(result),
        Err(err) => Err(PyTypeError::new_err(err)),
    }
}

#[pyfunction]
#[pyo3(signature = (
    y_pred,
    y_true)
)]
fn model_performance_regression<'py>(
    py: Python<'_>,
    y_pred: &Bound<'_, PyUntypedArray>,
    y_true: &Bound<'_, PyUntypedArray>,
) -> PyResult<HashMap<String, f32>> {
    match model_perf_regression(py, y_pred, y_true) {
        Ok(res) => Ok(res),
        Err(_) => Err(PyValueError::new_err::<String>(
            "Invalid arrays for y_pred and y_true".into(),
        )),
    }
}

#[pyfunction]
#[pyo3(signature = (
    y_pred,
    y_true)
)]
fn model_performance_classification<'py>(
    py: Python<'_>,
    y_pred: &Bound<'_, PyUntypedArray>,
    y_true: &Bound<'_, PyUntypedArray>,
) -> PyResult<HashMap<String, f32>> {
    match model_perf_classification(py, y_pred, y_true) {
        Ok(res) => Ok(res),
        Err(_) => Err(PyValueError::new_err::<String>(
            "Invalid arrays for y_pred and y_true".into(),
        )),
    }
}

#[pyfunction]
#[pyo3(signature = (
    y_pred,
    y_true,
    decision_threshold=0.5
)
)]
fn model_performance_logisitic_regression<'py>(
    py: Python<'_>,
    y_pred: &Bound<'_, PyUntypedArray>,
    y_true: &Bound<'_, PyUntypedArray>,
    decision_threshold: f32,
) -> PyResult<HashMap<String, f32>> {
    match model_perf_logistic_regression(py, y_pred, y_true, decision_threshold) {
        Ok(res) => Ok(res),
        Err(_) => Err(PyValueError::new_err::<String>(
            "Invalid arrays for y_pred and y_true".into(),
        )),
    }
}

#[pyfunction]
#[pyo3(signature = (
    baseline,
    latest,
    threshold=0.10
)
)]
fn regression_performance_runtime_full(
    baseline: HashMap<String, f32>,
    latest: HashMap<String, f32>,
    threshold: f32,
) -> PyResult<String> {
    let baseline: LinearRegressionReport = match LinearRegressionReport::try_from(baseline) {
        Ok(val) => val,
        Err(e) => {
            return Err(PyValueError::new_err(format!(
                "Invalid baseline report: {}",
                e
            )))
        }
    };
    let latest: LinearRegressionReport = match LinearRegressionReport::try_from(latest) {
        Ok(val) => val,
        Err(e) => {
            return Err(PyValueError::new_err(format!(
                "Invalid latest report: {}",
                e
            )))
        }
    };
    let results = latest.compare_to_baseline(&baseline, threshold);
    if results.len() > 0 {
        match serde_json::to_string(&FailureRuntimeReturn {
            passed: false,
            fail_report: Some(results),
        }) {
            Ok(val) => Ok(val),
            Err(_) => Err(PySystemError::new_err("Internal error")),
        }
    } else {
        match serde_json::to_string(&PassedRuntimeReturn { passed: true }) {
            Ok(val) => Ok(val),
            Err(_) => Err(PySystemError::new_err("Internal error")),
        }
    }
}

#[pyfunction]
#[pyo3(signature = (
    baseline,
    latest,
    evaluation_metrics,
    threshold=0.10
)
)]
fn regression_performance_runtime_partial(
    baseline: HashMap<String, f32>,
    latest: HashMap<String, f32>,
    evaluation_metrics: Vec<String>,
    threshold: f32,
) -> PyResult<String> {
    let baseline: LinearRegressionReport = match LinearRegressionReport::try_from(baseline) {
        Ok(val) => val,
        Err(e) => {
            return Err(PyValueError::new_err(format!(
                "Invalid baseline report: {}",
                e
            )))
        }
    };
    let latest: LinearRegressionReport = match LinearRegressionReport::try_from(latest) {
        Ok(val) => val,
        Err(e) => {
            return Err(PyValueError::new_err(format!(
                "Invalid latest report: {}",
                e
            )))
        }
    };
    let results = latest.compare_to_baseline(&baseline, threshold);
    if results.len() > 0 {
        match serde_json::to_string(&FailureRuntimeReturn {
            passed: false,
            fail_report: Some(results),
        }) {
            Ok(val) => Ok(val),
            Err(_) => Err(PySystemError::new_err("Internal error")),
        }
    } else {
        match serde_json::to_string(&PassedRuntimeReturn { passed: true }) {
            Ok(val) => Ok(val),
            Err(_) => Err(PySystemError::new_err("Internal error")),
        }
    }
}

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "_fair_ml")]
fn fair_ml(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(model_bias_analyzer, m)?)?;
    m.add_function(wrap_pyfunction!(data_bias_analyzer, m)?)?;
    m.add_function(wrap_pyfunction!(data_bias_runtime_check, m)?)?;
    m.add_function(wrap_pyfunction!(model_bias_runtime_check, m)?)?;
    m.add_function(wrap_pyfunction!(model_performance_regression, m)?)?;
    m.add_function(wrap_pyfunction!(model_performance_classification, m)?)?;
    m.add_function(wrap_pyfunction!(regression_performance_runtime_full, m)?)?;
    m.add_function(wrap_pyfunction!(model_performance_logisitic_regression, m)?)?;
    Ok(())
}
