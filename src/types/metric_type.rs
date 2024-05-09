use crate::bindings;

pub enum MetricType {
    Undefined                  = bindings::MetricType_UndefinedErrorMetric                  as isize,
    Absolute                   = bindings::MetricType_AbsoluteErrorMetric                   as isize,
    Fuzz                       = bindings::MetricType_FuzzErrorMetric                       as isize,
    MeanAbsolute               = bindings::MetricType_MeanAbsoluteErrorMetric               as isize,
    MeanErrorPerPixel          = bindings::MetricType_MeanErrorPerPixelErrorMetric          as isize,
    MeanSquared                = bindings::MetricType_MeanSquaredErrorMetric                as isize,
    NormalizedCrossCorrelation = bindings::MetricType_NormalizedCrossCorrelationErrorMetric as isize,
    PeakAbsolute               = bindings::MetricType_PeakAbsoluteErrorMetric               as isize,
    PeakSignalToNoiseRatio     = bindings::MetricType_PeakSignalToNoiseRatioErrorMetric     as isize,
    PerceptualHash             = bindings::MetricType_PerceptualHashErrorMetric             as isize,
    RootMeanSquared            = bindings::MetricType_RootMeanSquaredErrorMetric            as isize,
    StructuralSimilarity       = bindings::MetricType_StructuralSimilarityErrorMetric       as isize,
    StructuralDissimilarity    = bindings::MetricType_StructuralDissimilarityErrorMetric    as isize,
}

impl Default for MetricType {
    fn default() -> Self {
        return MetricType::Absolute;
    }
}

impl From<MetricType> for bindings::MetricType {
    fn from(value: MetricType) -> Self {
        return value as bindings::MetricType;
    }
}
