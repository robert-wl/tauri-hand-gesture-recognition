<script lang="ts">
  import { scale, fade } from "svelte/transition";
  import type { SVMHyperparameter } from "../../../../bindings";

  export let hyperparameters: SVMHyperparameter;
</script>

<div
  class="card-body pt-2 flex flex-col h-full items-start justify-start"
  in:fade|global={{ duration: 200 }}>
  <div class="flex flex-col w-full items-center flex-1">
    <div class="form-control w-4/5 max-w-xs">
      <div class="label">
        <span class="label-text">Select kernel</span>
      </div>
      <select
        bind:value={hyperparameters.kernel}
        class="select bg-gray-50 hover:border-gray-300 border-gray-300 focus:outline-none focus:scale-100">
        <option
          selected
          value="rbf">RBF</option>
        <option value="linear">Linear</option>
        <option value="poly">Polynomial</option>
        <option value="sigmoid">Sigmoid</option>
      </select>
    </div>
    <div class="form-control w-4/5 max-w-xs">
      <div class="label">
        <span class="label-text">C</span>
      </div>
      <input
        bind:value={hyperparameters.C}
        class="input bg-gray-50 hover:border-gray-300 border-gray-300 focus:outline-none focus:scale-100"
        placeholder="Positive float"
        type="text" />
    </div>
    <div class="form-control w-4/5 max-w-xs">
      <div class="label">
        <span class="label-text">Gamma</span>
      </div>
      <input
        bind:value={hyperparameters.gamma}
        class="input bg-gray-50 hover:border-gray-300 border-gray-300 focus:outline-none focus:scale-100"
        placeholder="Auto, scale or a positive float"
        type="text" />
    </div>
    {#if hyperparameters.kernel === "poly"}
      <div
        in:scale={{ duration: 200, opacity: 0.5, start: 0.5 }}
        out:fade={{ duration: 200 }}
        class="form-control w-4/5 max-w-xs">
        <div class="label">
          <span class="label-text">Degree</span>
        </div>
        <input
          bind:value={hyperparameters.degree}
          class="input bg-gray-50 hover:border-gray-300 border-gray-300 focus:outline-none focus:scale-100"
          placeholder="Positive integer"
          type="text" />
      </div>
    {/if}
  </div>
</div>
