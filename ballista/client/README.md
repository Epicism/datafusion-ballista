# Apache Ballista: Distributed Compute Platform for Apache Arrow DataFusion

**Apache Ballista** is a distributed compute platform built on [Apache Arrow](https://arrow.apache.org/) and [DataFusion](https://arrow.apache.org/datafusion/). It enables scalable, distributed data processing using a modern, memory-efficient execution engine. Ballista extends DataFusion's capabilities from standalone to **multi-node clusters**, facilitating efficient execution of large-scale data workloads.

Apache Ballista is built on the following foundational technologies:

- [Apache Arrow](https://github.com/apache/arrow-rs): Provides the memory model and compute kernels for efficient in-memory data processing.

- [Apache Arrow Flight Protocol](https://github.com/apache/arrow-rs/tree/main/arrow-flight): Facilitates efficient data transfer between processes.

- [Google Protocol Buffers](https://github.com/protocolbuffers/protobuf): Used for serializing query plans to make distributed execution fast and reliable.

Ballista is designed to be flexible and open-ended, supporting a wide range of use cases—from executing distributed data science workflows to integrating as part of a larger distributed compute program.

--
## Table of Contents
- [Apache Ballista: Distributed Compute Platform for Apache Arrow DataFusion](#apache-ballista-distributed-compute-platform-for-apache-arrow-datafusion)
  - [Table of Contents](#table-of-contents)
  - [Ballista Crate Details](#ballista-crate-details)
    - [ballista (this crate)](#ballista-this-crate)
    - [ballista-core](#ballista-core)
    - [ballista-executor](#ballista-executor)
    - [ballista-scheduler](#ballista-scheduler)
    - [ballista-cli](#ballista-cli)
    - [ballista-python](#ballista-python)
  - [Overview of Deployment Methods](#overview-of-deployment-methods)
    - [Standalone Deployment](#standalone-deployment)
    - [Distributed Deployment](#distributed-deployment)
    - [Python Binding](#python-binding)
    - [Rust Library (Embeddable Library)](#rust-library-embeddable-library)
  - [Deployment Method Details](#deployment-method-details)
    - [Standalone Deployment](#standalone-deployment-1)
    - [Distributed Deployment](#distributed-deployment-1)
    - [Python Binding](#python-binding-1)
    - [Rust Library](#rust-library)
  - [Configuration and Advanced Settings](#configuration-and-advanced-settings)
  - [Community and Additional Resources](#community-and-additional-resources)
  - [Deployment Options](#deployment-options)

---
## Ballista Crate Details

Ballista is composed of several crates that together provide the necessary components for distributed data processing. Below is an overview of each of these crates, along with links to their respective GitHub repositories and crates.io pages for more details.

### **ballista (this crate)**

The `ballista` crate provides the client API, enabling users to interact with the Ballista scheduler. This crate allows for the submission of queries and the retrieval of results, serving as the primary interface for users to engage with the Ballista cluster.

- **GitHub**: [ballista/client](https://github.com/apache/datafusion-ballista/tree/main/ballista/client/src)
- **Crates.io**: [ballista](https://crates.io/crates/ballista)

### **ballista-core**

The `ballista-core` crate serves as the backbone of the Ballista platform, containing shared data structures and logic across the system. This crate includes definitions for the execution plan, scheduler policies, and other fundamental components that ensure cohesive operation among different parts of Ballista.

- **GitHub**: [ballista/core](https://github.com/apache/datafusion-ballista/tree/main/ballista/core/src)
- **Crates.io**: [ballista-core](https://crates.io/crates/ballista-core)

### **ballista-executor**

The `ballista-executor` crate is responsible for executing tasks assigned by the scheduler. Executors handle the actual data processing work, executing query plans on data partitions and returning results to the scheduler. This crate is designed to be scalable, allowing multiple executors to run concurrently across different nodes in a cluster.

- **GitHub**: [ballista/executor](https://github.com/apache/datafusion-ballista/tree/main/ballista/executor/src)
- **Crates.io**: [ballista-executor](https://crates.io/crates/ballista-executor)

### **ballista-scheduler**

The `ballista-scheduler` crate orchestrates the distribution of tasks to executors. It manages execution plans, assigns tasks to available executors, and monitors their progress. The scheduler is critical for ensuring efficient resource utilization and load balancing within the Ballista cluster.

- **GitHub**: [ballista/scheduler](https://github.com/apache/datafusion-ballista/tree/main/ballista/scheduler/src)
- **Crates.io**: [ballista-scheduler](https://crates.io/crates/ballista-scheduler)

Understanding the roles of these crates is crucial for effectively deploying and utilizing Ballista in various environments. Each component plays a vital role in ensuring Ballista's ability to perform distributed data processing efficiently.

### **ballista-cli**

The `ballista-cli` crate provides a command-line interface for interacting with the Ballista cluster. This crate allows users to submit queries, manage jobs, and monitor the status of tasks running in the cluster. The CLI is a convenient tool for administrators and developers to interact with Ballista without writing custom code.

- **GitHub**: [ballista/cli](https://github.com/Epicism/datafusion-ballista/tree/main/ballista-cli)
- **Crates.io**: [ballista-cli](https://crates.io/crates/ballista-cli)

### **ballista-python**
The `ballista-python` crate provides a Python binding for Ballista, allowing users to interact with the Ballista cluster using Python. This crate enables data scientists and analysts to leverage Ballista's distributed capabilities through familiar Python tools and workflows.
- **GitHub**: [ballista/python](https://github.com/apache/datafusion-python)
- **Documentation**: [ballista-python](https://datafusion.apache.org/ballista/user-guide/python.html)

---

## Overview of Deployment Methods

**Ballista** offers three main ways to deploy and use its distributed capabilities. Below is an overview of each method, with links to detailed sections on how to get started, use cases, examples, and more.

1. [**Standalone Deployment**](#standalone-deployment):

   - Accessible through **GitHub or Crates.io**.
   - Use Ballista as a **standalone scheduler/executor** for local testing and development.

2. [**Distributed Deployment**](#distributed-deployment):

   - Expand from standalone deployment to run Ballista across **multiple nodes** for greater scalability.

3. [**Python Binding**](#python-binding):

   - A Python wrapper for the Rust binary.
   - Useful for **ad-hoc data management** or **data science workloads**.

4. [**Rust Library (Embeddable Library)**](#rust-library):

   - Ballista can be embedded into **custom distributed systems**.
   - Best suited for developers who want to **extend** Ballista’s capabilities as part of their distributed compute workload.

---
## Deployment Method Details

### Standalone Deployment

The **standalone deployment** mode of Ballista allows users to run the scheduler and executor locally on a single machine. This is ideal for **prototyping** data analysis workflows using DataFusion without the complexity of setting up a fully distributed cluster. It’s a good starting point for users exploring Ballista's capabilities and developing transformations or queries in a local environment.

#### Use Cases

Standalone Ballista can be used in several scenarios:

- **Local Prototyping**: Easily test and prototype SQL queries or data transformations before scaling up to a distributed setup.
- **Testing and Development**: Use the standalone deployment to develop and verify data analysis logic without requiring a distributed cluster.

#### Deployment Steps

Follow these steps to set up a standalone deployment of Ballista:

1. **Install the Ballista Scheduler and Executor**:

   - Use Cargo to install the necessary components:
     ```bash
     cargo install --locked ballista-scheduler
     cargo install --locked ballista-executor
     ```

2. **Start the Scheduler**:

   - Start the Ballista scheduler to manage task execution:
     ```bash
     RUST_LOG=info ballista-scheduler
     ```

3. **Run the Executor**:

   - Start the executor process locally:
     ```bash
     RUST_LOG=info ballista-executor -c 4
     ```

This setup allows you to process data locally, with the scheduler coordinating tasks and the executor performing the actual computation.

#### Example: Running a Standalone Query

To illustrate the standalone capabilities, let’s run a simple SQL query using Ballista:

The following code demonstrates how to use Ballista to count the number of trips in a Parquet dataset:

```rust
use ballista::prelude::*;
use datafusion::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let ctx = SessionContext::remote("df://localhost:50050").await?;
    let df = ctx.sql("SELECT COUNT(*) FROM trips").await?;
    df.show().await?;
    Ok(())
}
```

In this example, the scheduler and executor both run locally, making it ideal for testing your workflows before scaling up.

---

### Distributed Deployment

Ballista's **distributed deployment** mode allows you to expand from a standalone setup to a multi-node cluster, making processing large-scale data workloads efficiently possible. This deployment method is suitable for organizations or projects that require the scalability and resilience of distributed processing.

#### Use Cases

Distributed Ballista can be used in scenarios where data processing requirements exceed the capabilities of a single machine:

- **Scaling to Distributed Execution**: Expand from a standalone setup to distributed execution by adding more executors to \*\*scale workloads\*\* and parallelize data processing tasks.
- **Handling Large-Scale ETL**: Distributed deployment is ideal for running **ETL (Extract, Transform, Load) processes** that involve massive datasets requiring distributed processing power.

#### Deployment Steps

To move from a standalone to a distributed setup, follow these steps:

1. **Migrating from Standalone to Distributed**:

   - Migrating from standalone to distributed involves **restarting** the scheduler and executors with distributed configuration values. Refer to:
     - [Ballista Configuration Guide](https://datafusion.apache.org/ballista/user-guide/configs.html)
     - [Ballista Tuning Guide](https://datafusion.apache.org/ballista/user-guide/tuning-guide.html)

2. **Adding Executors**:

   - Start additional executor processes to scale up the cluster:
     ```bash
     RUST_LOG=info ballista-executor --bind-port 50052 -c 4
     ```

This setup allows Ballista to distribute tasks among multiple executors, significantly improving performance for large-scale data workloads.

#### Example: Distributed Query Execution

The following example demonstrates how to use multiple executors to run a distributed aggregation query:

```rust
use ballista::prelude::*;
use datafusion::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let ctx = SessionContext::remote("df://localhost:50050").await?;
    let df = ctx.sql("SELECT MIN(fare_amount), MAX(fare_amount) FROM trips").await?;
    df.show().await?;
    Ok(())
}
```

This example illustrates how Ballista can be used in a distributed context, with multiple executors handling the workload.

---

### Python Binding

The **Python binding** allows data scientists and analysts to interact with Ballista through Python, much like using **PySpark**. This makes it easy for those familiar with Python to leverage Ballista's distributed capabilities without diving into Rust code.

#### Use Cases

1. **Ad-hoc Data Management**:

   - Like Pandas, Ballista’s Python API can be used for **quick exploration** and **data management**.
   - **Example Scenarios**: Registering a CSV or Parquet dataset and running basic queries to explore data relationships.

2. **Data Science Workloads**:

   - Use Ballista’s Python bindings for **distributed data science tasks**, leveraging familiar tools to handle larger datasets.
   - **Example Scenarios**: Distributed aggregations, filtering, or preparing data for machine learning workflows.

#### Deployment Steps

1. **Installing the Python Package**:

   - Install the Ballista Python package:

   ```bash
   pip install ballista
   ```

2. **Connecting to Ballista Cluster**:

   - Set up a Ballista context to connect to a scheduler (similar to PySpark):

   ```python
   from ballista import BallistaBuilder

   # Connect to a scheduler running in the cluster
   ctx = BallistaBuilder().remote("df://scheduler-host:50050")

   # Register a Parquet file
   ctx.register_parquet("trips", "/path/to/parquet/file")

   # Execute a SQL query
   df = ctx.sql("SELECT COUNT(*) FROM trips")
   df.show()
   ``
   ```

#### Examples

- **Ad-hoc Query Example**:

  - Demonstrate using the Python API to run basic aggregations on a dataset.

  ```python
  df = ctx.sql("SELECT passenger_count, AVG(fare_amount) FROM trips GROUP BY passenger_count")
  df.show()
  ```

- **Data Science Workflow Example**:

  - **Placeholder**: Add an example of a more advanced data preparation workflow.

---

### Rust Library

Ballista can be used as a **rust library** to allow developers to embed Ballista directly into their applications or distributed compute platforms. This provides the flexibility to create custom schedulers, modify execution plans, and integrate Ballista as a core component of bespoke distributed systems.

#### Use Cases

1. **Embedding Ballista in Custom Compute Programs**:
   - Use Ballista as a **Rust library** to embed distributed compute capabilities within custom systems.
   - **Example Scenarios**: Build a custom compute platform or develop specialized schedulers.

#### Deployment Steps

1. **Adding Ballista as a Dependency**:

   - Add Ballista to your `Cargo.toml` file:

   ```toml
   [dependencies]
   ballista = "0.x"
   datafusion = "0.x"
   tokio = { version = "1", features = ["full"] }
   ```

2. **Extending Ballista Components**:

   - Customize schedulers and executors using Ballista’s extensible components.
   - See [Extending Components Guide](https://github.com/apache/datafusion-ballista/blob/main/docs/source/user-guide/extending-components.md).

#### Examples

- **Basic Embedding Example**:

  - Use Ballista as a library to execute a simple query.

  ```rust
  use ballista::prelude::*;
  use datafusion::prelude::*;

  #[tokio::main]
  async fn main() -> Result<()> {
      let ctx = BallistaContext::remote("df://localhost:50050").await?;
      let df = ctx.sql("SELECT COUNT(*) FROM trips").await?;
      df.show().await?;
      Ok(())
  }
  ```

- **Custom Scheduler Example**:

  - **Placeholder**: Include more advanced examples on how to customize schedulers or extend executors.
  - References:
    - [Custom Scheduler Example](https://github.com/apache/datafusion-ballista/blob/main/examples/examples/custom-scheduler.rs)
    - [Custom Executor Example](https://github.com/apache/datafusion-ballista/blob/main/examples/examples/custom-executor.rs)

- **Advanced Query Example**:

  The following example runs an advanced aggregation SQL query against a Parquet file (`yellow_tripdata_2022-01.parquet`) from the New York Taxi and Limousine Commission dataset:

  ```rust
  use ballista::prelude::*;
  use datafusion::common::Result;
  use datafusion::prelude::{col, SessionContext, ParquetReadOptions};
  use datafusion::functions_aggregate::{min_max::min, min_max::max, sum::sum, average::avg};

  #[tokio::main]
  async fn main() -> Result<()> {
      // Connect to Ballista scheduler
      let ctx = SessionContext::remote("df://localhost:50050").await?;

      let filename = "testdata/yellow_tripdata_2022-01.parquet";

      // Define the query using the DataFrame trait
      let df = ctx
          .read_parquet(filename, ParquetReadOptions::default())
          .await?
          .select_columns(&["passenger_count", "fare_amount"])?
          .aggregate(
              vec![col("passenger_count")],
              vec![
                  min(col("fare_amount")),
                  max(col("fare_amount")),
                  avg(col("fare_amount")),
                  sum(col("fare_amount")),
              ],
          )?
          .sort(vec![col("passenger_count").sort(true, true)])?;

      // Print the results
      df.show().await?;

      Ok(())
  }
  ```

This example demonstrates how to execute a distributed query involving aggregations and sorting. It shows Ballista's flexibility in processing real-world datasets.

---

## Configuration and Advanced Settings

### Common Configuration Scenarios

- **Scheduler Policies**:

  - Choose between `push-staged` or `pull-staged` strategies to determine how tasks are assigned to executors.

- **Executor Slots Policy**:

  - Configure policies such as `round-robin`, `bias`, or `round-robin-local` to manage how tasks are assigned.

- **Job Cleanup**:

  - Set up automatic **job cleanup** to manage completed job data and maintain efficient disk usage.

For more details, visit the [Configuration Guide](https://datafusion.apache.org/ballista/user-guide/configs.html).

---

## Community and Additional Resources

- [Ballista User Guide](https://datafusion.apache.org/ballista/user-guide/)
- [DataFusion Documentation](https://arrow.apache.org/datafusion/)
- [Python API Documentation](https://datafusion.apache.org/ballista/user-guide/python.html)
- [GitHub Repository](https://github.com/apache/datafusion-ballista)
- **Community Resources**: Join the **Apache Arrow** community on [Discord and GitHub Discussions](https://github.com/apache/arrow-rs?tab=readme-ov-file#arrow-rust-community).

---
### Deployment Options
Ballista can be deployed in various environments, offering flexibility and scalability to meet diverse data processing needs. Below are some common deployment options for Ballista:
- [Docker](https://datafusion.apache.org/ballista/user-guide/deployment/docker.html): Used to package executors and user-defined code, making deploying executors in different environments easy.
- [Docker Compose](https://datafusion.apache.org/ballista/user-guide/deployment/docker-compose.html): Allows you to define and run multi-container Docker applications, simplifying the deployment of Ballista clusters. Primarily used during testing.

- [Kubernetes](https://datafusion.apache.org/ballista/user-guide/deployment/kubernetes.html): Ballista can be deployed as a standalone or cluster in a Kubernetes environment. In a Kubernetes deployment, the scheduler can be configured to use etcd as a backing store, providing redundancy in case of a scheduler failure.

