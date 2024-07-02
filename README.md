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

[Tailcall]: https://tailcall.run/
[Gqlgen]: https://gqlgen.com/
[Apollo GraphQL]: https://new.apollographql.com/
[Netflix DGS]: https://netflix.github.io/dgs/
[Caliban]: https://ghostdogpr.github.io/caliban/
[async-graphql]: https://github.com/async-graphql/async-graphql

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

### Test Query
```graphql
{
  posts {
    title
  }
}
```

<!-- PERFORMANCE_RESULTS_START_1 -->

| Server | Requests/sec | Latency (ms) |
|--------:|--------------:|--------------:|
| [Tailcall] | `62,745.40` | `1.59` |
| [Caliban] | `9,345.30` | `11.06` |
| [async-graphql] | `7,574.73` | `13.21` |
| [Gqlgen] | `2,210.34` | `46.79` |
| [Apollo GraphQL] | `1,832.87` | `54.34` |
| [Netflix DGS] | `1,620.15` | `65.73` |

<!-- PERFORMANCE_RESULTS_END_1 -->

### Throughput (Higher is better)

![Throughput Histogram](assets/req_sec_histogram1.png)

### Latency (Lower is better)

![Latency Histogram](assets/latency_histogram1.png)

---

### Test Query
```graphql
{
  posts {
    id
    userId
    title
    user {
      id
      name
      email
    }
  }
}
```

<!-- PERFORMANCE_RESULTS_START_2 -->

| Server | Requests/sec | Latency (ms) |
|--------:|--------------:|--------------:|
| [Tailcall] | `30,574.00` | `3.26` |
| [Caliban] | `1,609.47` | `62.02` |
| [async-graphql] | `1,553.31` | `64.28` |
| [Gqlgen] | `662.59` | `151.58` |
| [Netflix DGS] | `362.76` | `156.19` |
| [Apollo GraphQL] | `293.87` | `338.20` |

<!-- PERFORMANCE_RESULTS_END_2 -->

### Throughput (Higher is better)

![Throughput Histogram](assets/req_sec_histogram2.png)

### Latency (Lower is better)

![Latency Histogram](assets/latency_histogram2.png)

---

### Test Query
```graphql
{
  greet
}
```

<!-- PERFORMANCE_RESULTS_START_3 -->

| Server | Requests/sec | Latency (ms) |
|--------:|--------------:|--------------:|
| [Tailcall] | `56,534.50` | `1.81` |
| [Caliban] | `49,900.30` | `2.16` |
| [async-graphql] | `43,681.60` | `2.34` |
| [Gqlgen] | `43,120.20` | `4.82` |
| [Apollo GraphQL] | `8,648.76` | `12.00` |
| [Netflix DGS] | `7,230.38` | `16.64` |

<!-- PERFORMANCE_RESULTS_END_3 -->

### Throughput (Higher is better)

![Throughput Histogram](assets/req_sec_histogram3.png)

### Latency (Lower is better)

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
