<script lang="ts">
  import { Route, Router } from "svelte-routing";
  import Home from "./routes/Home.svelte";
  import Dataset from "./routes/preprocess/DatasetList.svelte";
  import Preprocess from "./routes/preprocess/Preprocess.svelte";
  import { cubicIn } from "svelte/easing";
  import { blur, draw, fade, fly, scale, slide } from "svelte/transition";
  import Training from "./routes/training/Training.svelte";
</script>

<Router>
  <!--      <Route component={BlogPost} path="/blog/:id" />-->
  <!--      <Route component={Blog} path="/blog" />-->
  <!--      <Route component={About} path="/about" />-->
  <Route path="/">
    <Home />
  </Route>
  <Route path="/dataset">
    <Dataset />
  </Route>
  <Route
    let:params
    path="/preprocess/:name">
    <Preprocess name={params.name} />
  </Route>
  <Route path="/training">
    <Training />
  </Route>
</Router>
