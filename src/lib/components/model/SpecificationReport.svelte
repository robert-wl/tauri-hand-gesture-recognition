<script lang="ts">
  import type { Model } from "../../../../bindings";
  import { scale } from "svelte/transition";

  export let model: Model;
  export let saveModel: () => void;
  export let retrainModel: () => void;
</script>

<div
  class="card w-64 min-h-80 h-fit bg-base-100 shadow-xl sticky top-0 border-primary border-t-2"
  in:scale|global={{ duration: 200, opacity: 0.5, start: 0.5 }}>
  <div class="card-title pb-0 py-4 flex flex-col items-center justify-between">
    <h2 class="text-xl font-bold">{model.name}</h2>
    <hr class="my-2 w-full border-gray-200" />
  </div>
  <div class="card-body pt-2 flex flex-col h-full items-start justify-start">
    <table class="table">
      <thead>
        <tr>
          <th>Specifications</th>
          <th>Value</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td>Kernel</td>
          <td>{model.model_specification.hyperparameters.kernel}</td>
        </tr>
        <tr>
          <td>C</td>
          <td>{model.model_specification.hyperparameters.C}</td>
        </tr>
        <tr>
          <td>Gamma</td>
          <td>{model.model_specification.hyperparameters.gamma}</td>
        </tr>
        {#if model.model_specification.hyperparameters.kernel === "poly"}
          <tr>
            <td>Degree</td>
            <td>{model.model_specification.hyperparameters.degree}</td>
          </tr>
        {/if}
        <tr>
          <td>Accuracy</td>
          <td>{model.model_specification.accuracy.toFixed(2)}</td>
        </tr>
        <tr>
          <td>Precision</td>
          <td>{model.model_specification.precision.toFixed(2)}</td>
        </tr>
        <tr>
          <td>Recall</td>
          <td>{model.model_specification.recall.toFixed(2)}</td>
        </tr>
        <tr>
          <td>F1 Score</td>
          <td>{model.model_specification.f1.toFixed(2)}</td>
        </tr>
      </tbody>
    </table>
    <button
      class="btn btn-primary btn-sm text-white min-h-0 h-fit font-bold py-2.5 mt-2 w-full"
      on:click={saveModel}>Save Model</button>
    <button
      class="btn btn-error btn-sm text-white min-h-0 h-fit font-bold py-2.5 mt-2 w-full"
      on:click={retrainModel}>Retrain Model</button>
  </div>
</div>
