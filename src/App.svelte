<script lang="ts">
  import { Route, Router } from "svelte-routing";
  import Home from "./routes/Home.svelte";
  import Preprocess from "./routes/Preprocess.svelte";
</script>

<Router>
  <!--      <Route component={BlogPost} path="/blog/:id" />-->
  <!--      <Route component={Blog} path="/blog" />-->
  <!--      <Route component={About} path="/about" />-->
  <Route path="/preprocess">
    <Preprocess />
  </Route>
  <Route path="/">
    <Home />
  </Route>
</Router>