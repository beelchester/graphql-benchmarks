import caliban.*
import caliban.execution.QueryExecution
import caliban.quick.*
import zio.*

object Main extends ZIOAppDefault {
  override val bootstrap: ZLayer[ZIOAppArgs, Any, Any] =
    Runtime.removeDefaultLoggers ++ Runtime.disableFlags(RuntimeFlag.FiberRoots)

  private val api = graphQL(RootResolver(Query(Service.posts)))
  // print the schema to the console
  println(api.render)
  println("http://localhost:8000/graphql")
  def run =
    api
      .runServer(8000, apiPath = "/graphql")
      .provide(Service.layer, Client.live, ZLayer.scoped(Configurator.setQueryExecution(QueryExecution.Batched)))
}
