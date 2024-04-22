<script lang="ts">
  import { scale } from "svelte/transition";
  import { Link } from "svelte-routing";
  import BackIcon from "../icons/BackIcon.svelte";
</script>

<div
  class="card w-80 min-h-40 bg-base-100 shadow-xl border-primary border-t-2"
  in:scale|global={{ duration: 200, opacity: 0.5, start: 0.5 }}>
  <div class="card-body items-center">
    <h2 class="card-title text-xl text-center pb-2">Dataset not found</h2>
    <p class="text-center">Please preprocess a dataset first.</p>

    <Link
      class="btn btn-accent min-h-0 h-10 rounded-sm mt-4"
      to="/">
      <BackIcon />
      <span>Go back</span>
    </Link>
  </div>
</div>
