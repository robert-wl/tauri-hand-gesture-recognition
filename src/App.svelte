<script lang="ts">
  import { Route, Router } from "svelte-routing";
  import Home from "./routes/Home.svelte";
  import Dataset from "./routes/preprocess/DatasetList.svelte";
  import Preprocess from "./routes/preprocess/Preprocess.svelte";
</script>

<Router>
  <!--      <Route component={BlogPost} path="/blog/:id" />-->
  <!--      <Route component={Blog} path="/blog" />-->
  <!--      <Route component={About} path="/about" />-->
  <Route path="/dataset">
    <Dataset />
  </Route>
  <Route
    path="/preprocess/:name"
    let:params>
    <Preprocess name={params.name} />
  </Route>
  <Route path="/">
    <Home />
  </Route>
</Router>
