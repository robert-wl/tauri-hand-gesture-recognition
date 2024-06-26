<script lang="ts">
  import type { Model } from "../../../../bindings";
  import { scale } from "svelte/transition";
  export let model: Model;
</script>

<div
  class="card w-fit min-h-80 bg-base-100 shadow-xl sticky top-0 border-primary border-t-2"
  in:scale|global={{ duration: 200, opacity: 0.5, start: 0.5 }}>
  <div class="card-title pb-0 py-4 flex flex-col items-center justify-between">
    <h2 class="text-xl font-bold">Classification Report</h2>
    <hr class="my-2 w-full border-gray-200" />
  </div>
  <div class="card-body pt-2 flex flex-col h-full items-start justify-start">
    <table class="table">
      <thead>
        <tr>
          <th>Class</th>
          <th>Precision</th>
          <th>Recall</th>
          <th>F1-Score</th>
          <th>Support</th>
        </tr>
      </thead>
      <tbody>
        {#each Object.entries(model.model_specification.classification_report) as [reportKey, reportVal]}
          {#if reportKey === "class"}
            {#each Object.entries(reportVal).sort(([a, _], [b, __]) => a.localeCompare(b)) as [key, value]}
              <tr>
                <td>{key}</td>
                <td>{value.precision.toFixed(4)}</td>
                <td>{value.recall.toFixed(4)}</td>
                <td>{value["f1-score"].toFixed(4)}</td>
                <td>{value.support}</td>
              </tr>
            {/each}
          {:else if reportKey === "accuracy"}
            <tr>
              <td>{reportKey}</td>
              <td></td>
              <td></td>
              <td>{(+reportVal).toFixed(4)}</td>
            </tr>
          {:else if typeof reportVal !== "number"}
            <tr>
              <td>{reportKey}</td>
              <td>{(+reportVal.precision).toFixed(4)}</td>
              <td>{(+reportVal.recall).toFixed(4)}</td>
              <td>{(+reportVal["f1-score"]).toFixed(4)}</td>
              <td>{reportVal.support}</td>
            </tr>
          {/if}
        {/each}
      </tbody>
    </table>
  </div>
</div>
