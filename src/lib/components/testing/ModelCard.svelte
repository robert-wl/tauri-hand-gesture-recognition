<script lang="ts">
  import type { Model } from "../../../../bindings";
  import { Link } from "svelte-routing";
  import { scale } from "svelte/transition";

  export let model: Model;
  export let selectModel: (name: string) => void;
</script>

<div
  class="card w-64 min-h-40 bg-base-100 shadow-xl border-primary border-t-2"
  in:scale|global={{ duration: 200, opacity: 0.5, start: 0.5 }}>
  <div class="card-body items-center pb-2">
    <h2 class="card-title text-xl text-center pb-2">{model.name}</h2>
    <p class="text-center font-semibold">{model.model_specification.algorithm}</p>
    <p class="text-center">Trained Using {model.model_specification.dataset_name}</p>
    <p class="text-center">Accuracy: {(model.model_specification.accuracy * 100).toFixed(2)}%</p>
  </div>
  <hr class="my-2 w-full border-gray-200" />
  <div class="card-body items-center pt-2">
    <Link
      class="btn btn-primary btn-sm min-h-0 h-fit font-bold text-white py-2.5 mt-2 w-full"
      to={`/testing-dataset/${model.name}`}>
      Select Model
    </Link>
    <button
      class="btn bg-red-500 hover:bg-red-600 btn-sm min-h-0 h-fit font-bold text-white py-2.5 mt-2 w-full"
      on:click={() => selectModel(model.name)}>
      Preview Model
    </button>
  </div>
</div>
