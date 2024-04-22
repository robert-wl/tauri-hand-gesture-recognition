<script lang="ts">
  import type { Model } from "../../../../bindings";
  import { scale } from "svelte/transition";
  export let model: Model;
</script>

<div
  class="card w-fit min-h-80 h-fit bg-base-100 shadow-xl sticky top-0 border-primary border-t-2"
  in:scale|global={{ duration: 200, opacity: 0.5, start: 0.5 }}>
  <div class="card-title pb-0 py-4 flex flex-col items-center justify-between">
    <h2 class="text-xl font-bold">Confusion Matrix</h2>
    <hr class="my-2 w-full border-gray-200" />
  </div>
  <div class="card-body pt-2 flex flex-col h-full items-start justify-start">
    <img
      alt="Dataset Thumbnail"
      class="object-cover max-w-[40rem] rounded-sm transition-opacity"
      src={`data:image/png;base64,${model.confusion_matrix_image}`} />
  </div>
</div>
