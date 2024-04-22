<script lang="ts">
  import type { Model } from "../../../../bindings";
  import { Link } from "svelte-routing";
  import { scale } from "svelte/transition";

  export let model: Model;
</script>

<div
  class="card w-64 min-h-40 bg-base-100 shadow-xl border-primary border-t-2"
  in:scale|global={{ duration: 200, opacity: 0.5, start: 0.5 }}>
  <div class="card-body items-center">
    <h2 class="card-title text-xl text-center pb-2">{model.name}</h2>
    <p class="text-center">Trained Using {model.model_specification.dataset_name}</p>
    <p class="text-center">Accuracy: {(model.model_specification.accuracy * 100).toFixed(2)}%</p>
    <Link
      class="btn btn-primary btn-sm min-h-0 h-fit font-bold text-white py-2.5 mt-2 w-full"
      to={`/testing-dataset/${model.name}`}>
      Select Model
    </Link>
  </div>
</div>
