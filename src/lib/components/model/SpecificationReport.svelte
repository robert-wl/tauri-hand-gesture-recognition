<script lang="ts">
  import type { Model } from "../../../../bindings";
  import { scale } from "svelte/transition";
  import { Link } from "svelte-routing";

  export let model: Model;
  export let saveModel: Nullable<() => void> = null;
  export let retrainModel: Nullable<() => void> = null;
  export let isPreview: boolean = false;
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
        {#if "Svm" in model.model_specification.hyperparameters}
          <tr>
            <td>Kernel</td>
            <td>{model.model_specification.hyperparameters.Svm.kernel}</td>
          </tr>
          <tr>
            <td>C</td>
            <td>{model.model_specification.hyperparameters.Svm.C}</td>
          </tr>
          <tr>
            <td>Gamma</td>
            <td>{model.model_specification.hyperparameters.Svm.gamma}</td>
          </tr>
          {#if model.model_specification.hyperparameters.Svm.kernel === "poly"}
            <tr>
              <td>Degree</td>
              <td>{model.model_specification.hyperparameters.Svm.degree}</td>
            </tr>
          {/if}
        {:else if "Knn" in model.model_specification.hyperparameters}
          <tr>
            <td>Neighbors</td>
            <td>{model.model_specification.hyperparameters.Knn.n_neighbors}</td>
          </tr>
          <tr>
            <td>Weights</td>
            <td>{model.model_specification.hyperparameters.Knn.weights}</td>
          </tr>
          <tr>
            <td>Algorithm</td>
            <td>{model.model_specification.hyperparameters.Knn.algorithm}</td>
          </tr>
          <tr>
            <td>Metric</td>
            <td>{model.model_specification.hyperparameters.Knn.metric}</td>
          </tr>
        {:else if "Lr" in model.model_specification.hyperparameters}
          <tr>
            <td>Penalty</td>
            <td>{model.model_specification.hyperparameters.Lr.penalty}</td>
          </tr>
          <tr>
            <td>C</td>
            <td>{model.model_specification.hyperparameters.Lr.C}</td>
          </tr>
          <tr>
            <td>Solver</td>
            <td>{model.model_specification.hyperparameters.Lr.solver}</td>
          </tr>
          <tr>
            <td>Max Iterations</td>
            <td>{model.model_specification.hyperparameters.Lr.max_iter}</td>
          </tr>
        {/if}
        <tr>
          <td>Accuracy</td>
          <td>{model.model_specification.accuracy.toFixed(4)}</td>
        </tr>
        <tr>
          <td>Precision</td>
          <td>{model.model_specification.precision.toFixed(4)}</td>
        </tr>
        <tr>
          <td>Recall</td>
          <td>{model.model_specification.recall.toFixed(4)}</td>
        </tr>
        <tr>
          <td>F1 Score</td>
          <td>{model.model_specification.f1.toFixed(4)}</td>
        </tr>
      </tbody>
    </table>
    {#if saveModel}
      <button
        class="btn btn-primary btn-sm text-white min-h-0 h-fit font-bold py-2.5 mt-2 w-full"
        on:click={saveModel}>Save Model</button>
    {/if}
    {#if retrainModel}
      <button
        class="btn btn-error btn-sm text-white min-h-0 h-fit font-bold py-2.5 mt-2 w-full"
        on:click={retrainModel}>Retrain Model</button>
    {/if}
    {#if isPreview}
      <Link
        class="btn btn-primary btn-sm min-h-0 h-fit font-bold text-white py-2.5 mt-2 w-full"
        to={`/testing-dataset/${model.name}`}>
        Select Model
      </Link>
    {/if}
  </div>
</div>
