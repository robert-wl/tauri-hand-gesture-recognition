<script lang="ts">
  import { fade } from "svelte/transition";
  import type { LRHyperparameter } from "../../../../bindings";

  export let hyperparameters: LRHyperparameter;
</script>

<div
  class="card-body pt-2 flex flex-col h-full items-start justify-start"
  in:fade|global={{ duration: 200 }}>
  <div class="flex flex-col w-full items-center flex-1">
    <div class="form-control w-4/5 max-w-xs">
      <div class="label">
        <span class="label-text">Select Penalty</span>
      </div>
      <select
        bind:value={hyperparameters.penalty}
        class="select bg-gray-50 hover:border-gray-300 border-gray-300 focus:outline-none focus:scale-100">
        <option
          selected
          value="l2">L2</option>
        <option value="l1">L1</option>
        <option value="elasticnet">Elasticnet</option>
        <option value="none">None</option>
      </select>
    </div>
    <div class="form-control w-4/5 max-w-xs">
      <div class="label">
        <span class="label-text">C Value</span>
      </div>
      <input
        bind:value={hyperparameters.C}
        class="input bg-gray-50 hover:border-gray-300 border-gray-300 focus:outline-none focus:scale-100"
        placeholder="Positive float"
        type="text" />
    </div>
    <div class="form-control w-4/5 max-w-xs">
      <div class="label">
        <span class="label-text">Select Penalty</span>
      </div>
      <select
        bind:value={hyperparameters.solver}
        class="select bg-gray-50 hover:border-gray-300 border-gray-300 focus:outline-none focus:scale-100">
        <option
          selected
          value="lbfgs">L-BFGS</option>
        <option value="liblinear">Liblinear</option>
        <option value="newton-cg">Newton-CG</option>
        <option value="newton-cholesky">Newton-Cholesky</option>
        <option value="sag">SAG</option>
        <option value="saga">SAGA</option>
      </select>
    </div>
    <div class="form-control w-4/5 max-w-xs">
      <div class="label">
        <span class="label-text">Max Iterations</span>
      </div>
      <input
        bind:value={hyperparameters.max_iter}
        class="input bg-gray-50 hover:border-gray-300 border-gray-300 focus:outline-none focus:scale-100"
        placeholder="Positive integer"
        type="text" />
    </div>
  </div>
</div>
