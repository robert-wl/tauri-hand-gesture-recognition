<script lang="ts">
  import { scale, fade } from "svelte/transition";

  export let image: string;
</script>

<div
  class="card w-full min-h-40 h-fit bg-base-100 shadow-xl border-primary border-t-2"
  in:scale|global={{ duration: 200, opacity: 0.5, start: 0.5 }}>
  <div class="card-title flex flex-col gap-0 relative group p-2">
    <img
      alt="Dataset Thumbnail"
      class="object-cover w-full rounded-sm transition-opacity rounded-t-none border-b border-gray-200"
      in:fade={{ duration: 500 }}
      out:fade={{ duration: 0 }}
      src={`data:image/jpeg;base64,${image}`} />
  </div>
</div>
