<script lang="ts">
  import { scale } from "svelte/transition";

  export let title: string;
  export let imageUrl: string;
</script>

<div
  class="card w-[35rem] min-h-40 bg-base-100 shadow-xl border-primary border-t-2"
  in:scale|global={{ duration: 200, opacity: 0.5, start: 0.5 }}>
  <div class="card-title flex flex-col justify-center items-center w-full py-2 gap-0">
    <h2 class="text-xl font-bold">{title}</h2>
    <hr class="my-2 w-full border-gray-200" />
  </div>
  <div class="card-body items-center">
    <div class="flex items-center justify-center w-full">
      <div class="flex flex-col items-center justify-center w-full h-64 border-2 rounded-bg bg-gray-50 hover:bg-gray-100">
        <img
          alt="preview"
          class="w-full h-full rounded-md object-cover"
          src={imageUrl} />
      </div>
    </div>
  </div>
</div>
