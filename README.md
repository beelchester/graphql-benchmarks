# GraphQL Benchmarks <!-- omit from toc -->

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://codespaces.new/tailcallhq/graphql-benchmarks)

Explore and compare the performance of the fastest GraphQL frameworks through our comprehensive benchmarks.

- [Introduction](#introduction)
- [Quick Start](#quick-start)
- [Benchmark Results](#benchmark-results)
  - [Throughput (Higher is better)](#throughput-higher-is-better)
  - [Latency (Lower is better)](#latency-lower-is-better)
- [Architecture](#architecture)
  - [WRK](#wrk)
  - [GraphQL](#graphql)
  - [Nginx](#nginx)
  - [Jsonplaceholder](#jsonplaceholder)
- [GraphQL Schema](#graphql-schema)
- [Contribute](#contribute)

[Tailcall]: https://github.com/tailcallhq/tailcall
[Gqlgen]: https://github.com/99designs/gqlgen
[Apollo GraphQL]: https://github.com/apollographql/apollo-server
[Netflix DGS]: https://github.com/netflix/dgs-framework
[Caliban]: https://github.com/ghostdogpr/caliban
[async-graphql]: https://github.com/async-graphql/async-graphql
[Hasura]: https://github.com/hasura/graphql-engine
[GraphQL JIT]: https://github.com/zalando-incubator/graphql-jit

## Introduction

This document presents a comparative analysis of several renowned GraphQL frameworks. Dive deep into the performance metrics, and get insights into their throughput and latency.

> **NOTE:** This is a work in progress suite of benchmarks, and we would appreciate help from the community to add more frameworks or tune the existing ones for better performance.

## Quick Start

Get started with the benchmarks:

1. Click on this [link](https://codespaces.new/tailcallhq/graphql-benchmarks) to set up on GitHub Codespaces.
2. Once set up in Codespaces, initiate the benchmark tests:

```bash
./setup.sh
./run_benchmarks.sh
```

## Benchmark Results

<!-- PERFORMANCE_RESULTS_START -->

| Query | Server | Requests/sec | Latency (ms) | Relative |
|-------:|--------:|--------------:|--------------:|---------:|
| 1 | `{ posts { id userId title user { id name email }}}` |
|| [Tailcall] | `28,477.30` | `3.50` | `103.19x` |
|| [Hasura] | `5,387.00` | `19.43` | `19.52x` |
|| [async-graphql] | `2,017.14` | `50.11` | `7.31x` |
|| [Caliban] | `1,693.14` | `58.92` | `6.14x` |
|| [GraphQL JIT] | `1,367.62` | `72.82` | `4.96x` |
|| [Gqlgen] | `801.98` | `123.72` | `2.91x` |
|| [Netflix DGS] | `363.55` | `168.26` | `1.32x` |
|| [Apollo GraphQL] | `275.96` | `355.71` | `1.00x` |
| 2 | `{ posts { title }}` |
|| [Tailcall] | `58,706.80` | `1.70` | `41.76x` |
|| [async-graphql] | `10,187.80` | `9.96` | `7.25x` |
|| [Caliban] | `9,579.89` | `10.80` | `6.81x` |
|| [Hasura] | `6,115.97` | `16.36` | `4.35x` |
|| [Gqlgen] | `2,219.07` | `46.64` | `1.58x` |
|| [Apollo GraphQL] | `1,787.76` | `55.84` | `1.27x` |
|| [Netflix DGS] | `1,572.62` | `71.16` | `1.12x` |
|| [GraphQL JIT] | `1,405.82` | `71.02` | `1.00x` |
| 3 | `{ greet }` |
|| [Caliban] | `66,590.00` | `1.17` | `12.63x` |
|| [Tailcall] | `59,804.10` | `1.68` | `11.34x` |
|| [Gqlgen] | `47,331.80` | `4.98` | `8.97x` |
|| [async-graphql] | `46,932.40` | `2.16` | `8.90x` |
|| [Apollo GraphQL] | `8,168.92` | `12.49` | `1.55x` |
|| [Netflix DGS] | `8,101.34` | `15.25` | `1.54x` |
|| [Hasura] | `6,680.66` | `15.25` | `1.27x` |
|| [GraphQL JIT] | `5,273.76` | `18.94` | `1.00x` |

<!-- PERFORMANCE_RESULTS_END -->



### 1. `{posts {title body user {name}}}`
#### Throughput (Higher is better)

![Throughput Histogram](assets/req_sec_histogram1.png)

#### Latency (Lower is better)

![Latency Histogram](assets/latency_histogram1.png)

### 2. `{posts {title body}}`
#### Throughput (Higher is better)

![Throughput Histogram](assets/req_sec_histogram2.png)

#### Latency (Lower is better)

![Latency Histogram](assets/latency_histogram2.png)

### 3. `{greet}`
#### Throughput (Higher is better)

![Throughput Histogram](assets/req_sec_histogram3.png)

#### Latency (Lower is better)

![Latency Histogram](assets/latency_histogram3.png)

## Architecture

![Architecture Diagram](assets/architecture.png)

A client (`wrk`) sends requests to a GraphQL server to fetch post titles. The GraphQL server, in turn, retrieves data from an external source, `jsonplaceholder.typicode.com`, routed through the `nginx` reverse proxy.

### WRK

`wrk` serves as our test client, sending GraphQL requests at a high rate.

### GraphQL

Our tested GraphQL server. We evaluated various implementations, ensuring no caching on the GraphQL server side.

### Nginx

A reverse-proxy that caches every response, mitigating rate-limiting and reducing network uncertainties.

### Jsonplaceholder

The primary upstream service forming the base for our GraphQL API. We query its `/posts` API via the GraphQL server.

## GraphQL Schema

Inspect the generated GraphQL schema employed for the benchmarks:

```graphql
schema {
  query: Query
}

type Query {
  posts: [Post]
}

type Post {
  id: Int!
  userId: Int!
  title: String!
  body: String!
  user: User
}

type User {
  id: Int!
  name: String!
  username: String!
  email: String!
  phone: String
  website: String
}
```

## Contribute

Your insights are invaluable! Test these benchmarks, share feedback, or contribute by adding more GraphQL frameworks or refining existing ones. Open an issue or a pull request, and let's build a robust benchmarking resource together!
