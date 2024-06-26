<script lang="ts">
  import { fade } from "svelte/transition";
  import type { KNNHyperparameter } from "../../../../bindings";

  export let hyperparameters: KNNHyperparameter;
</script>

<div
  class="card-body pt-2 flex flex-col h-full items-start justify-start"
  in:fade|global={{ duration: 200 }}>
  <div class="flex flex-col w-full items-center flex-1">
    <div class="form-control w-4/5 max-w-xs">
      <div class="label">
        <span class="label-text">Select Metric</span>
      </div>
      <select
        bind:value={hyperparameters.metric}
        class="select bg-gray-50 hover:border-gray-300 border-gray-300 focus:outline-none focus:scale-100">
        <option
          selected
          value="minkowski">Minkowski</option>
        <option value="euclidean">Euclidean</option>
        <option value="manhattan">Manhattan</option>
        <option value="chebyshev">Chebyshev</option>
      </select>
    </div>
    <div class="form-control w-4/5 max-w-xs">
      <div class="label">
        <span class="label-text">Select Algorithm</span>
      </div>
      <select
        bind:value={hyperparameters.algorithm}
        class="select bg-gray-50 hover:border-gray-300 border-gray-300 focus:outline-none focus:scale-100">
        <option
          selected
          value="auto">Auto</option>
        <option value="ball_tree">Ball Tree</option>
        <option value="kd_tree">KD Tree</option>
        <option value="brute">Brute</option>
      </select>
    </div>
    <div class="form-control w-4/5 max-w-xs">
      <div class="label">
        <span class="label-text">N Neighbors</span>
      </div>
      <input
        bind:value={hyperparameters.n_neighbors}
        class="input bg-gray-50 hover:border-gray-300 border-gray-300 focus:outline-none focus:scale-100"
        placeholder="Positive float"
        type="text" />
    </div>
    <div class="form-control w-4/5 max-w-xs">
      <div class="label">
        <span class="label-text">Select Weights</span>
      </div>
      <select
        bind:value={hyperparameters.weights}
        class="select bg-gray-50 hover:border-gray-300 border-gray-300 focus:outline-none focus:scale-100">
        <option
          selected
          value="uniform">Uniform</option>
        <option value="distance">Distance</option>
      </select>
    </div>
  </div>
</div>
