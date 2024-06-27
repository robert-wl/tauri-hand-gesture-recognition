<script>
  import { scale } from "svelte/transition";
</script>

<div
  class="card w-1/2 max-w-96 items-center justify-center h-fit bg-base-100 shadow-xl border-primary border-t-2"
  in:scale|global={{ duration: 200, opacity: 0.5, start: 0.5 }}>
  <div class="w-64 h-64">
    <img
      alt="Loading"
      class="object-cover"
      src="/loading.gif" />
  </div>

  <hr class="my-2 w-full border-gray-200" />
  <p class="text-center text-lg font-bold py-4 pb-6">Loading...</p>
</div>
