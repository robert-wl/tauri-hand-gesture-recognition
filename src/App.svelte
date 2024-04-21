<script lang="ts">
  import { Route, Router } from "svelte-routing";
  import Home from "./routes/Home.svelte";
  import PreprocessDataset from "./routes/preprocess/PreprocessDataset.svelte";
  import Preprocess from "./routes/preprocess/Preprocess.svelte";
  import Training from "./routes/training/Training.svelte";
  import TrainingDataset from "./routes/training/TrainingDataset.svelte";
</script>

<Router>
  <Route path="/">
    <Home />
  </Route>
  <Route path="/preprocess-dataset">
    <PreprocessDataset />
  </Route>
  <Route path="/training-dataset">
    <TrainingDataset />
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
