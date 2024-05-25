/*
 * Copyright 2024 5ohue
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use crate::bindings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum StatisticType {
    Undefined = bindings::StatisticType_UndefinedStatistic,
    Gradient = bindings::StatisticType_GradientStatistic,
    Maximum = bindings::StatisticType_MaximumStatistic,
    Mean = bindings::StatisticType_MeanStatistic,
    Median = bindings::StatisticType_MedianStatistic,
    Minimum = bindings::StatisticType_MinimumStatistic,
    Mode = bindings::StatisticType_ModeStatistic,
    Nonpeak = bindings::StatisticType_NonpeakStatistic,
    RootMeanSquare = bindings::StatisticType_RootMeanSquareStatistic,
    StandardDeviation = bindings::StatisticType_StandardDeviationStatistic,
    Contrast = bindings::StatisticType_ContrastStatistic,
}

impl Default for StatisticType {
    fn default() -> Self {
        return StatisticType::Undefined;
    }
}

impl From<StatisticType> for bindings::StatisticType {
    fn from(value: StatisticType) -> Self {
        return value as bindings::StatisticType;
    }
}

impl From<bindings::StatisticType> for StatisticType {
    fn from(value: bindings::StatisticType) -> Self {
        /*
         * SAFETY:
         *
         * `StatisticType` has the same repr as `bindings::StatisticType` - u32
         *
         * If `value` is less than Contrast than it is in the vaild range and can be safely
         * reinterpreted as `StatisticType`
         */
        if value <= bindings::StatisticType_ContrastStatistic {
            return unsafe { std::mem::transmute(value) };
        }
        return StatisticType::default();
    }
}
