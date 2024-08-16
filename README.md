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
|| [Tailcall] | `28,603.90` | `3.49` | `123.75x` |
|| [async-graphql] | `1,937.30` | `51.81` | `8.38x` |
|| [Caliban] | `1,767.34` | `56.32` | `7.65x` |
|| [GraphQL JIT] | `1,334.01` | `74.64` | `5.77x` |
|| [Gqlgen] | `783.53` | `126.63` | `3.39x` |
|| [Netflix DGS] | `365.56` | `217.40` | `1.58x` |
|| [Apollo GraphQL] | `274.21` | `358.27` | `1.19x` |
|| [Hasura] | `231.14` | `427.10` | `1.00x` |
| 2 | `{ posts { title }}` |
|| [Tailcall] | `58,250.00` | `1.71` | `83.01x` |
|| [async-graphql] | `10,239.30` | `9.86` | `14.59x` |
|| [Caliban] | `9,845.46` | `10.49` | `14.03x` |
|| [Gqlgen] | `2,197.20` | `47.08` | `3.13x` |
|| [Apollo GraphQL] | `1,800.21` | `55.47` | `2.57x` |
|| [Netflix DGS] | `1,598.99` | `70.07` | `2.28x` |
|| [GraphQL JIT] | `1,346.84` | `74.14` | `1.92x` |
|| [Hasura] | `701.74` | `142.08` | `1.00x` |
| 3 | `{ greet }` |
|| [Caliban] | `66,984.30` | `1.09` | `26.99x` |
|| [Tailcall] | `59,165.70` | `1.71` | `23.84x` |
|| [Gqlgen] | `48,126.10` | `5.27` | `19.39x` |
|| [async-graphql] | `47,435.10` | `2.18` | `19.11x` |
|| [Netflix DGS] | `8,263.04` | `14.73` | `3.33x` |
|| [Apollo GraphQL] | `8,216.94` | `12.39` | `3.31x` |
|| [GraphQL JIT] | `5,238.70` | `19.06` | `2.11x` |
|| [Hasura] | `2,482.13` | `40.48` | `1.00x` |

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
