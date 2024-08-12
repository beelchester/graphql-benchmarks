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
|| [Tailcall] | `29,885.80` | `3.33` | `137.61x` |
|| [async-graphql] | `1,987.89` | `50.33` | `9.15x` |
|| [Caliban] | `1,712.32` | `58.57` | `7.88x` |
|| [GraphQL JIT] | `1,328.30` | `74.94` | `6.12x` |
|| [Gqlgen] | `770.81` | `128.71` | `3.55x` |
|| [Netflix DGS] | `371.62` | `167.13` | `1.71x` |
|| [Apollo GraphQL] | `247.46` | `397.01` | `1.14x` |
|| [Hasura] | `217.18` | `462.87` | `1.00x` |
| 2 | `{ posts { title }}` |
|| [Tailcall] | `58,870.30` | `1.69` | `80.10x` |
|| [async-graphql] | `9,837.65` | `10.27` | `13.38x` |
|| [Caliban] | `9,649.39` | `10.71` | `13.13x` |
|| [Gqlgen] | `2,151.74` | `48.19` | `2.93x` |
|| [Apollo GraphQL] | `1,634.93` | `61.09` | `2.22x` |
|| [Netflix DGS] | `1,623.12` | `69.26` | `2.21x` |
|| [GraphQL JIT] | `1,378.26` | `72.45` | `1.88x` |
|| [Hasura] | `735.00` | `136.01` | `1.00x` |
| 3 | `{ greet }` |
|| [Caliban] | `67,658.00` | `1.11` | `29.17x` |
|| [Tailcall] | `59,695.50` | `1.69` | `25.73x` |
|| [async-graphql] | `47,414.30` | `2.28` | `20.44x` |
|| [Gqlgen] | `47,187.30` | `4.81` | `20.34x` |
|| [Netflix DGS] | `8,359.72` | `14.64` | `3.60x` |
|| [Apollo GraphQL] | `7,710.36` | `13.29` | `3.32x` |
|| [GraphQL JIT] | `5,129.89` | `19.47` | `2.21x` |
|| [Hasura] | `2,319.82` | `43.33` | `1.00x` |

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
