// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

#![doc = include_str!("../README.md")]

pub mod collect;
#[cfg(feature = "build-binary")]
pub mod config;
pub mod execution_engine;
pub mod execution_loop;
pub mod executor;
pub mod executor_process;
pub mod executor_server;
pub mod flight_service;
pub mod metrics;
pub mod shutdown;
pub mod terminate;

mod cpu_bound_executor;
mod standalone;

use ballista_core::error::BallistaError;
use std::net::SocketAddr;

pub use standalone::new_standalone_executor;
pub use standalone::new_standalone_executor_from_builder;
pub use standalone::new_standalone_executor_from_state;

use log::info;

use crate::shutdown::Shutdown;
use ballista_core::serde::protobuf::{
    task_status, FailedTask, OperatorMetricsSet, ShuffleWritePartition, SuccessfulTask,
    TaskStatus,
};
use ballista_core::serde::scheduler::PartitionId;

/// [ArrowFlightServerProvider] provides a function which creates a new Arrow Flight server.
///
/// The function should take two arguments:
/// [SocketAddr] - the address to bind the server to
/// [Shutdown] - a shutdown signal to gracefully shutdown the server
/// Returns a [tokio::task::JoinHandle] which will be registered as service handler
///
pub type ArrowFlightServerProvider = dyn Fn(SocketAddr, Shutdown) -> tokio::task::JoinHandle<Result<(), BallistaError>>
    + Send
    + Sync;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskExecutionTimes {
    launch_time: u64,
    start_exec_time: u64,
    end_exec_time: u64,
}

pub fn as_task_status(
    execution_result: ballista_core::error::Result<Vec<ShuffleWritePartition>>,
    executor_id: String,
    task_id: usize,
    stage_attempt_num: usize,
    partition_id: PartitionId,
    operator_metrics: Option<Vec<OperatorMetricsSet>>,
    execution_times: TaskExecutionTimes,
) -> TaskStatus {
    let metrics = operator_metrics.unwrap_or_default();
    match execution_result {
        Ok(partitions) => {
            info!(
                "Task {:?} finished with operator_metrics array size {}",
                task_id,
                metrics.len()
            );
            TaskStatus {
                task_id: task_id as u32,
                job_id: partition_id.job_id,
                stage_id: partition_id.stage_id as u32,
                stage_attempt_num: stage_attempt_num as u32,
                partition_id: partition_id.partition_id as u32,
                launch_time: execution_times.launch_time,
                start_exec_time: execution_times.start_exec_time,
                end_exec_time: execution_times.end_exec_time,
                metrics,
                status: Some(task_status::Status::Successful(SuccessfulTask {
                    executor_id,
                    partitions,
                })),
            }
        }
        Err(e) => {
            let error_msg = e.to_string();
            info!("Task {task_id:?} failed: {error_msg}");

            TaskStatus {
                task_id: task_id as u32,
                job_id: partition_id.job_id,
                stage_id: partition_id.stage_id as u32,
                stage_attempt_num: stage_attempt_num as u32,
                partition_id: partition_id.partition_id as u32,
                launch_time: execution_times.launch_time,
                start_exec_time: execution_times.start_exec_time,
                end_exec_time: execution_times.end_exec_time,
                metrics,
                status: Some(task_status::Status::Failed(FailedTask::from(e))),
            }
        }
    }
}
